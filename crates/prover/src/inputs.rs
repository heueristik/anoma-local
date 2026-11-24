//! The inputs of the circuits, with the names and shapes of their Noir parameters, as nargo reads them. A resource
//! logic builds its private inputs from the public types.

use anoma_arm::action::ResourceInstance;
use anoma_arm::field::{coordinates, scalar_limbs, to_hex};
use anoma_arm::{Fr, Point};
use serde::{Serialize, Serializer};

use crate::Error;
use crate::circuit_prover::CircuitInputs;

/// A field element, which nargo reads as a hexadecimal string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Field(pub Fr);

impl Serialize for Field {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&to_hex(self.0))
    }
}

/// A quantity, which nargo reads as a decimal string, because TOML integers have 64 bits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Quantity(pub u128);

impl Serialize for Quantity {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&self.0)
    }
}

/// The inputs of `circuit` by their Noir names.
pub fn circuit_inputs(circuit: &str, inputs: &impl Serialize) -> Result<CircuitInputs, Error> {
    CircuitInputs::try_from(inputs).map_err(|source| Error::Inputs {
        circuit: circuit.to_string(),
        source,
    })
}

pub fn fields(values: &[Fr]) -> Vec<Field> {
    values.iter().copied().map(Field).collect()
}

/// An `EmbeddedCurveScalar`: the low and high 128 bits of a Grumpkin scalar.
#[derive(Serialize)]
pub(crate) struct Scalar {
    lo: Field,
    hi: Field,
}

impl From<anoma_arm::Scalar> for Scalar {
    fn from(scalar: anoma_arm::Scalar) -> Self {
        let (lo, hi) = scalar_limbs(scalar);
        Self {
            lo: Field(lo),
            hi: Field(hi),
        }
    }
}

#[derive(Serialize)]
pub struct Resource {
    logic_ref: Field,
    label_ref: Field,
    value_ref: Field,
    quantity: Quantity,
    nonce: Field,
    nullifier_key_commitment: Field,
    is_ephemeral: bool,
    rand_seed: Field,
}

impl From<&anoma_arm::resource::Resource> for Resource {
    fn from(resource: &anoma_arm::resource::Resource) -> Self {
        Self {
            logic_ref: Field(resource.logic_ref),
            label_ref: Field(resource.label_ref),
            value_ref: Field(resource.value_ref),
            quantity: Quantity(resource.quantity),
            nonce: Field(resource.nonce),
            nullifier_key_commitment: Field(resource.nullifier_key_commitment),
            is_ephemeral: resource.is_ephemeral,
            rand_seed: Field(resource.rand_seed),
        }
    }
}

#[derive(Serialize)]
pub(crate) struct KindWitness {
    counter: Field,
    y: Field,
}

impl From<anoma_arm::kind::Kind> for KindWitness {
    fn from(kind: anoma_arm::kind::Kind) -> Self {
        Self {
            counter: Field(kind.counter),
            y: Field(kind.base.y),
        }
    }
}

/// A `MembershipPath`, padded to the largest path length.
#[derive(Serialize)]
pub(crate) struct MembershipPath {
    siblings: Vec<Field>,
    sibling_is_left: Vec<bool>,
    length: u32,
}

impl TryFrom<&anoma_arm::belt::MembershipPath> for MembershipPath {
    type Error = Error;

    fn try_from(path: &anoma_arm::belt::MembershipPath) -> Result<Self, Error> {
        let max = anoma_arm::belt::MAX_PATH_LENGTH;
        let length = path.siblings.len();
        if length > max {
            return Err(Error::PathTooLong { length, max });
        }
        let mut siblings: Vec<Field> = path
            .siblings
            .iter()
            .map(|sibling| Field(sibling.hash))
            .collect();
        siblings.resize(max, Field(Fr::from(0u64)));
        let mut sibling_is_left: Vec<bool> = path
            .siblings
            .iter()
            .map(|sibling| sibling.is_left)
            .collect();
        sibling_is_left.resize(max, false);
        Ok(Self {
            siblings,
            sibling_is_left,
            length: u32::try_from(length).expect("the length is at most MAX_PATH_LENGTH"),
        })
    }
}

/// A `ResourceInstance` of the compliance circuit.
#[derive(Serialize)]
pub(crate) struct ComplianceResourceInstance {
    is_consumed: bool,
    tag: Field,
    logic_ref: Field,
    commitment_tree_root: Field,
}

impl From<&ResourceInstance> for ComplianceResourceInstance {
    fn from(resource: &ResourceInstance) -> Self {
        Self {
            is_consumed: resource.is_consumed,
            tag: Field(resource.tag),
            logic_ref: Field(resource.logic_ref),
            commitment_tree_root: Field(resource.commitment_tree_root),
        }
    }
}

