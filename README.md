[![Build Status][actions-badge]][actions-url]
[![Crate][crates-badge]][crates-url]
[![Docs][docs-badge]][docs-url]
[![License][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/eo.svg
[crates-url]: https://crates.io/crates/tokio
[docs-badge]: https://img.shields.io/docsrs/eo.svg
[docs-url]: https://docs.rs/eo
[actions-badge]: https://github.com/sorokya/eo/workflows/Rust/badge.svg
[actions-url]: https://github.com/sorokya/eo/actions?query=branch%3Amaster
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/sorokya/eo/blob/master/LICENSE

# Archived

Please use the new [eolib](https://github.com/sorokya/eolib-rs) crate

# EO
This crate will contain most of the data structures and methods needed
to create any kind of program for [Endless Online](https://game.eoserv.net/).

## Modules
* *character* - enums for character specific values
* *data* - eo primitive types, stream builder and stream writer.
    * *pubs* - structs, and enums for reading+writing eo pub data files.
* *net* - packet processing functions, and enums for network related reply codes
* *quest* - enums for quest specific values
* *world* - enums that are shared between different objects in the game world

## Features
* [x] Number encoding
* [x] Packet encoding
* [x] EO StreamBuilder+StreamReader provides a tool for working with EO Byte arrays
* [ ] Client pub file reading+writing (Reading works)
* [ ] Server pub file reading+writing (Reading works)
* [ ] Network protocol data structures & serialization (Building these out as I go with the [reoserv](https://github.com/sorokya/reoserv) project)
  * [ ] Client request packets
  * [ ] Server response packets
