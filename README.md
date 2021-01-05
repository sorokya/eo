![Rust](https://github.com/sorokya/eo/workflows/Rust/badge.svg)

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
* [ ] Server pub file reading+writing (Need to finish Skill masters and talk files)
