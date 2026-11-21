//! Partial transactions, which anyone can compose, and the final proof of a balanced transaction.

use anoma_arm::digest::{Action, Deployment};
use anoma_arm::{Fr, Point, Scalar};
use ark_ec::{AffineRepr, CurveGroup};

use crate::Error;
use crate::action::{COMPLIANCE, ProvedAction};
use crate::circuit_prover::{CircuitProver, Target};
use crate::inputs::{self, Field, StepState, circuit_inputs, fields};
use crate::root_proof_wrapper::{RootProofWrapper, TransactionProof};

/// The circuit of the step proof of the first action.
pub const FIRST_STEP: &str = "first_step";
/// The circuit of the step proof of each later action.
pub const NEXT_STEP: &str = "next_step";
/// The circuit that checks the last step proof and the balance.
pub const ROOT: &str = "root";

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
/// proof, and its wrap.
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
    let compliance = prover.verification_key(COMPLIANCE)?;
    let first_step = prover.verification_key(FIRST_STEP)?;
    let next_step = prover.verification_key(NEXT_STEP)?;

    let mut last = prover.prove(
        FIRST_STEP,
        &circuit_inputs(
            FIRST_STEP,
            &inputs::FirstStep {
                compliance_vk: fields(&compliance.fields),
                compliance_key_hash: Field(compliance.hash),
                first_step_key_hash: Field(first_step.hash),
                next_step_key_hash: Field(next_step.hash),
                action: action_proofs(first),
            },
        )?,
        Target::Step,
    )?;
    let mut last_circuit = FIRST_STEP;
    let mut last_vk = &first_step;
    for action in rest {
        last = prover.prove(
            NEXT_STEP,
            &circuit_inputs(
                NEXT_STEP,
                &inputs::NextStep {
                    previous_vk: fields(&last_vk.fields),
                    previous_proof: fields(&last.fields),
                    previous: StepState::from_public_inputs(last_circuit, &last.public_inputs)?,
                    previous_is_first_step: last_circuit == FIRST_STEP,
                    compliance_vk: fields(&compliance.fields),
                    action: action_proofs(action),
                },
            )?,
            Target::Step,
        )?;
        last_circuit = NEXT_STEP;
        last_vk = &next_step;
    }

    let root = prover.prove(
        ROOT,
        &circuit_inputs(
            ROOT,
            &inputs::Root {
                last_vk: fields(&last_vk.fields),
                last_proof: fields(&last.fields),
                last: StepState::from_public_inputs(last_circuit, &last.public_inputs)?,
                last_is_first_step: last_circuit == FIRST_STEP,
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

fn action_proofs(proved: &ProvedAction) -> inputs::ActionProofs {
    let (delta_x, delta_y) = inputs::point(proved.delta);
    inputs::ActionProofs {
        instance: proved.action.instance().into(),
        delta_x,
        delta_y,
        compliance_proof: fields(&proved.compliance_proof.fields),
        consumed_logic_vk: fields(&proved.consumed_logic_proof.verification_key.fields),
        consumed_logic_proof: fields(&proved.consumed_logic_proof.proof.fields),
        created_logic_vk: fields(&proved.created_logic_proof.verification_key.fields),
        created_logic_proof: fields(&proved.created_logic_proof.proof.fields),
    }
}
