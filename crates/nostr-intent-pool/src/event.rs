//! The Nostr events of the pool. An intent event has no content: an `x` tag names the SHA-256 hash of its blob, and a
//! NIP-40 tag its expiration. A NIP-09 deletion request of its author withdraws it.

use std::collections::BTreeSet;

use nostr::event::{Event, EventBuilder, EventId, FinalizeEvent, IntoEventBuilder, Kind, Tag};
use nostr::filter::{Filter, SingleLetterTag};
use nostr::key::{Keys, PublicKey};
use nostr::nips::nip09::EventDeletionRequest;
use nostr::types::Timestamp;

/// The kind of an intent event. It is a regular kind, so a relay keeps each event, and no NIP assigns it.
pub const INTENT_KIND: Kind = Kind::Custom(4_211);

/// The tag of the blob hash, as Blossom and NIP-94 name it.
const BLOB_HASH: &str = "x";

/// The tag of the kinds that a deletion request withdraws.
const KIND: &str = "k";

pub(crate) fn intent(
    keys: &Keys,
    blob_hash: [u8; 32],
    expiration: Timestamp,
) -> Result<Event, nostr::error::Error> {
    EventBuilder::new(INTENT_KIND, "")
        .tag(Tag::custom(BLOB_HASH, [hex::encode(blob_hash)]))
        .tag(Tag::expiration(expiration))
        .finalize(keys)
}

pub(crate) fn deletion(keys: &Keys, id: EventId) -> Result<Event, nostr::error::Error> {
    EventDeletionRequest::new()
        .id(id)
        .into_event_builder()
        .tag(Tag::custom(KIND, [INTENT_KIND.as_u16().to_string()]))
        .finalize(keys)
}

/// The filters of the intent events and of the deletion requests that withdraw intent events.
pub(crate) fn filters() -> [Filter; 2] {
    [
        Filter::new().kind(INTENT_KIND),
        Filter::new().kind(Kind::EventDeletion).custom_tag(
            SingleLetterTag::LOWERCASE_K,
            INTENT_KIND.as_u16().to_string(),
        ),
    ]
}

/// The intent events that are not expired at `now` and that no deletion request of their author names, oldest first.
pub(crate) fn live(events: impl IntoIterator<Item = Event>, now: Timestamp) -> Vec<Event> {
    let (intents, deletions): (Vec<Event>, Vec<Event>) = events
        .into_iter()
        .partition(|event| event.kind == INTENT_KIND);
    let deleted: BTreeSet<(PublicKey, EventId)> = deletions
        .iter()
        .filter(|deletion| deletion.kind == Kind::EventDeletion)
        .flat_map(|deletion| deletion.tags.event_ids().map(|id| (deletion.pubkey, id)))
        .collect();
    let mut live: Vec<Event> = intents
        .into_iter()
        .filter(|intent| {
            !intent.is_expired_at(now) && !deleted.contains(&(intent.pubkey, intent.id))
        })
        .collect();
    live.sort_by_key(|intent| (intent.created_at, intent.id));
    live
}

/// The blob hash of an intent event, if it has one `x` tag with 64 hexadecimal digits.
pub(crate) fn blob_hash(intent: &Event) -> Option<[u8; 32]> {
    let mut tags = intent.tags.iter().filter(|tag| tag.kind() == BLOB_HASH);
    let (Some(tag), None) = (tags.next(), tags.next()) else {
        return None;
    };
    let mut hash = [0; 32];
    hex::decode_to_slice(tag.content()?, &mut hash).ok()?;
    Some(hash)
}
