#![allow(dead_code)]

use std::time::Duration;

use anoma_arm::action::{Action, ResourceInstance};
use anoma_arm::app_data::{AppData, Blob, DeletionCriterion};
use anoma_arm::{Fr, Point};
use anoma_nostr_intent_pool::NostrIntentPool;
use anoma_prover::action::{LogicProof, ProvedAction};
use anoma_prover::circuit_prover::{Proof, VerificationKey};
use ark_ec::AffineRepr;
use axum::Router;
use blossom_rs::{BlobServer, MemoryBackend};
use nostr_sdk::local_relay::LocalRelay;
use tokio::net::TcpListener;

/// The largest event that the relay takes, a common limit of public relays.
pub const RELAY_EVENT_LIMIT: usize = 64 * 1024;

/// The fields of a leaf proof and of a logic key of the circuits: 14,656 and 3,680 bytes, as bb writes them.
pub const PROOF_FIELDS: usize = 458;
pub const KEY_FIELDS: usize = 115;

/// The public inputs of a compliance unit: four for each resource, then the two coordinates of the delta.
const COMPLIANCE_INPUTS_PER_RESOURCE: usize = 4;

/// The public inputs of a logic: the tag, the flag, the action tree root and the app data hash.
const LOGIC_INPUTS: usize = 4;

/// More field values than one action takes, so that no two actions share a value.
const FIELD_STRIDE: u64 = 1 << 16;

/// The lifetime of the intents of a pool that no test lets expire.
pub const LIFETIME: Duration = Duration::from_secs(3600);

pub async fn relay() -> anyhow::Result<LocalRelay> {
    let relay = LocalRelay::builder()
        .max_event_size(RELAY_EVENT_LIMIT)
        .build();
    relay.run().await?;
    Ok(relay)
}

/// A Blossom server in memory that requires a signed authorization for each upload and deletion.
pub async fn blossom() -> anyhow::Result<String> {
    serve(|url| BlobServer::new_with_auth(MemoryBackend::new(), url).router()).await
}

/// A server that answers each download with `blob`, whatever hash the request names.
pub async fn false_blossom(blob: &'static [u8]) -> anyhow::Result<String> {
    serve(|_| Router::new().route("/{sha256}", axum::routing::get(move || async move { blob })))
        .await
}

pub async fn pool(relay: &LocalRelay, blossom: &str) -> anyhow::Result<NostrIntentPool> {
    Ok(NostrIntentPool::connect(&[relay.url().await], blossom.to_string(), LIFETIME).await?)
}

/// An action that consumes one resource and creates one, with proofs and keys of the sizes of the circuits. Each field
/// of each action has its own value, so a decoding that moves a field changes the action. The pool checks no proof.
pub fn action(index: u64) -> ProvedAction {
    let start = index * FIELD_STRIDE;
    let mut next = start;
    let mut fields = |count: usize| -> Vec<Fr> {
        let fields = (next..next + count as u64).map(Fr::from).collect();
        next += count as u64;
        fields
    };
    let blob = |deletion_criterion, bytes: &[u8]| Blob {
        deletion_criterion,
        bytes: bytes.to_vec(),
    };
    let app_data = [
        AppData {
            resource_payload: vec![blob(DeletionCriterion::Immediately, b"consumed")],
            ..AppData::default()
        },
        AppData {
            resource_payload: vec![blob(DeletionCriterion::Never, b"resource")],
            discovery_payload: vec![blob(DeletionCriterion::Never, b"discovery")],
            external_payload: vec![blob(DeletionCriterion::Immediately, b"external")],
            application_payload: vec![
                blob(DeletionCriterion::Never, b""),
                blob(DeletionCriterion::Immediately, b"application"),
            ],
        },
    ];
    let resources: Vec<ResourceInstance> = app_data
        .into_iter()
        .zip([true, false])
        .map(|(app_data, is_consumed)| ResourceInstance {
            is_consumed,
            tag: fields(1)[0],
            logic_ref: fields(1)[0],
            commitment_tree_root: fields(1)[0],
            app_data,
        })
        .collect();
    let compliance_proof = Proof {
        fields: fields(PROOF_FIELDS),
        public_inputs: fields(COMPLIANCE_INPUTS_PER_RESOURCE * resources.len() + 2),
    };
    let logic_proofs = resources
        .iter()
        .map(|_| LogicProof {
            verification_key: VerificationKey {
                fields: fields(KEY_FIELDS),
                hash: fields(1)[0],
            },
            proof: Proof {
                fields: fields(PROOF_FIELDS),
                public_inputs: fields(LOGIC_INPUTS),
            },
        })
        .collect();
    assert!(
        next - start <= FIELD_STRIDE,
        "an action takes at most {FIELD_STRIDE} field values"
    );
    ProvedAction {
        action: Action { resources },
        delta: Point::generator(),
        compliance_proof,
        logic_proofs,
    }
}

/// Serves the router of `server`, which takes the URL of the server, on a free port.
async fn serve(server: impl FnOnce(&str) -> Router) -> anyhow::Result<String> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let url = format!("http://{}", listener.local_addr()?);
    let router = server(&url);
    tokio::spawn(async move { axum::serve(listener, router).await });
    Ok(url)
}
