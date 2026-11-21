//! The leaf proofs of one action: its compliance unit and the logics of its two resources.

use anoma_arm::belt::MembershipPath;
use anoma_arm::delta::unit_delta;
use anoma_arm::digest::{Action, AppData};
use anoma_arm::field::{coordinates, to_hex};
use anoma_arm::kind::Kind;
use anoma_arm::resource::{NullifierKey, Resource, action_tree_root, created_nonce};
use anoma_arm::{Fr, Point, Scalar};

use crate::circuit_prover::{CircuitInputs, CircuitProver, Proof, Target, VerificationKey};
use crate::inputs::{self, Field, circuit_inputs};
use crate::{Error, PartialTransaction};

/// The circuit of the compliance unit.
pub const COMPLIANCE: &str = "compliance";

/// A resource logic: its circuit, whose key hash is the logic ref, and its private inputs.
#[derive(Clone, Debug, PartialEq)]
pub struct Logic {
    pub circuit: String,
    /// The inputs of the circuit besides `tag`, `is_consumed`, `action_tree_root` and `app_data_hash`.
    pub witness: CircuitInputs,
}

impl Logic {
    /// The logic that accepts every resource.
    pub fn trivial() -> Self {
        Self {
            circuit: "trivial_logic".to_string(),
            witness: CircuitInputs::new(),
        }
    }
}

/// How an action shows that its consumed resource exists.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Membership {
    /// The path of a persistent consumed resource in the commitment tree.
    Persistent(MembershipPath),
    /// A historical root for an ephemeral consumed resource. The compliance unit does not check it, but the action
    /// shows it, so a recent root hides that the resource is ephemeral.
    Ephemeral { root: Fr },
}

/// The private data of an action: the consumed resource and what opens it, and the created resource.
#[derive(Clone, Debug)]
pub struct ActionWitness {
    pub consumed: Resource,
    pub consumed_logic: Logic,
    pub consumed_app_data: AppData,
    pub nullifier_key: NullifierKey,
    pub membership: Membership,
    pub created: Resource,
    pub created_logic: Logic,
    pub created_app_data: AppData,
    pub rcv: Scalar,
}

impl ActionWitness {
    /// The action that creates `created` from an ephemeral resource of its kind with `consumed_quantity`, which
    /// `source` opens, and shows the historical root `commitment_tree_root`. It sets the nonce of `created`, and both
    /// resources have the logic of the kind.
    pub fn from_ephemeral(
        created: Resource,
        consumed_quantity: u128,
        source: NullifierKey,
        logic: Logic,
        commitment_tree_root: Fr,
        rcv: Scalar,
    ) -> Result<Self, Error> {
        let consumed = Resource {
            quantity: consumed_quantity,
            nullifier_key_commitment: source.commitment(),
            is_ephemeral: true,
            ..created
        };
        Ok(Self {
            created: Resource {
                nonce: created_nonce(consumed.nullifier(source)?),
                ..created
            },
            consumed,
            consumed_logic: logic.clone(),
            consumed_app_data: AppData::default(),
            nullifier_key: source,
            membership: Membership::Ephemeral {
                root: commitment_tree_root,
            },
            created_logic: logic,
            created_app_data: AppData::default(),
            rcv,
        })
    }
}

/// The proof of a resource logic and the verification key whose hash is the logic ref.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LogicProof {
    pub verification_key: VerificationKey,
    pub proof: Proof,
}

/// An action, its unit delta, and its three leaf proofs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProvedAction {
    pub action: Action,
    pub delta: Point,
    pub compliance_proof: Proof,
    pub consumed_logic_proof: LogicProof,
    pub created_logic_proof: LogicProof,
}

impl ProvedAction {
    /// Verifies the three leaf proofs against the public inputs that the step circuits give them. Each logic proof must
    /// carry one of `logic_keys`, the keys of the logics that the caller knows.
    pub fn verify(
        &self,
        prover: &impl CircuitProver,
        logic_keys: &[VerificationKey],
    ) -> Result<(), Error> {
        let action = &self.action;
        let (delta_x, delta_y) = coordinates(self.delta);
        prover.verify(
            COMPLIANCE,
            &prover.verification_key(COMPLIANCE)?,
            &self.compliance_proof.fields,
            &[
                action.nullifier,
                action.consumed_logic_ref,
                action.commitment_tree_root,
                action.commitment,
                action.created_logic_ref,
                delta_x,
                delta_y,
            ],
            Target::Leaf,
        )?;
        let root = action_tree_root(action.nullifier, action.commitment);
        for (logic_proof, logic_ref, public_inputs) in [
            (
                &self.consumed_logic_proof,
                action.consumed_logic_ref,
                [
                    action.nullifier,
                    Fr::from(1u64),
                    root,
                    action.consumed_app_data.hash(),
                ],
            ),
            (
                &self.created_logic_proof,
                action.created_logic_ref,
                [
                    action.commitment,
                    Fr::from(0u64),
                    root,
                    action.created_app_data.hash(),
                ],
            ),
        ] {
            let key = logic_keys
                .iter()
                .find(|key| key.hash == logic_ref && **key == logic_proof.verification_key)
                .ok_or_else(|| Error::UnknownLogic {
                    logic_ref: to_hex(logic_ref),
                })?;
            prover.verify(
                &to_hex(logic_ref),
                key,
                &logic_proof.proof.fields,
                &public_inputs,
                Target::Leaf,
            )?;
        }
        Ok(())
    }
}

