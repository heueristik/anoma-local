use anoma_arm::action::{Action, ResourceInstance};
use anoma_arm::app_data::AppData;
use anoma_arm::{Fr, Point};
use anoma_intent_pool::{IntentPool, MemoryIntentPool, PooledIntent};
use anoma_prover::action::{LogicProof, ProvedAction};
use anoma_prover::circuit_prover::Proof;

/// An action with the nullifier `nullifier` and empty proofs: the pool checks no proof.
fn action(nullifier: u64) -> ProvedAction {
    let resource = |is_consumed, tag: u64, logic_ref: u64| ResourceInstance {
        is_consumed,
        tag: Fr::from(tag),
        logic_ref: Fr::from(logic_ref),
        commitment_tree_root: Fr::from(0u64),
        app_data: AppData::default(),
    };
    ProvedAction {
        action: Action {
            resources: vec![resource(true, nullifier, 2), resource(false, 3, 4)],
        },
        delta: Point::default(),
        compliance_proof: Proof::default(),
        logic_proofs: vec![LogicProof::default(); 2],
    }
}

#[tokio::test]
async fn intents_gives_the_submitted_intents_in_the_order_of_submission() -> anyhow::Result<()> {
    let mut pool = MemoryIntentPool::default();
    let first = pool.submit(vec![action(1)]).await?;
    let second = pool.submit(vec![action(2), action(3)]).await?;

    let intents = pool.intents().await?;

    assert_eq!(
        intents,
        [
            PooledIntent {
                id: first,
                actions: vec![action(1)],
            },
            PooledIntent {
                id: second,
                actions: vec![action(2), action(3)],
            },
        ]
    );
    Ok(())
}

#[tokio::test]
async fn remove_removes_only_the_intent_with_the_id() -> anyhow::Result<()> {
    let mut pool = MemoryIntentPool::default();
    let removed = pool.submit(vec![action(1)]).await?;
    let kept = pool.submit(vec![action(2)]).await?;

    pool.remove(removed).await?;

    let ids: Vec<_> = pool
        .intents()
        .await?
        .iter()
        .map(|intent| intent.id)
        .collect();
    assert_eq!(ids, [kept]);
    Ok(())
}

#[tokio::test]
async fn submit_gives_a_new_id_to_a_copy_of_an_intent() -> anyhow::Result<()> {
    let mut pool = MemoryIntentPool::default();
    let original = pool.submit(vec![action(1)]).await?;
    pool.remove(original).await?;

    let copy = pool.submit(vec![action(1)]).await?;

    assert_ne!(copy, original);
    Ok(())
}
