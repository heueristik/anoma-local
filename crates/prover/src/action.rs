//! The leaf proofs of one action: its compliance unit and the logic of each of its resources.

use std::sync::Arc;

use anoma_arm::action::{Action, MAX_RESOURCES, ResourceInstance, action_tree_root};
use anoma_arm::app_data::AppData;
use anoma_arm::belt::MembershipPath;
use anoma_arm::delta::{ResourceDelta, unit_delta};
use anoma_arm::field::{coordinates, to_hex};
use anoma_arm::kind::Kind;
use anoma_arm::resource::{NullifierKey, Resource, created_nonce};
use anoma_arm::{Fr, Point, Scalar};

use crate::circuit_prover::{CircuitProver, Proof, Target, VerificationKey};
use crate::inputs::{self, Field, circuit_inputs};
use crate::resource_logic::{LogicContext, ResourceLogic};
use crate::{Error, PartialTransaction};

/// The circuit of the compliance unit of an action of `resource_count` resources.
pub fn compliance_circuit(resource_count: usize) -> String {
    format!("compliance_{resource_count}")
}

/// How an action shows that a consumed resource exists.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Membership {
    /// The path of a persistent consumed resource in the commitment tree.
    Persistent(MembershipPath),
    /// A historical root for an ephemeral consumed resource. The compliance unit does not check it, but the action
    /// shows it, so a recent root hides that the resource is ephemeral.
    Ephemeral { root: Fr },
}

/// What opens a consumed resource: its nullifier key, and what shows that the resource exists.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Consumption {
    pub nullifier_key: NullifierKey,
    pub membership: Membership,
}

/// The private data of one resource of an action.
#[derive(Clone, Debug)]
pub struct ResourceWitness {
    pub resource: Resource,
    pub logic: Arc<dyn ResourceLogic>,
    pub app_data: AppData,
    /// What opens the resource, if the action consumes it.
    pub consumption: Option<Consumption>,
}

impl ResourceWitness {
    pub fn consumed(
        resource: Resource,
        logic: Arc<dyn ResourceLogic>,
        nullifier_key: NullifierKey,
        membership: Membership,
    ) -> Self {
        Self {
            resource,
            logic,
            app_data: AppData::default(),
            consumption: Some(Consumption {
                nullifier_key,
                membership,
            }),
        }
    }

    /// A created resource; `ActionWitness::new` sets its nonce.
    pub fn created(resource: Resource, logic: Arc<dyn ResourceLogic>) -> Self {
        Self {
            resource,
            logic,
            app_data: AppData::default(),
            consumption: None,
        }
    }

    pub fn is_consumed(&self) -> bool {
        self.consumption.is_some()
    }

    /// The nullifier of a consumed resource or the commitment of a created resource.
    pub fn tag(&self) -> Result<Fr, Error> {
        Ok(match &self.consumption {
            Some(consumption) => self.resource.nullifier(consumption.nullifier_key)?,
            None => self.resource.commitment(),
        })
    }
}

/// The private data of an action: its resources in the order of its compliance unit, and the rcv of its delta.
#[derive(Clone, Debug)]
pub struct ActionWitness {
    pub resources: Vec<ResourceWitness>,
    pub rcv: Scalar,
}

impl ActionWitness {
    /// The action of `resources` in this order. It sets the nonce of each created resource from its index among the
    /// created resources and from the consumed nullifiers.
    pub fn new(mut resources: Vec<ResourceWitness>, rcv: Scalar) -> Result<Self, Error> {
        let nullifiers = resources
            .iter()
            .filter(|witness| witness.is_consumed())
            .map(ResourceWitness::tag)
            .collect::<Result<Vec<_>, _>>()?;
        for (index, created) in resources
            .iter_mut()
            .filter(|witness| !witness.is_consumed())
            .enumerate()
        {
            created.resource.nonce = created_nonce(index, &nullifiers);
        }
        Ok(Self { resources, rcv })
    }

