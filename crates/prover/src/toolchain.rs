//! The runs of nargo and bb on the compiled circuits. bb writes each file as 32-byte big-endian fields.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use anoma_arm::Fr;
use anoma_arm::field::{from_bytes, to_bytes};
use serde::Serialize;

use crate::Error;

/// The bb target of a proof: a leaf proof hides its witness, a step or root proof does not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Leaf,
    Step,
}

impl Target {
    fn name(self) -> &'static str {
        match self {
            Self::Leaf => "noir-recursive",
            Self::Step => "noir-recursive-no-zk",
        }
    }
}

/// The verification key of a circuit, as the fields that a recursive verifier takes, and its hash.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VerificationKey {
    pub fields: Vec<Fr>,
    pub hash: Fr,
}

/// A proof, as the fields that a recursive verifier takes, and its public inputs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Proof {
    pub fields: Vec<Fr>,
    pub public_inputs: Vec<Fr>,
}

/// The nargo and bb binaries, the compiled circuit workspace, and a directory for the files of the runs.
#[derive(Clone, Debug)]
pub struct Toolchain {
    pub nargo: PathBuf,
    pub bb: PathBuf,
    pub circuits: PathBuf,
    pub work: PathBuf,
}

/// Makes the names of the files of each run unique in the process.
static RUN: AtomicU64 = AtomicU64::new(0);

impl Toolchain {
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

    /// Writes the verification key of a circuit on the first call, and reads it.
    pub fn verification_key(&self, circuit: &str) -> Result<VerificationKey, Error> {
        let directory = self.key_directory(circuit)?;
        let hash_file = directory.join("vk_hash");
        let hash = read_fields(&hash_file)?;
        let [hash] = hash[..] else {
            return Err(format_error(
                &hash_file,
                format!("{} fields, expected 1", hash.len()),
            ));
        };
        Ok(VerificationKey {
            fields: read_fields(&directory.join("vk"))?,
            hash,
        })
    }

    /// The hash of the verification key of a circuit, which is the logic ref of a resource logic.
    pub fn key_hash(&self, circuit: &str) -> Result<Fr, Error> {
        Ok(self.verification_key(circuit)?.hash)
    }

    /// Solves the circuit for the inputs, proves it, and checks the proof. A failed run keeps its files.
    pub fn prove(
        &self,
        circuit: &str,
        inputs: &impl Serialize,
        target: Target,
    ) -> Result<Proof, Error> {
        let run = format!(
            "{circuit}-{}-{}",
            std::process::id(),
            RUN.fetch_add(1, Ordering::Relaxed)
        );
        let directory = self.work.join("proofs").join(&run);
        create_dir(&directory)?;

        let prover_file = directory.join("inputs");
        let text = toml::to_string(inputs).map_err(|source| Error::Inputs {
            circuit: circuit.to_string(),
            source,
        })?;
        write(&prover_file.with_extension("toml"), text.as_bytes())?;
        self.run(
            "nargo execute",
            circuit,
            Command::new(&self.nargo)
                .arg("execute")
                .arg("--program-dir")
                .arg(&self.circuits)
                .args(["--package", circuit, "--silence-warnings", "--prover-name"])
                .arg(&prover_file)
                .arg(&run),
        )?;
        let solved = self.circuits.join("target").join(format!("{run}.gz"));
        let witness = directory.join("witness.gz");
        std::fs::rename(&solved, &witness).map_err(|source| Error::Io {
            path: solved,
            source,
        })?;

        let key_directory = self.key_directory(circuit)?;
        self.run(
            "bb prove",
            circuit,
            Command::new(&self.bb)
                .args(["prove", "--verify", "-t", target.name(), "-b"])
                .arg(self.artifact(circuit))
                .arg("-w")
                .arg(&witness)
                .arg("-k")
                .arg(key_directory.join("vk"))
                .arg("-o")
                .arg(&directory),
        )?;
        let proof = Proof {
            fields: read_fields(&directory.join("proof"))?,
            public_inputs: read_fields(&directory.join("public_inputs"))?,
        };
        std::fs::remove_dir_all(&directory).map_err(|source| Error::Io {
            path: directory,
            source,
        })?;
        Ok(proof)
    }

    /// The directory with the `vk` and `vk_hash` files of a circuit, which this writes on the first call.
    fn key_directory(&self, circuit: &str) -> Result<PathBuf, Error> {
        let directory = self.work.join("vk").join(circuit);
        if !directory.join("vk_hash").exists() {
            create_dir(&directory)?;
            // The key of a circuit is the same for both targets.
            self.run(
                "bb write_vk",
                circuit,
                Command::new(&self.bb)
                    .args(["write_vk", "-t", Target::Step.name(), "-b"])
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

    fn run(
        &self,
        command: &'static str,
        circuit: &str,
        process: &mut Command,
    ) -> Result<(), Error> {
        let program = PathBuf::from(process.get_program());
        let output = process
            .output()
            .map_err(|source| Error::Spawn { program, source })?;
        if output.status.success() {
            return Ok(());
        }
        Err(Error::Tool {
            command,
            circuit: circuit.to_string(),
            output: format!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ),
        })
    }
}

/// The fields as bb writes them, and as the Groth16 wrap reads them.
pub fn to_field_bytes(fields: &[Fr]) -> Vec<u8> {
    fields.iter().flat_map(|&field| to_bytes(field)).collect()
}

fn read_fields(path: &Path) -> Result<Vec<Fr>, Error> {
    let bytes = std::fs::read(path).map_err(|source| Error::Io {
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
    Ok(chunks
        .iter()
        .map(|&chunk| from_bytes(chunk))
        .collect::<Result<_, _>>()?)
}

fn format_error(path: &Path, reason: String) -> Error {
    Error::Format {
        path: path.to_path_buf(),
        reason,
    }
}

fn create_dir(path: &Path) -> Result<(), Error> {
    std::fs::create_dir_all(path).map_err(|source| Error::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn write(path: &Path, bytes: &[u8]) -> Result<(), Error> {
    std::fs::write(path, bytes).map_err(|source| Error::Io {
        path: path.to_path_buf(),
        source,
    })
}
