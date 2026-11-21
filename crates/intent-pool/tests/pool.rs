use anoma_arm::digest::{Action, AppData};
use anoma_arm::{Fr, Point};
use anoma_discovery::{ReceiverKeys, app_data};
use anoma_intent_pool::IntentPool;
use anoma_prover::action::{LogicProof, ProvedAction};
use anoma_prover::toolchain::Proof;

/// An action whose created resource has the app data, with empty proofs: the pool checks no proof.
fn action(created_app_data: AppData) -> ProvedAction {
    ProvedAction {
        action: Action {
            nullifier: Fr::from(1u64),
            consumed_logic_ref: Fr::from(2u64),
            consumed_app_data: AppData::default(),
            commitment: Fr::from(3u64),
            created_logic_ref: Fr::from(4u64),
            created_app_data,
            commitment_tree_root: Fr::from(0u64),
        },
        delta: Point::default(),
        compliance_proof: Proof::default(),
        consumed_logic_proof: LogicProof::default(),
        created_logic_proof: LogicProof::default(),
    }
}

#[test]
fn discover_finds_only_the_intents_for_the_keys_with_their_payloads() -> anyhow::Result<()> {
    let solver = ReceiverKeys::random();
    let mut pool = IntentPool::default();
    let for_solver = pool.submit(vec![action(app_data(&solver.receiver(), b"terms")?)]);
    pool.submit(vec![action(app_data(
        &ReceiverKeys::random().receiver(),
        b"other terms",
    )?)]);
    pool.submit(vec![action(AppData::default())]);

    let discovered = pool.discover(&solver);

    let [found] = &discovered[..] else {
        anyhow::bail!("expected one intent, got {discovered:?}");
    };
    assert_eq!(found.id, for_solver);
    assert_eq!(found.payloads, [b"terms".to_vec()]);
    Ok(())
}

#[test]
fn take_removes_the_intent_from_the_pool() -> anyhow::Result<()> {
    let solver = ReceiverKeys::random();
    let mut pool = IntentPool::default();
    let id = pool.submit(vec![action(app_data(&solver.receiver(), b"terms")?)]);

    let taken = pool.take(id);

    assert!(taken.is_some());
    assert!(pool.is_empty());
    assert!(pool.discover(&solver).is_empty());
    Ok(())
}