    /// The action that creates `created` from an ephemeral resource of its kind with `consumed_quantity`, which
    /// `source` opens, and shows the historical root `commitment_tree_root`. Both resources have the logic of the kind.
    pub fn from_ephemeral(
        created: Resource,
        consumed_quantity: u128,
        source: NullifierKey,
        logic: Arc<dyn ResourceLogic>,
        commitment_tree_root: Fr,
        rcv: Scalar,
    ) -> Result<Self, Error> {
        let consumed = Resource {
            quantity: consumed_quantity,
            nullifier_key_commitment: source.commitment(),
            is_ephemeral: true,
            ..created
        };
        Self::new(
            vec![
                ResourceWitness::consumed(
                    consumed,
                    Arc::clone(&logic),
                    source,
                    Membership::Ephemeral {
                        root: commitment_tree_root,
                    },
                ),
                ResourceWitness::created(created, logic),
            ],
            rcv,
        )
    }

    /// The created resources, in the order of the action.
    pub fn created(&self) -> impl Iterator<Item = &ResourceWitness> {
        self.resources
            .iter()
            .filter(|witness| !witness.is_consumed())
    }

    /// The created resources, in the order of the action, to change their app data.
    pub fn created_mut(&mut self) -> impl Iterator<Item = &mut ResourceWitness> {
        self.resources
            .iter_mut()
            .filter(|witness| !witness.is_consumed())
    }

    pub fn tags(&self) -> Result<Vec<Fr>, Error> {
        self.resources.iter().map(ResourceWitness::tag).collect()
    }
}

/// The proof of a resource logic and the verification key whose hash is the logic ref.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LogicProof {
    pub verification_key: VerificationKey,
    pub proof: Proof,
}

/// An action, its unit delta, and its leaf proofs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProvedAction {
    pub action: Action,
    pub delta: Point,
    pub compliance_proof: Proof,
    /// The proof of the logic of each resource, in the order of the action.
    pub logic_proofs: Vec<LogicProof>,
}

