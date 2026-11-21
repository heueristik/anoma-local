//! The choice of the intents to fill, as an integer program for CP-SAT: one Boolean for each intent, and for each kind
//! the constraint that the filled intents give at least the minimums that they want.

use std::collections::BTreeSet;

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
    pub min_quantity: u128,
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("order {index} has a quantity above the largest integer of CP-SAT, 2^63 - 1")]
    QuantityTooLarge { index: usize },
    #[error("CP-SAT found no solution: {0:?}")]
    Status(CpSolverStatus),
}

/// The indices of the most orders that balance together, if each filled order gets its minimum and the solver keeps
/// the rest.
pub fn solve(orders: &[Order]) -> Result<Vec<usize>, Error> {
    let quantities = orders
        .iter()
        .enumerate()
        .map(|(index, order)| {
            let quantity =
                |value: u128| i64::try_from(value).map_err(|_| Error::QuantityTooLarge { index });
            Ok((
                quantity(order.give_quantity)?,
                quantity(order.min_quantity)?,
            ))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    let mut model = CpModelBuilder::default();
    let fill: Vec<_> = orders.iter().map(|_| model.new_bool_var()).collect();
    let kinds: BTreeSet<KindRef> = orders
        .iter()
        .flat_map(|order| [order.give, order.want])
        .collect();
    for kind in kinds {
        let mut balance = LinearExpr::default();
        for ((order, &(give, want)), &filled) in orders.iter().zip(&quantities).zip(&fill) {
            if order.give == kind {
                balance += (give, filled);
            }
            if order.want == kind {
                balance -= (want, filled);
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
    match response.status() {
        CpSolverStatus::Optimal | CpSolverStatus::Feasible => Ok(fill
            .iter()
            .enumerate()
            .filter(|(_, filled)| filled.solution_value(&response))
            .map(|(index, _)| index)
            .collect()),
        status => Err(Error::Status(status)),
    }
}
