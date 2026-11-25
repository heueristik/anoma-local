//! The pool against a relay and a Blossom server in this process.

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

use anoma_intent_pool::IntentPool;
use anoma_nostr_intent_pool::{INTENT_KIND, NostrIntentPool, encoding};
use blossom_rs::{BlossomClient, Signer};
use nostr::event::{Event, EventBuilder, EventId, FinalizeEvent, IntoEventBuilder, Kind, Tag};
use nostr::filter::Filter;
use nostr::key::Keys;
use nostr::nips::nip09::EventDeletionRequest;
use nostr::types::Timestamp;
use nostr_sdk::client::Client;
use sha2::{Digest, Sha256};

#[tokio::test]
async fn intents_lists_the_submitted_intents_with_their_actions() -> anyhow::Result<()> {
    let relay = common::relay().await?;
    let blossom = common::blossom().await?;
    let mut party = common::pool(&relay, &blossom).await?;
    let solver = common::pool(&relay, &blossom).await?;
    let actions = [
        vec![common::action(0)],
        vec![common::action(1), common::action(2)],
    ];
    let base64_length = encoding::to_bytes(&actions[0]).len().div_ceil(3) * 4;
    assert!(
        base64_length > common::RELAY_EVENT_LIMIT,
        "one action in base64 is larger than the relay takes in an event"
    );

    let mut submitted = BTreeMap::new();
    for intent in &actions {
        submitted.insert(party.submit(intent.clone()).await?, intent.clone());
    }

    let listed: BTreeMap<_, _> = solver
        .intents()
        .await?
        .into_iter()
        .map(|intent| (intent.id, intent.actions))
        .collect();
    assert_eq!(listed, submitted);
    Ok(())
}

#[tokio::test]
async fn submit_signs_each_intent_with_a_new_key_and_names_only_the_blob_hash() -> anyhow::Result<()>
{
    let relay = common::relay().await?;
    let blossom = common::blossom().await?;
    let mut party = common::pool(&relay, &blossom).await?;
    let actions = vec![common::action(0)];

    let ids = [
        party.submit(actions.clone()).await?,
        party.submit(actions.clone()).await?,
    ];

    let events = client(&relay)
        .await?
        .fetch_events(Filter::new().kind(INTENT_KIND))
        .await?;
    assert_eq!(
        events.iter().map(|event| event.id).collect::<BTreeSet<_>>(),
        BTreeSet::from(ids)
    );
    let authors: BTreeSet<_> = events.iter().map(|event| event.pubkey).collect();
    assert_eq!(authors.len(), ids.len(), "no two intents share a key");
    let blob_hash = hex::encode(Sha256::digest(encoding::to_bytes(&actions)));
    for event in &events {
        assert_eq!(event.content, "");
        let tags: Vec<&str> = event.tags.iter().map(Tag::kind).collect();
        assert_eq!(tags, ["x", "expiration"]);
        assert_eq!(
            event.tags.iter().next().and_then(Tag::content),
            Some(blob_hash.as_str())
        );
    }
    Ok(())
}

#[tokio::test]
async fn remove_withdraws_an_intent_of_this_pool_from_all_pools() -> anyhow::Result<()> {
    let relay = common::relay().await?;
    let blossom = common::blossom().await?;
    let mut party = common::pool(&relay, &blossom).await?;
    let solver = common::pool(&relay, &blossom).await?;
    let withdrawn_actions = vec![common::action(0)];
    let withdrawn = party.submit(withdrawn_actions.clone()).await?;
    let kept = party.submit(vec![common::action(1)]).await?;
    assert_eq!(
        listed(&solver).await?,
        [withdrawn, kept].into(),
        "the solver lists both intents"
    );

    party.remove(withdrawn).await?;

    assert_eq!(listed(&solver).await?, [kept].into());
    assert_eq!(listed(&party).await?, [kept].into());
    let blob_hash = hex::encode(Sha256::digest(encoding::to_bytes(&withdrawn_actions)));
    let blob_store = BlossomClient::new(vec![blossom], Signer::generate());
    assert!(
        !blob_store
            .exists(&blob_hash)
            .await
            .map_err(anyhow::Error::msg)?,
        "the blob is deleted"
    );
    Ok(())
}

#[tokio::test]
async fn remove_hides_an_intent_of_another_pool_from_this_pool_only() -> anyhow::Result<()> {
    let relay = common::relay().await?;
    let blossom = common::blossom().await?;
    let mut party = common::pool(&relay, &blossom).await?;
    let mut solver = common::pool(&relay, &blossom).await?;
    let id = party.submit(vec![common::action(0)]).await?;

    solver.remove(id).await?;

    assert_eq!(listed(&solver).await?, BTreeSet::new());
    assert_eq!(
        listed(&common::pool(&relay, &blossom).await?).await?,
        [id].into()
    );
    Ok(())
}

