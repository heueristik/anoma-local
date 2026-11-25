//! Conversions of the resource machine types to the types of the contract.

use alloy::primitives::{B256, Bytes};
use anoma_arm::Fr;
use anoma_arm::action::{Action, ResourceInstance};
use anoma_arm::app_data::Blob;
use anoma_arm::field::to_bytes;

use crate::generated::anoma_verifier::IAnomaVerifier;

/// The transaction that `AnomaVerifier.execute` takes.
pub fn transaction(actions: &[Action], proof: &[u8]) -> IAnomaVerifier::Transaction {
    IAnomaVerifier::Transaction {
        actions: actions.iter().map(action).collect(),
        proof: Bytes::copy_from_slice(proof),
    }
}

pub fn action(action: &Action) -> IAnomaVerifier::Action {
    IAnomaVerifier::Action {
        resources: action.resources.iter().map(resource).collect(),
    }
}

/// A field element as the contract stores it.
pub fn word(value: Fr) -> B256 {
    B256::from(to_bytes(value))
}

fn resource(resource: &ResourceInstance) -> IAnomaVerifier::Resource {
    let app_data = &resource.app_data;
    IAnomaVerifier::Resource {
        isConsumed: resource.is_consumed,
        tag: word(resource.tag),
        logicRef: word(resource.logic_ref),
        commitmentTreeRoot: word(resource.commitment_tree_root),
        appData: IAnomaVerifier::AppData {
            resourcePayload: blobs(&app_data.resource_payload),
            discoveryPayload: blobs(&app_data.discovery_payload),
            externalPayload: blobs(&app_data.external_payload),
            applicationPayload: blobs(&app_data.application_payload),
        },
    }
}

fn blobs(blobs: &[Blob]) -> Vec<IAnomaVerifier::ExpirableBlob> {
    blobs
        .iter()
        .map(|blob| IAnomaVerifier::ExpirableBlob {
            deletionCriterion: blob.deletion_criterion as u8,
            blob: Bytes::copy_from_slice(&blob.bytes),
        })
        .collect()
}
