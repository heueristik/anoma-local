use anoma_arm::Fr;
use anoma_solver::matching::{Error, KindRef, Order, solve};
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

fn order(give: u64, give_quantity: u128, want: u64, min_quantity: u128) -> Order {
    Order {
        give: kind(give),
        give_quantity,
        want: kind(want),
        min_quantity,
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
fn solve_leaves_an_order_whose_minimum_no_other_order_gives() -> anyhow::Result<()> {
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
fn solve_errors_on_a_quantity_above_the_largest_integer_of_cp_sat() {
    let orders = [
        order(APPLE, 4, BANANA, 4),
        order(BANANA, u128::from(i64::MAX as u64) + 1, APPLE, 3),
    ];

    assert_eq!(solve(&orders), Err(Error::QuantityTooLarge { index: 1 }));
}