#[tokio::test]
async fn intents_skips_an_intent_that_a_deletion_request_on_another_relay_withdraws()
-> anyhow::Result<()> {
    let (keeping, deleting) = (common::relay().await?, common::relay().await?);
    let blossom = common::blossom().await?;
    let keys = Keys::generate();
    let blob = encoding::to_bytes(&[common::action(0)]);
    BlossomClient::new(vec![blossom.clone()], Signer::generate())
        .upload(&blob, "application/octet-stream")
        .await
        .map_err(anyhow::Error::msg)?;
    let intent = intent(&keys, &blob)?;
    for relay in [&keeping, &deleting] {
        client(relay).await?.send_event(&intent).await?;
    }
    let relays = [keeping.url().await, deleting.url().await];
    let solver = NostrIntentPool::connect(&relays, blossom, common::LIFETIME).await?;
    assert_eq!(
        listed(&solver).await?,
        [intent.id].into(),
        "the solver lists the intent"
    );

    client(&deleting)
        .await?
        .send_event(&deletion(&keys, intent.id)?)
        .await?;

    assert_eq!(
        client(&keeping)
            .await?
            .fetch_events(Filter::new().kind(INTENT_KIND))
            .await?
            .len(),
        1,
        "one relay still holds the intent"
    );
    assert_eq!(listed(&solver).await?, BTreeSet::new());
    Ok(())
}

#[tokio::test]
async fn intents_ignores_a_deletion_request_of_another_key() -> anyhow::Result<()> {
    // A relay refuses a deletion request for an event of another key that it holds, but not one for an event that it
    // does not hold.
    let (holding, other) = (common::relay().await?, common::relay().await?);
    let blossom = common::blossom().await?;
    let mut party = common::pool(&holding, &blossom).await?;
    let id = party.submit(vec![common::action(0)]).await?;
    let relays = [holding.url().await, other.url().await];
    let solver = NostrIntentPool::connect(&relays, blossom, common::LIFETIME).await?;

    client(&other)
        .await?
        .send_event(&deletion(&Keys::generate(), id)?)
        .await?;

    assert_eq!(
        client(&other)
            .await?
            .fetch_events(Filter::new().kind(Kind::EventDeletion))
            .await?
            .len(),
        1,
        "a relay of the solver holds the deletion request"
    );
    assert_eq!(listed(&solver).await?, [id].into());
    Ok(())
}

#[tokio::test]
async fn intents_skips_an_expired_intent() -> anyhow::Result<()> {
    let relay = common::relay().await?;
    let blossom = common::blossom().await?;
    let lifetime = Duration::from_secs(1);
    let mut party = NostrIntentPool::connect(&[relay.url().await], blossom, lifetime).await?;
    let id = party.submit(vec![common::action(0)]).await?;
    assert_eq!(
        listed(&party).await?,
        [id].into(),
        "the pool lists the intent before it expires"
    );

    // The expiration of the intent is in whole seconds, so it passes within two lifetimes.
    tokio::time::sleep(2 * lifetime).await;

    assert_eq!(listed(&party).await?, BTreeSet::new());
    Ok(())
}

#[tokio::test]
async fn intents_skips_an_intent_whose_blob_does_not_have_its_hash() -> anyhow::Result<()> {
    let relay = common::relay().await?;
    let blob = encoding::to_bytes(&[common::action(0)]);
    let mut other_blob = blob.clone();
    other_blob.push(0);
    let blossom = common::false_blossom(other_blob.leak()).await?;
    let pool = common::pool(&relay, &blossom).await?;

    let intent = intent(&Keys::generate(), &blob)?;
    client(&relay).await?.send_event(&intent).await?;
    assert_eq!(
        client(&relay)
            .await?
            .fetch_events(Filter::new().kind(INTENT_KIND))
            .await?
            .len(),
        1,
        "the relay holds the intent"
    );

    assert_eq!(listed(&pool).await?, BTreeSet::new());
    Ok(())
}

/// An intent event of the keys that names the blob, as the pool publishes it.
fn intent(keys: &Keys, blob: &[u8]) -> anyhow::Result<Event> {
    Ok(EventBuilder::new(INTENT_KIND, "")
        .tag(Tag::custom("x", [hex::encode(Sha256::digest(blob))]))
        .tag(Tag::expiration(Timestamp::now() + common::LIFETIME))
        .finalize(keys)?)
}

/// A deletion request of the keys for the intent event, as the pool publishes it.
fn deletion(keys: &Keys, id: EventId) -> anyhow::Result<Event> {
    Ok(EventDeletionRequest::new()
        .id(id)
        .into_event_builder()
        .tag(Tag::custom("k", [INTENT_KIND.as_u16().to_string()]))
        .finalize(keys)?)
}

/// A client of the relay with no key of its own.
async fn client(relay: &nostr_sdk::local_relay::LocalRelay) -> anyhow::Result<Client> {
    let client = Client::default();
    client.add_relay(relay.url().await).await?;
    client.connect().and_wait(Duration::from_secs(10)).await;
    Ok(client)
}

async fn listed(pool: &NostrIntentPool) -> anyhow::Result<BTreeSet<EventId>> {
    Ok(pool
        .intents()
        .await?
        .into_iter()
        .map(|intent| intent.id)
        .collect())
}
