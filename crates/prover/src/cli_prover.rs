//! The circuit prover that runs nargo and bb on the compiled circuits. bb writes each file as 32-byte big-endian fields.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use anoma_arm::Fr;
use anoma_arm::field::{fields_to_bytes, from_bytes};

use crate::circuit_prover::{
    CircuitError, CircuitInputs, CircuitProver, Proof, Target, VerificationKey,
};

/// The nargo and bb binaries, the compiled circuit workspace, and a directory for the files of the runs.
#[derive(Clone, Debug)]
pub struct CliProver {
    pub nargo: PathBuf,
    pub bb: PathBuf,
    pub circuits: PathBuf,
    pub work: PathBuf,
}

/// Why a run of nargo or bb fails, other than a failed check of the inputs or of a proof.
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("{command} failed: {output}")]
    Tool {
        command: &'static str,
        output: String,
    },
    #[error("cannot run {program}: {source}")]
    Spawn {
        program: PathBuf,
        source: std::io::Error,
    },
    #[error("cannot read or write {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("{path} is not a bb file of the expected shape: {reason}")]
    Format { path: PathBuf, reason: String },
    #[error("cannot write the inputs: {0}")]
    Inputs(#[from] toml::ser::Error),
}

/// Makes the names of the files of each run unique in the process.
static RUN: AtomicU64 = AtomicU64::new(0);

impl CircuitProver for CliProver {
    /// Writes the verification key of a circuit on the first call, and reads it.
    fn verification_key(&self, circuit: &str) -> Result<VerificationKey, CircuitError> {
        let failed = |source| failed(circuit, source);
        let directory = self.key_directory(circuit).map_err(failed)?;
        let hash_file = directory.join("vk_hash");
        let hash = read_fields(&hash_file).map_err(failed)?;
        let [hash] = hash[..] else {
            return Err(failed(format_error(
                &hash_file,
                format!("{} fields, expected 1", hash.len()),
            )));
        };
        Ok(VerificationKey {
            fields: read_fields(&directory.join("vk")).map_err(failed)?,
            hash,
        })
    }

    /// Solves the circuit with nargo, proves it with bb, and checks the proof. A failed run keeps its files.
    fn prove(
        &self,
        circuit: &str,
        inputs: &CircuitInputs,
        target: Target,
    ) -> Result<Proof, CircuitError> {
        let failed = |source| failed(circuit, source);
        let run = run_name(circuit);
        let directory = self.work.join("proofs").join(&run);
        create_dir(&directory).map_err(failed)?;

        let prover_file = directory.join("inputs");
        let text = toml::to_string(inputs)
            .map_err(CliError::from)
            .map_err(failed)?;
        write(&prover_file.with_extension("toml"), text.as_bytes()).map_err(failed)?;
        let executed = check(
            Command::new(&self.nargo)
                .arg("execute")
                .arg("--program-dir")
                .arg(&self.circuits)
                .args(["--package", circuit, "--silence-warnings", "--prover-name"])
                .arg(&prover_file)
                .arg(&run),
        )
        .map_err(failed)?;
        if let Err(reason) = executed {
            return Err(CircuitError::Unsatisfied {
                circuit: circuit.to_string(),
                reason,
            });
        }
        let solved = self.circuits.join("target").join(format!("{run}.gz"));
        let witness = directory.join("witness.gz");
        std::fs::rename(&solved, &witness)
            .map_err(|source| CliError::Io {
                path: solved,
                source,
            })
            .map_err(failed)?;

        let key_directory = self.key_directory(circuit).map_err(failed)?;
        run_tool(
            "bb prove",
            Command::new(&self.bb)
                .args(["prove", "--verify", "-t", target_name(target), "-b"])
                .arg(self.artifact(circuit))
                .arg("-w")
                .arg(&witness)
                .arg("-k")
                .arg(key_directory.join("vk"))
                .arg("-o")
                .arg(&directory),
        )
        .map_err(failed)?;
        let proof = Proof {
            fields: read_fields(&directory.join("proof")).map_err(failed)?,
            public_inputs: read_fields(&directory.join("public_inputs")).map_err(failed)?,
        };
        remove_dir(&directory).map_err(failed)?;
        Ok(proof)
    }

