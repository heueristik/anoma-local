//! Conversions of the resource machine types to the types of the contract.

use alloy::primitives::{B256, Bytes};
use anoma_arm::Fr;
use anoma_arm::action::Action;
use anoma_arm::app_data::{AppData, Blob};
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
        consumed: resource_instance(
            action.nullifier,
            action.consumed_logic_ref,
            &action.consumed_app_data,
        ),
        created: resource_instance(
            action.commitment,
            action.created_logic_ref,
            &action.created_app_data,
        ),
        commitmentTreeRoot: word(action.commitment_tree_root),
    }
}

/// A field element as the contract stores it.
pub fn word(value: Fr) -> B256 {
    B256::from(to_bytes(value))
}

fn resource_instance(
    tag: Fr,
    logic_ref: Fr,
    app_data: &AppData,
) -> IAnomaVerifier::ResourceInstance {
    IAnomaVerifier::ResourceInstance {
        tag: word(tag),
        logicRef: word(logic_ref),
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
