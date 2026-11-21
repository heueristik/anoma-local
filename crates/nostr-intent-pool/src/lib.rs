//! An intent pool on Nostr relays. Each intent is an event signed with a new key, so the intents of one party do not
//! share a key. The event names the SHA-256 hash of a blob on a Blossom server, and the blob holds the proved actions:
//! an action of two resources is about 51 KB, more than many relays take in an event. The pool checks no proof and no
//! payload; a solver checks what it lists.
//!
//! No pool can remove an intent from the relays for all solvers. For an intent that this pool submitted, `remove`
//! publishes a NIP-09 deletion request and deletes the blob; for any intent, it hides the intent from the later lists
//! of this pool. A deletion request or an expiration does not stop a solver that holds the actions: only the
//! nullifiers on the chain do.

mod blossom;
pub mod encoding;
mod event;

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::time::Duration;

use anoma_intent_pool::{IntentPool, PooledIntent};
use anoma_prover::action::ProvedAction;
use blossom_rs::BlossomClient;
use nostr::event::{Event, EventId};
use nostr::key::Keys;
use nostr::types::{RelayUrl, Timestamp};
use nostr_sdk::client::Client;
use sha2::{Digest, Sha256};

pub use event::INTENT_KIND;

/// How long the pool waits for a relay to connect, to answer a query, or to accept an event.
const TIMEOUT: Duration = Duration::from_secs(10);

/// The content type of a blob.
const BLOB_TYPE: &str = "application/octet-stream";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Relay(#[from] nostr_sdk::error::Error),
    #[error("cannot sign the event: {0}")]
    Sign(#[from] nostr::error::Error),
    #[error("no relay accepts event {id}: {failures:?}")]
    NotAccepted {
        id: EventId,
        failures: HashMap<RelayUrl, String>,
    },
    #[error("the Blossom server does not store blob {blob_hash}: {reason}")]
    Upload { blob_hash: String, reason: String },
    #[error("the Blossom server does not delete blob {blob_hash}: {reason}")]
    Delete { blob_hash: String, reason: String },
}

/// The relays and the Blossom server of the pool, the keys of the intents that it submitted, and the intents that it
/// removed.
pub struct NostrIntentPool {
    client: Client,
    blossom_server: String,
    /// Downloads with one new key, which the Blossom server sees in each request of this pool.
    downloader: BlossomClient,
    lifetime: Duration,
    submitted: BTreeMap<EventId, Submitted>,
    removed: BTreeSet<EventId>,
}

/// The key that signs an intent event, its deletion request and the requests for its blob.
struct Submitted {
    keys: Keys,
    blob_hash: [u8; 32],
}

impl IntentPool for NostrIntentPool {
    type IntentId = EventId;
    type Error = Error;

    /// Stores the actions in a blob, then publishes an intent event that names the blob.
    async fn submit(&mut self, actions: Vec<ProvedAction>) -> Result<EventId, Error> {
        let keys = Keys::generate();
        let blob = encoding::to_bytes(&actions);
        let blob_hash: [u8; 32] = Sha256::digest(&blob).into();
        blossom::client(&self.blossom_server, keys.clone())
            .upload(&blob, BLOB_TYPE)
            .await
            .map_err(|reason| Error::Upload {
                blob_hash: hex::encode(blob_hash),
                reason,
            })?;
        let intent = event::intent(&keys, blob_hash, Timestamp::now() + self.lifetime)?;
        self.publish(&intent).await?;
        self.submitted
            .insert(intent.id, Submitted { keys, blob_hash });
        Ok(intent.id)
    }

    /// The intents on the relays, without the expired, withdrawn and removed intents, oldest first by the creation time
    /// of their events. It skips an intent whose blob it cannot download, or whose blob does not decode.
    async fn intents(&self) -> Result<Vec<PooledIntent<EventId>>, Error> {
        let events = self
            .client
            .fetch_events(event::filters())
            .timeout(TIMEOUT)
            .await?;
        let mut intents = Vec::new();
        for intent in event::live(events, Timestamp::now()) {
            if self.removed.contains(&intent.id) {
                continue;
            }
            if let Some(actions) = self.actions(&intent).await {
                intents.push(PooledIntent {
                    id: intent.id,
                    actions,
                });
            }
        }
        Ok(intents)
    }

    /// Hides the intent from the later lists of this pool. If this pool submitted it, it also publishes a deletion
    /// request and deletes the blob. On an error, the pool keeps the intent.
    async fn remove(&mut self, id: EventId) -> Result<(), Error> {
        if let Some(submitted) = self.submitted.get(&id) {
            self.publish(&event::deletion(&submitted.keys, id)?).await?;
            let blob_hash = hex::encode(submitted.blob_hash);
            blossom::client(&self.blossom_server, submitted.keys.clone())
                .delete(&blob_hash)
                .await
                .map_err(|reason| Error::Delete { blob_hash, reason })?;
            self.submitted.remove(&id);
        }
        self.removed.insert(id);
        Ok(())
    }
}

impl NostrIntentPool {
    /// Connects to the relays. Each intent that the pool submits expires `lifetime` after its submission.
    pub async fn connect(
        relays: &[RelayUrl],
        blossom_server: String,
        lifetime: Duration,
    ) -> Result<Self, Error> {
        let client = Client::default();
        for relay in relays {
            client.add_relay(relay).await?;
        }
        client.connect().and_wait(TIMEOUT).await;
        Ok(Self {
            client,
            downloader: blossom::client(&blossom_server, Keys::generate()),
            blossom_server,
            lifetime,
            submitted: BTreeMap::new(),
            removed: BTreeSet::new(),
        })
    }

    /// Sends the event to all relays, and fails if no relay accepts it.
    async fn publish(&self, event: &Event) -> Result<(), Error> {
        let output = self.client.send_event(event).ok_timeout(TIMEOUT).await?;
        if output.success.is_empty() {
            return Err(Error::NotAccepted {
                id: event.id,
                failures: output.failed,
            });
        }
        Ok(())
    }

    /// The actions of the blob of an intent event, if the blob decodes. `BlossomClient::download` rejects a blob whose
    /// SHA-256 hash is not the one that the event names.
    async fn actions(&self, intent: &Event) -> Option<Vec<ProvedAction>> {
        let blob_hash = event::blob_hash(intent)?;
        let blob = self
            .downloader
            .download(&hex::encode(blob_hash))
            .await
            .ok()?;
        encoding::from_bytes(&blob)
    }
}
