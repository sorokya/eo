//! # EO
//! This crate will contain most of the data structures and methods needed
//! to create any kind of program for [Endless Online](https://game.eoserv.net/).

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

#[cfg(feature = "serde")]
extern crate serde;

/// character related data structures
pub mod character;
/// EO primitive types, stream reader, and stream builder
pub mod data;
/// network related data structures
pub mod net;
/// quest related data structures
pub mod quest;
/// world related data structures
pub mod world;
