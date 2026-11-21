//! The choice of the intents to fill, as an integer program for CP-SAT: one Boolean for each intent, and for each kind
//! the constraint that the filled intents give at least the quantities that they want.

use std::collections::{BTreeMap, BTreeSet};

use anoma_arm::Fr;
use cp_sat::builder::{CpModelBuilder, LinearExpr};
use cp_sat::proto::CpSolverStatus;

/// The kind of a resource: its logic ref and its label ref.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KindRef {
    pub logic_ref: Fr,
    pub label_ref: Fr,
}

/// What an intent gives and what it wants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Order {
    pub give: KindRef,
    pub give_quantity: u128,
    pub want: KindRef,
    pub want_quantity: u128,
}

/// The orders to fill, by index, and what is left of each kind after each filled order gets the quantity that it wants.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Matching {
    pub chosen: Vec<usize>,
    /// The kinds with a surplus above 0.
    pub surplus: BTreeMap<KindRef, u128>,
}

/// The largest sum of the quantities of one kind that the orders in the model give, or that they want: CP-SAT rejects a
/// constraint whose terms can sum above half of the largest 64-bit integer.
pub const MAX_SUM: i64 = i64::MAX / 2;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("CP-SAT found no solution: {0:?}")]
    Status(CpSolverStatus),
}

/// The most orders that balance together, if each filled order gets the quantity that it wants and the solver keeps
/// the surplus. It leaves out the orders that would raise a sum of a kind above `MAX_SUM`.
pub fn solve(orders: &[Order]) -> Result<Matching, Error> {
    let modeled = within_max_sum(orders);

    let mut model = CpModelBuilder::default();
    let fill: Vec<_> = modeled.iter().map(|_| model.new_bool_var()).collect();
    let kinds: BTreeSet<KindRef> = modeled
        .iter()
        .flat_map(|order| [orders[order.index].give, orders[order.index].want])
        .collect();
    for kind in kinds {
        let mut balance = LinearExpr::default();
        for (modeled, &filled) in modeled.iter().zip(&fill) {
            let order = &orders[modeled.index];
            if order.give == kind {
                balance += (modeled.give, filled);
            }
            if order.want == kind {
                balance -= (modeled.want, filled);
            }
        }
        model.add_ge(balance, 0);
    }
    model.maximize(
        fill.iter()
            .map(|&filled| (1, filled))
            .collect::<LinearExpr>(),
    );

    let response = model.solve();
    let status = response.status();
    if !matches!(status, CpSolverStatus::Optimal | CpSolverStatus::Feasible) {
        return Err(Error::Status(status));
    }
    let chosen: Vec<usize> = modeled
        .iter()
        .zip(&fill)
        .filter(|(_, filled)| filled.solution_value(&response))
        .map(|(order, _)| order.index)
        .collect();
    // The constraints keep each balance at 0 or above.
    let mut balances: BTreeMap<KindRef, u128> = BTreeMap::new();
    for &index in &chosen {
        *balances.entry(orders[index].give).or_default() += orders[index].give_quantity;
    }
    for &index in &chosen {
        *balances.entry(orders[index].want).or_default() -= orders[index].want_quantity;
    }
    balances.retain(|_, surplus| *surplus > 0);
    Ok(Matching {
        chosen,
        surplus: balances,
    })
}

/// An order in the model: its index, and its quantities as integers of CP-SAT.
struct Modeled {
    index: usize,
    give: i64,
    want: i64,
}

/// The orders that the model holds, in the order of `orders`. It takes them smallest first, so that large orders cannot
/// keep small ones out, and skips each order that would raise a sum of a kind above `MAX_SUM`.
fn within_max_sum(orders: &[Order]) -> Vec<Modeled> {
    let mut by_size: Vec<usize> = (0..orders.len()).collect();
    by_size.sort_by_key(|&index| orders[index].give_quantity.max(orders[index].want_quantity));
    let mut gives = BTreeMap::new();
    let mut wants = BTreeMap::new();
    let mut modeled = Vec::new();
    for index in by_size {
        let order = &orders[index];
        let (Some((give, give_sum)), Some((want, want_sum))) = (
            raised(&gives, order.give, order.give_quantity),
            raised(&wants, order.want, order.want_quantity),
        ) else {
            continue;
        };
        gives.insert(order.give, give_sum);
        wants.insert(order.want, want_sum);
        modeled.push(Modeled { index, give, want });
    }
    modeled.sort_by_key(|order| order.index);
    modeled
}

/// The quantity as an integer of CP-SAT and the sum of its kind with it, if that sum is at most `MAX_SUM`.
fn raised(sums: &BTreeMap<KindRef, i64>, kind: KindRef, quantity: u128) -> Option<(i64, i64)> {
    let quantity = i64::try_from(quantity).ok()?;
    let sum = sums
        .get(&kind)
        .copied()
        .unwrap_or(0)
        .checked_add(quantity)?;
    (sum <= MAX_SUM).then_some((quantity, sum))
}
