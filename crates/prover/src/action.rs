//! The leaf proofs of one action: its compliance unit and the logics of its two resources.

use anoma_arm::belt::MembershipPath;
use anoma_arm::delta::unit_delta;
use anoma_arm::digest::{Action, AppData};
use anoma_arm::field::to_hex;
use anoma_arm::kind::Kind;
use anoma_arm::resource::{NullifierKey, Resource, action_tree_root, created_nonce};
use anoma_arm::{Fr, Point, Scalar};

use crate::inputs::{self, Field};
use crate::toolchain::{Proof, Target, Toolchain, VerificationKey};
use crate::{Error, PartialTransaction};

/// The circuit of the compliance unit.
pub const COMPLIANCE: &str = "compliance";

/// A resource logic: its circuit, whose key hash is the logic ref, and its private inputs.
#[derive(Clone, Debug, PartialEq)]
pub struct Logic {
    pub circuit: String,
    /// The inputs of the circuit besides `tag`, `is_consumed`, `action_tree_root` and `app_data_hash`.
    pub witness: toml::Table,
}

impl Logic {
    /// The logic that accepts every resource.
    pub fn trivial() -> Self {
        Self {
            circuit: "trivial_logic".to_string(),
            witness: toml::Table::new(),
        }
    }
}

/// The private data of an action: the consumed resource and what opens it, and the created resource.
#[derive(Clone, Debug)]
pub struct ActionWitness {
    pub consumed: Resource,
    pub consumed_logic: Logic,
    pub consumed_app_data: AppData,
    pub nullifier_key: NullifierKey,
    /// The path of a persistent consumed resource in the commitment tree, or `None` for an ephemeral one.
    pub path: Option<MembershipPath>,
    pub created: Resource,
    pub created_logic: Logic,
    pub created_app_data: AppData,
    pub rcv: Scalar,
}

impl ActionWitness {
    /// The action that creates `created` from an ephemeral resource of its kind with `consumed_quantity`, which
    /// `source` opens. It sets the nonce of `created`, and both resources have the logic of the kind.
    pub fn from_ephemeral(
        created: Resource,
        consumed_quantity: u128,
        source: NullifierKey,
        logic: Logic,
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
            path: None,
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

/// Proves an action, as a partial transaction of one action.
pub fn prove(toolchain: &Toolchain, witness: &ActionWitness) -> Result<PartialTransaction, Error> {
    let nullifier = witness.consumed.nullifier(witness.nullifier_key)?;
    // The root of the empty tree is a root of every AnomaVerifier, and the compliance unit skips it for an ephemeral
    // resource.
    let (path, commitment_tree_root) = match (&witness.path, witness.consumed.is_ephemeral) {
        (Some(path), false) => (path, path.root(witness.consumed.commitment())),
        (None, true) => (&MembershipPath::default(), Fr::from(0u64)),
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
    let compliance_proof = toolchain.prove(
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
        Target::Leaf,
    )?;

    let root = action_tree_root(nullifier, commitment);
    let consumed_logic_proof = prove_logic(
        toolchain,
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
        toolchain,
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
    toolchain: &Toolchain,
    logic: &Logic,
    logic_ref: Fr,
    inputs: inputs::Logic,
) -> Result<LogicProof, Error> {
    let verification_key = toolchain.verification_key(&logic.circuit)?;
    if verification_key.hash != logic_ref {
        return Err(Error::LogicRef {
            circuit: logic.circuit.clone(),
            logic_ref: to_hex(logic_ref),
            key_hash: to_hex(verification_key.hash),
        });
    }
    let proof = toolchain.prove(&logic.circuit, &inputs, Target::Leaf)?;
    Ok(LogicProof {
        verification_key,
        proof,
    })
}
