#[cfg(feature = "ser")]
#[macro_use]
extern crate serde;

pub use bitset_core::BitSet;
#[doc(hidden)]
pub use itertools::izip;
#[doc(hidden)]
pub use paste::paste;

mod bitset;
mod component_iterator;
mod components;
mod entities;
mod entity_iterator;
mod entity;
mod join;

pub use self::bitset::*;
pub use self::component_iterator::*;
pub use self::components::*;
pub use self::entities::*;
pub use self::entity_iterator::*;
pub use self::entity::*;
pub use self::join::*;