/// Proves an action, as a partial transaction of one action.
pub fn prove(
    prover: &impl CircuitProver,
    witness: &ActionWitness,
) -> Result<PartialTransaction, Error> {
    let nullifier = witness.consumed.nullifier(witness.nullifier_key)?;
    let (path, commitment_tree_root) = match (&witness.membership, witness.consumed.is_ephemeral) {
        (Membership::Persistent(path), false) => (path, path.root(witness.consumed.commitment())),
        (Membership::Ephemeral { root }, true) => (&MembershipPath::default(), *root),
        _ => return Err(Error::Membership),
    };
    let commitment = witness.created.commitment();
    let action = Action {
        nullifier,
        consumed_logic_ref: witness.consumed.logic_ref,
        consumed_app_data: witness.consumed_app_data.clone(),
        commitment,
        created_logic_ref: witness.created.logic_ref,
        created_app_data: witness.created_app_data.clone(),
        commitment_tree_root,
    };

    let consumed_kind = Kind::new(witness.consumed.logic_ref, witness.consumed.label_ref);
    let created_kind = Kind::new(witness.created.logic_ref, witness.created.label_ref);
    let delta = unit_delta(
        consumed_kind.base,
        witness.consumed.quantity,
        created_kind.base,
        witness.created.quantity,
        witness.rcv,
    );
    let (delta_x, delta_y) = inputs::point(delta);
    let compliance_proof = prover.prove(
        COMPLIANCE,
        &circuit_inputs(
            COMPLIANCE,
            &inputs::Compliance {
                nullifier: Field(nullifier),
                consumed_logic_ref: Field(action.consumed_logic_ref),
                commitment_tree_root: Field(action.commitment_tree_root),
                commitment: Field(commitment),
                created_logic_ref: Field(action.created_logic_ref),
                delta_x,
                delta_y,
                consumed: (&witness.consumed).into(),
                nullifier_key: Field(witness.nullifier_key.0),
                path: path.try_into()?,
                consumed_kind: consumed_kind.into(),
                created: (&witness.created).into(),
                created_kind: created_kind.into(),
                rcv: witness.rcv.into(),
            },
        )?,
        Target::Leaf,
    )?;

    let root = action_tree_root(nullifier, commitment);
    let consumed_logic_proof = prove_logic(
        prover,
        &witness.consumed_logic,
        witness.consumed.logic_ref,
        inputs::Logic {
            tag: Field(nullifier),
            is_consumed: true,
            action_tree_root: Field(root),
            app_data_hash: Field(action.consumed_app_data.hash()),
            witness: &witness.consumed_logic.witness,
        },
    )?;
    let created_logic_proof = prove_logic(
        prover,
        &witness.created_logic,
        witness.created.logic_ref,
        inputs::Logic {
            tag: Field(commitment),
            is_consumed: false,
            action_tree_root: Field(root),
            app_data_hash: Field(action.created_app_data.hash()),
            witness: &witness.created_logic.witness,
        },
    )?;

    Ok(PartialTransaction {
        actions: vec![ProvedAction {
            action,
            delta,
            compliance_proof,
            consumed_logic_proof,
            created_logic_proof,
        }],
        rcv_total: witness.rcv,
    })
}

/// Proves a resource logic, after it checks that the logic ref of the resource is the key hash of the logic.
fn prove_logic(
    prover: &impl CircuitProver,
    logic: &Logic,
    logic_ref: Fr,
    inputs: inputs::Logic,
) -> Result<LogicProof, Error> {
    let verification_key = prover.verification_key(&logic.circuit)?;
    if verification_key.hash != logic_ref {
        return Err(Error::LogicRef {
            circuit: logic.circuit.clone(),
            logic_ref: to_hex(logic_ref),
            key_hash: to_hex(verification_key.hash),
        });
    }
    let proof = prover.prove(
        &logic.circuit,
        &circuit_inputs(&logic.circuit, &inputs)?,
        Target::Leaf,
    )?;
    Ok(LogicProof {
        verification_key,
        proof,
    })
}