/// A `ResourceWitness` of the compliance circuit.
#[derive(Serialize)]
pub(crate) struct ComplianceResourceWitness {
    pub resource: Resource,
    pub kind: KindWitness,
    pub nullifier_key: Field,
    pub path: MembershipPath,
}

#[derive(Serialize)]
pub(crate) struct ComplianceInstance {
    pub resources: Vec<ComplianceResourceInstance>,
    pub delta_x: Field,
    pub delta_y: Field,
}

#[derive(Serialize)]
pub(crate) struct ComplianceWitness {
    pub resources: Vec<ComplianceResourceWitness>,
    pub rcv: Scalar,
}

#[derive(Serialize)]
pub(crate) struct Compliance {
    pub instance: ComplianceInstance,
    pub witness: ComplianceWitness,
}

/// The four public inputs of every resource logic, and the private inputs of one logic.
#[derive(Serialize)]
pub(crate) struct Logic<'a> {
    pub tag: Field,
    pub is_consumed: bool,
    pub action_tree_root: Field,
    pub app_data_hash: Field,
    #[serde(flatten)]
    pub private_inputs: &'a CircuitInputs,
}

/// An `ActionTreePath`, which a resource logic takes to find a resource in its action.
#[derive(Serialize)]
pub struct ActionTreePath {
    siblings: Vec<Field>,
    index: u32,
}

impl From<&anoma_arm::action::ActionTreePath> for ActionTreePath {
    fn from(path: &anoma_arm::action::ActionTreePath) -> Self {
        Self {
            siblings: fields(&path.siblings),
            // An index of an action tree is below `MAX_RESOURCES`.
            index: path.index as u32,
        }
    }
}

/// A `ResourceInstance` of the digest of the step circuits: the public data of a resource with its app data hash.
#[derive(Serialize)]
pub(crate) struct StepResourceInstance {
    is_consumed: bool,
    tag: Field,
    logic_ref: Field,
    commitment_tree_root: Field,
    app_data_hash: Field,
}

impl From<&ResourceInstance> for StepResourceInstance {
    fn from(resource: &ResourceInstance) -> Self {
        Self {
            is_consumed: resource.is_consumed,
            tag: Field(resource.tag),
            logic_ref: Field(resource.logic_ref),
            commitment_tree_root: Field(resource.commitment_tree_root),
            app_data_hash: Field(resource.app_data.hash()),
        }
    }
}

#[derive(Serialize)]
pub(crate) struct ActionProofs {
    pub resources: Vec<StepResourceInstance>,
    pub delta_x: Field,
    pub delta_y: Field,
    pub compliance_proof: Vec<Field>,
    pub logic_vks: Vec<Vec<Field>>,
    pub logic_proofs: Vec<Vec<Field>>,
}

/// The key hashes of the compliance and step circuits; entry k - 1 is the circuit of an action of k resources.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct CircuitKeys {
    pub compliance: Vec<Field>,
    pub first_step: Vec<Field>,
    pub next_step: Vec<Field>,
}

/// A step circuit: the first or a next step, for an action of `size` resources.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct StepCircuit {
    pub is_first_step: bool,
    pub size: u32,
}

/// A `StepState`, which is also the list of public inputs of a step proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct StepState {
    pub digest: Field,
    pub delta_x: Field,
    pub delta_y: Field,
    pub keys_hash: Field,
}

impl StepState {
    pub fn from_public_inputs(circuit: &str, public_inputs: &[Fr]) -> Result<Self, Error> {
        let &[digest, delta_x, delta_y, keys_hash] = public_inputs else {
            return Err(Error::PublicInputCount {
                circuit: circuit.to_string(),
                count: public_inputs.len(),
                expected: 4,
            });
        };
        Ok(Self {
            digest: Field(digest),
            delta_x: Field(delta_x),
            delta_y: Field(delta_y),
            keys_hash: Field(keys_hash),
        })
    }
}

#[derive(Serialize)]
pub(crate) struct FirstStep<'a> {
    pub keys: &'a CircuitKeys,
    pub compliance_vk: Vec<Field>,
    pub action: ActionProofs,
}

#[derive(Serialize)]
pub(crate) struct NextStep<'a> {
    pub previous_vk: Vec<Field>,
    pub previous_proof: Vec<Field>,
    pub previous: StepState,
    pub previous_circuit: StepCircuit,
    pub keys: &'a CircuitKeys,
    pub compliance_vk: Vec<Field>,
    pub action: ActionProofs,
}

#[derive(Serialize)]
pub(crate) struct Root {
    pub last_vk: Vec<Field>,
    pub last_proof: Vec<Field>,
    pub last: StepState,
    pub last_circuit: StepCircuit,
    pub rcv_total: Scalar,
    pub chain_id: Field,
    pub verifier: Field,
}

/// The x and y coordinates of a point.
pub(crate) fn point(point: Point) -> (Field, Field) {
    let (x, y) = coordinates(point);
    (Field(x), Field(y))
}
