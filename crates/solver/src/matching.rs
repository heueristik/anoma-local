//! The port to a matcher, which chooses the intents to fill, and the orders and matchings that it takes and gives.

use std::collections::BTreeMap;

use anoma_arm::Fr;

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

/// Chooses orders that balance together: the filled orders give at least the quantities of each kind that they want.
pub trait Matcher {
    type Error: std::error::Error + Send + Sync + 'static;

    fn choose(&self, orders: &[Order]) -> Result<Matching, Self::Error>;
}
