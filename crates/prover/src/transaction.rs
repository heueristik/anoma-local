//! Partial transactions, which anyone can compose, and the final proof of a balanced transaction.

use anoma_arm::action::{Action, MAX_RESOURCES};
use anoma_arm::digest::Deployment;
use anoma_arm::{Fr, Point, Scalar};
use ark_ec::{AffineRepr, CurveGroup};

use crate::Error;
use crate::action::{ProvedAction, check_resource_count, compliance_circuit};
use crate::circuit_prover::{CircuitProver, Target};
use crate::inputs::{self, CircuitKeys, Field, StepCircuit, StepState, circuit_inputs, fields};
use crate::root_proof_wrapper::{RootProofWrapper, TransactionProof};

/// The circuit that checks the last step proof and the balance.
pub const ROOT: &str = "root";

/// The circuit of the step proof of a first action of `resource_count` resources.
pub fn first_step_circuit(resource_count: usize) -> String {
    format!("first_step_{resource_count}")
}

/// The circuit of the step proof of a later action of `resource_count` resources.
pub fn next_step_circuit(resource_count: usize) -> String {
    format!("next_step_{resource_count}")
}

/// Proved actions whose deltas need not balance, and the sum of their rcv values, which opens their delta.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartialTransaction {
    pub actions: Vec<ProvedAction>,
    pub rcv_total: Scalar,
}

impl PartialTransaction {
    /// Appends the actions of `other`.
    pub fn compose(mut self, other: Self) -> Self {
        self.actions.extend(other.actions);
        self.rcv_total += other.rcv_total;
        self
    }

    /// The sum of the unit deltas.
    pub fn delta(&self) -> Point {
        self.actions
            .iter()
            .fold(Point::zero().into_group(), |sum, action| sum + action.delta)
            .into_affine()
    }

    /// Whether the quantities of each kind balance: then the delta is the generator times the rcv total.
    pub fn is_balanced(&self) -> bool {
        self.delta() == (Point::generator() * self.rcv_total).into_affine()
    }
}

/// A transaction as the contract takes it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    pub actions: Vec<Action>,
    pub proof: TransactionProof,
}

/// Proves a balanced partial transaction for the AnomaVerifier `deployment`: one step proof for each action, the root
/// proof, and its wrap. The step circuit of an action is the one for its number of resources.
pub fn prove(
    prover: &impl CircuitProver,
    wrapper: &impl RootProofWrapper,
    partial: &PartialTransaction,
    deployment: &Deployment,
) -> Result<Transaction, Error> {
    let (first, rest) = partial.actions.split_first().ok_or(Error::Empty)?;
    if !partial.is_balanced() {
        return Err(Error::Unbalanced);
    }
    let keys = circuit_keys(prover)?;

    let mut last_circuit = step_circuit(true, first)?;
    let mut last_name = first_step_circuit(last_circuit.size as usize);
    let mut last = prover.prove(
        &last_name,
        &circuit_inputs(
            &last_name,
            &inputs::FirstStep {
                keys: &keys,
                compliance_vk: compliance_vk(prover, first)?,
                action: action_proofs(first),
            },
        )?,
        Target::Step,
    )?;
    for action in rest {
        let circuit = step_circuit(false, action)?;
        let name = next_step_circuit(circuit.size as usize);
        last = prover.prove(
            &name,
            &circuit_inputs(
                &name,
                &inputs::NextStep {
                    previous_vk: fields(&prover.verification_key(&last_name)?.fields),
                    previous_proof: fields(&last.fields),
                    previous: StepState::from_public_inputs(&last_name, &last.public_inputs)?,
                    previous_circuit: last_circuit,
                    keys: &keys,
                    compliance_vk: compliance_vk(prover, action)?,
                    action: action_proofs(action),
                },
            )?,
            Target::Step,
        )?;
        last_circuit = circuit;
        last_name = name;
    }

    let root = prover.prove(
        ROOT,
        &circuit_inputs(
            ROOT,
            &inputs::Root {
                last_vk: fields(&prover.verification_key(&last_name)?.fields),
                last_proof: fields(&last.fields),
                last: StepState::from_public_inputs(&last_name, &last.public_inputs)?,
                last_circuit,
                rcv_total: partial.rcv_total.into(),
                chain_id: Field(Fr::from(deployment.chain_id)),
                verifier: Field(deployment.verifier_field()),
            },
        )?,
        Target::Step,
    )?;
    let proof = wrapper
        .wrap(&root)
        .map_err(|error| Error::Wrap(Box::new(error)))?;
    Ok(Transaction {
        actions: partial
            .actions
            .iter()
            .map(|action| action.action.clone())
            .collect(),
        proof,
    })
}

/// The key hashes of the compliance and step circuits of each size of action, which each step and the root check.
fn circuit_keys(prover: &impl CircuitProver) -> Result<CircuitKeys, Error> {
    let key_hashes = |circuit: fn(usize) -> String| {
        (1..=MAX_RESOURCES)
            .map(|size| Ok(Field(prover.verification_key(&circuit(size))?.hash)))
            .collect::<Result<Vec<_>, Error>>()
    };
    Ok(CircuitKeys {
        compliance: key_hashes(compliance_circuit)?,
        first_step: key_hashes(first_step_circuit)?,
        next_step: key_hashes(next_step_circuit)?,
    })
}

fn step_circuit(is_first_step: bool, proved: &ProvedAction) -> Result<StepCircuit, Error> {
    let size = proved.action.resources.len();
    check_resource_count(size)?;
    Ok(StepCircuit {
        is_first_step,
        // `check_resource_count` keeps the size at most `MAX_RESOURCES`.
        size: size as u32,
    })
}

fn compliance_vk(prover: &impl CircuitProver, proved: &ProvedAction) -> Result<Vec<Field>, Error> {
    let circuit = compliance_circuit(proved.action.resources.len());
    Ok(fields(&prover.verification_key(&circuit)?.fields))
}

fn action_proofs(proved: &ProvedAction) -> inputs::ActionProofs {
    let (delta_x, delta_y) = inputs::point(proved.delta);
    inputs::ActionProofs {
        resources: proved.action.resources.iter().map(Into::into).collect(),
        delta_x,
        delta_y,
        compliance_proof: fields(&proved.compliance_proof.fields),
        logic_vks: proved
            .logic_proofs
            .iter()
            .map(|logic| fields(&logic.verification_key.fields))
            .collect(),
        logic_proofs: proved
            .logic_proofs
            .iter()
            .map(|logic| fields(&logic.proof.fields))
            .collect(),
    }
}