    fn verify(
        &self,
        circuit: &str,
        key: &VerificationKey,
        proof: &[Fr],
        public_inputs: &[Fr],
        target: Target,
    ) -> Result<(), CircuitError> {
        let failed = |source| failed(circuit, source);
        let directory = self.work.join("verify").join(run_name(circuit));
        create_dir(&directory).map_err(failed)?;
        let [key_file, proof_file, public_inputs_file] =
            ["vk", "proof", "public_inputs"].map(|name| directory.join(name));
        write(&key_file, &fields_to_bytes(&key.fields)).map_err(failed)?;
        write(&proof_file, &fields_to_bytes(proof)).map_err(failed)?;
        write(&public_inputs_file, &fields_to_bytes(public_inputs)).map_err(failed)?;
        let verified = check(
            Command::new(&self.bb)
                .args(["verify", "-t", target_name(target), "-k"])
                .arg(&key_file)
                .arg("-p")
                .arg(&proof_file)
                .arg("-i")
                .arg(&public_inputs_file),
        );
        remove_dir(&directory).map_err(failed)?;
        verified
            .map_err(failed)?
            .map_err(|reason| CircuitError::Rejected {
                circuit: circuit.to_string(),
                reason,
            })
    }
}

impl CliProver {
    /// Takes nargo and bb from the `NARGO` and `BB` variables, or else from the `PATH`.
    pub fn from_env(circuits: PathBuf, work: PathBuf) -> Self {
        let tool = |variable, default| {
            std::env::var_os(variable).map_or_else(|| PathBuf::from(default), PathBuf::from)
        };
        Self {
            nargo: tool("NARGO", "nargo"),
            bb: tool("BB", "bb"),
            circuits,
            work,
        }
    }

    /// The directory with the `vk` and `vk_hash` files of a circuit, which this writes on the first call.
    fn key_directory(&self, circuit: &str) -> Result<PathBuf, CliError> {
        let directory = self.work.join("vk").join(circuit);
        if !directory.join("vk_hash").exists() {
            create_dir(&directory)?;
            // The key of a circuit is the same for both targets.
            run_tool(
                "bb write_vk",
                Command::new(&self.bb)
                    .args(["write_vk", "-t", target_name(Target::Step), "-b"])
                    .arg(self.artifact(circuit))
                    .arg("-o")
                    .arg(&directory),
            )?;
        }
        Ok(directory)
    }

    fn artifact(&self, circuit: &str) -> PathBuf {
        self.circuits.join("target").join(format!("{circuit}.json"))
    }
}

fn failed(circuit: &str, source: CliError) -> CircuitError {
    CircuitError::Failed {
        circuit: circuit.to_string(),
        source: Box::new(source),
    }
}

/// The bb target of a proof.
fn target_name(target: Target) -> &'static str {
    match target {
        Target::Leaf => "noir-recursive",
        Target::Step => "noir-recursive-no-zk",
    }
}

/// A name for the files of one run of a circuit, unique in the process.
fn run_name(circuit: &str) -> String {
    format!(
        "{circuit}-{}-{}",
        std::process::id(),
        RUN.fetch_add(1, Ordering::Relaxed)
    )
}

/// Runs a tool whose failure is an error.
fn run_tool(command: &'static str, process: &mut Command) -> Result<(), CliError> {
    check(process)?.map_err(|output| CliError::Tool { command, output })
}

/// Runs a tool whose failure is a failed check; the inner error is the output of the tool.
fn check(process: &mut Command) -> Result<Result<(), String>, CliError> {
    let program = PathBuf::from(process.get_program());
    let output = process
        .output()
        .map_err(|source| CliError::Spawn { program, source })?;
    if output.status.success() {
        return Ok(Ok(()));
    }
    Ok(Err(format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )))
}

fn read_fields(path: &Path) -> Result<Vec<Fr>, CliError> {
    let bytes = std::fs::read(path).map_err(|source| CliError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let (chunks, remainder) = bytes.as_chunks::<32>();
    if !remainder.is_empty() {
        return Err(format_error(
            path,
            format!("{} bytes, not a multiple of 32", bytes.len()),
        ));
    }
    chunks
        .iter()
        .map(|&chunk| from_bytes(chunk))
        .collect::<Result<_, _>>()
        .map_err(|error| format_error(path, error.to_string()))
}

fn format_error(path: &Path, reason: String) -> CliError {
    CliError::Format {
        path: path.to_path_buf(),
        reason,
    }
}

fn create_dir(path: &Path) -> Result<(), CliError> {
    std::fs::create_dir_all(path).map_err(|source| CliError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn remove_dir(path: &Path) -> Result<(), CliError> {
    std::fs::remove_dir_all(path).map_err(|source| CliError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn write(path: &Path, bytes: &[u8]) -> Result<(), CliError> {
    std::fs::write(path, bytes).map_err(|source| CliError::Io {
        path: path.to_path_buf(),
        source,
    })
}
