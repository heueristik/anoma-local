use anoma_arm::Fr;
use anoma_solver::matching::{KindRef, MAX_SUM, Order, solve};
use std::collections::BTreeMap;

fn kind(label: u64) -> KindRef {
    KindRef {
        logic_ref: Fr::from(1u64),
        label_ref: Fr::from(label),
    }
}

const APPLE: u64 = 1;
const BANANA: u64 = 2;
const CHERRY: u64 = 3;

fn order(give: u64, give_quantity: u128, want: u64, want_quantity: u128) -> Order {
    Order {
        give: kind(give),
        give_quantity,
        want: kind(want),
        want_quantity,
    }
}

#[test]
fn solve_fills_two_orders_that_give_at_least_what_the_other_wants() -> anyhow::Result<()> {
    let orders = [order(APPLE, 4, BANANA, 4), order(BANANA, 7, APPLE, 3)];

    let matching = solve(&orders)?;

    assert_eq!(matching.chosen, [0, 1]);
    assert_eq!(
        matching.surplus,
        BTreeMap::from([(kind(APPLE), 4 - 3), (kind(BANANA), 7 - 4)])
    );
    Ok(())
}

#[test]
fn solve_fills_a_ring_of_three_orders_of_which_no_two_balance() -> anyhow::Result<()> {
    let orders = [
        order(APPLE, 2, BANANA, 2),
        order(BANANA, 2, CHERRY, 2),
        order(CHERRY, 2, APPLE, 2),
    ];

    let matching = solve(&orders)?;

    assert_eq!(matching.chosen, [0, 1, 2]);
    assert_eq!(
        matching.surplus,
        BTreeMap::new(),
        "each order gives what the next wants"
    );
    Ok(())
}

#[test]
fn solve_leaves_an_order_whose_wanted_quantity_no_other_order_gives() -> anyhow::Result<()> {
    let orders = [
        order(APPLE, 4, BANANA, 4),
        order(BANANA, 7, APPLE, 3),
        order(CHERRY, 5, APPLE, 10),
    ];

    assert_eq!(solve(&orders)?.chosen, [0, 1]);
    Ok(())
}

#[test]
fn solve_fills_no_order_of_a_single_party() -> anyhow::Result<()> {
    assert_eq!(solve(&[order(APPLE, 4, BANANA, 4)])?, Default::default());
    Ok(())
}

#[test]
fn solve_fills_orders_whose_quantities_of_a_kind_sum_to_the_max_sum() -> anyhow::Result<()> {
    let max = u128::try_from(MAX_SUM)?;
    let orders = [
        order(APPLE, max, BANANA, max),
        order(BANANA, max, APPLE, max),
    ];

    assert_eq!(solve(&orders)?.chosen, [0, 1]);
    Ok(())
}

#[test]
fn solve_leaves_out_the_order_that_raises_a_sum_of_a_kind_above_the_max_sum() -> anyhow::Result<()>
{
    let large = u128::try_from(MAX_SUM)? - 3;
    let orders = [
        order(APPLE, 4, BANANA, 4),
        order(BANANA, 7, APPLE, 3),
        order(CHERRY, large, APPLE, large),
        order(APPLE, large, CHERRY, large),
    ];
    assert!(
        4 + large > u128::try_from(MAX_SUM)?,
        "the premise is that the apples of orders 0 and 3 sum above the max sum, and orders 2 and 3 balance alone"
    );

    assert_eq!(solve(&orders)?.chosen, [0, 1]);
    Ok(())
}

#[test]
fn solve_leaves_out_an_order_with_a_quantity_above_the_largest_integer_of_cp_sat()
-> anyhow::Result<()> {
    let orders = [
        order(APPLE, 4, BANANA, 4),
        order(BANANA, 7, APPLE, 3),
        order(APPLE, u128::MAX, BANANA, 1),
    ];

    assert_eq!(solve(&orders)?.chosen, [0, 1]);
    Ok(())
}
