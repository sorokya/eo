//! # EO
//! This crate will contain most of the data structures and methods needed
//! to create any kind of program for [Endless Online](https://game.eoserv.net/).

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

/// Contains EO primitive types, stream reader, and stream builder
pub mod data;
/// Contains Network related types for EO
pub mod net;
