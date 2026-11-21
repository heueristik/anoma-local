//! The resource machine of the Anoma local circuits, computed outside the circuits: the hashes, resources, kind bases,
//! deltas, the commitment tree and the action digest, each equal to the Noir library and the contract.

pub mod belt;
pub mod delta;
pub mod digest;
mod error;
pub mod field;
pub mod hash;
pub mod kind;
pub mod resource;

pub use ark_bn254::Fr;
pub use ark_grumpkin::{Affine as Point, Fr as Scalar};
pub use error::Error;
