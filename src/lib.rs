//! # EO
//! This crate will contain most of the data structures and methods needed
//! to create any kind of program for [Endless Online](https://game.eoserv.net/).

#[cfg(feature = "serde")]
extern crate serde;
/// EO primitive types, stream reader, and stream builder
pub mod data;
/// network related data structures
pub mod net;
/// EO protocol data structs, enumerations, and packets
pub mod protocol;
/// EO pub file data structs, and enumerations
pub mod pubs;