impl ProvedAction {
    /// Verifies the leaf proofs against the public inputs that the step circuits give them. Each logic proof must
    /// carry one of `logic_keys`, the keys of the logics that the caller knows.
    pub fn verify(
        &self,
        prover: &impl CircuitProver,
        logic_keys: &[VerificationKey],
    ) -> Result<(), Error> {
        let resources = &self.action.resources;
        check_resource_count(resources.len())?;
        if self.logic_proofs.len() != resources.len() {
            return Err(Error::LogicProofCount {
                logic_proof_count: self.logic_proofs.len(),
                resource_count: resources.len(),
            });
        }
        let circuit = compliance_circuit(resources.len());
        prover.verify(
            &circuit,
            &prover.verification_key(&circuit)?,
            &self.compliance_proof.fields,
            &compliance_public_inputs(&self.action, self.delta),
            Target::Leaf,
        )?;
        let root = action_tree_root(&self.action.tags())?;
        for (resource, logic_proof) in resources.iter().zip(&self.logic_proofs) {
            let key = logic_keys
                .iter()
                .find(|key| key.hash == resource.logic_ref && **key == logic_proof.verification_key)
                .ok_or_else(|| Error::UnknownLogic {
                    logic_ref: to_hex(resource.logic_ref),
                })?;
            prover.verify(
                &to_hex(resource.logic_ref),
                key,
                &logic_proof.proof.fields,
                &logic_public_inputs(resource, root),
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
    let resource_count = witness.resources.len();
    check_resource_count(resource_count)?;
    let mut instances = Vec::with_capacity(resource_count);
    let mut compliance_witnesses = Vec::with_capacity(resource_count);
    let mut deltas = Vec::with_capacity(resource_count);
    for resource_witness in &witness.resources {
        let resource = &resource_witness.resource;
        let kind = Kind::new(resource.logic_ref, resource.label_ref);
        let (commitment_tree_root, nullifier_key, path) = match &resource_witness.consumption {
            Some(consumption) => match (&consumption.membership, resource.is_ephemeral) {
                (Membership::Persistent(path), false) => (
                    path.root(resource.commitment()),
                    consumption.nullifier_key.0,
                    path.clone(),
                ),
                (Membership::Ephemeral { root }, true) => (
                    *root,
                    consumption.nullifier_key.0,
                    MembershipPath::default(),
                ),
                _ => return Err(Error::Membership),
            },
            // The compliance unit takes a nullifier key and a path for each resource, but checks them only for a
            // consumed one.
            None => (Fr::from(0u64), Fr::from(0u64), MembershipPath::default()),
        };
        instances.push(ResourceInstance {
            is_consumed: resource_witness.is_consumed(),
            tag: resource_witness.tag()?,
            logic_ref: resource.logic_ref,
            commitment_tree_root,
            app_data: resource_witness.app_data.clone(),
        });
        compliance_witnesses.push(inputs::ComplianceResourceWitness {
            resource: resource.into(),
            kind: kind.into(),
            nullifier_key: Field(nullifier_key),
            path: (&path).try_into()?,
        });
        deltas.push(ResourceDelta {
            kind_base: kind.base,
            quantity: resource.quantity,
            is_consumed: resource_witness.is_consumed(),
        });
    }
    let action = Action {
        resources: instances,
    };
    let delta = unit_delta(&deltas, witness.rcv);

    let (delta_x, delta_y) = inputs::point(delta);
    let circuit = compliance_circuit(resource_count);
    let compliance_proof = prover.prove(
        &circuit,
        &circuit_inputs(
            &circuit,
            &inputs::Compliance {
                instance: inputs::ComplianceInstance {
                    resources: action.resources.iter().map(Into::into).collect(),
                    delta_x,
                    delta_y,
                },
                witness: inputs::ComplianceWitness {
                    resources: compliance_witnesses,
                    rcv: witness.rcv.into(),
                },
            },
        )?,
        Target::Leaf,
    )?;

    let action_tree_root = action_tree_root(&action.tags())?;
    let logic_proofs = action
        .resources
        .iter()
        .enumerate()
        .map(|(index, instance)| {
            prove_logic(
                prover,
                &LogicContext::new(witness, index, action_tree_root),
                instance,
            )
        })
        .collect::<Result<_, _>>()?;

    Ok(PartialTransaction {
        actions: vec![ProvedAction {
            action,
            delta,
            compliance_proof,
            logic_proofs,
        }],
        rcv_total: witness.rcv,
    })
}

/// Checks that circuits exist for an action of `resource_count` resources.
pub(crate) fn check_resource_count(resource_count: usize) -> Result<(), Error> {
    if (1..=MAX_RESOURCES).contains(&resource_count) {
        Ok(())
    } else {
        Err(Error::ResourceCount {
            resource_count,
            max: MAX_RESOURCES,
        })
    }
}

/// The public inputs of the compliance proof of an action: for each resource its `is_consumed` flag, tag, logic ref and
/// commitment tree root, then the unit delta.
fn compliance_public_inputs(action: &Action, delta: Point) -> Vec<Fr> {
    let (delta_x, delta_y) = coordinates(delta);
    action
        .resources
        .iter()
        .flat_map(|resource| {
            [
                Fr::from(resource.is_consumed),
                resource.tag,
                resource.logic_ref,
                resource.commitment_tree_root,
            ]
        })
        .chain([delta_x, delta_y])
        .collect()
}

/// The public inputs of the logic proof of a resource.
fn logic_public_inputs(resource: &ResourceInstance, action_tree_root: Fr) -> [Fr; 4] {
    [
        resource.tag,
        Fr::from(resource.is_consumed),
        action_tree_root,
        resource.app_data.hash(),
    ]
}

/// Proves the logic of the resource of `context`, after it checks that the logic ref of the resource is the key hash of
/// the logic.
fn prove_logic(
    prover: &impl CircuitProver,
    context: &LogicContext,
    instance: &ResourceInstance,
) -> Result<LogicProof, Error> {
    let logic = context.witness().logic.as_ref();
    let circuit = logic.circuit();
    let verification_key = prover.verification_key(circuit)?;
    if verification_key.hash != instance.logic_ref {
        return Err(Error::LogicRef {
            circuit: circuit.to_string(),
            logic_ref: to_hex(instance.logic_ref),
            key_hash: to_hex(verification_key.hash),
        });
    }
    let inputs = inputs::Logic {
        tag: Field(instance.tag),
        is_consumed: instance.is_consumed,
        action_tree_root: Field(context.action_tree_root()),
        app_data_hash: Field(instance.app_data.hash()),
        private_inputs: &logic.private_inputs(context)?,
    };
    let proof = prover.prove(circuit, &circuit_inputs(circuit, &inputs)?, Target::Leaf)?;
    Ok(LogicProof {
        verification_key,
        proof,
    })
}
