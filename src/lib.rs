//! simple_units: a simple unit system for Rust
//!
//! Written by Willi Kappler, Version 0.1 (2017.02.22)
//!
//! Repository: https://github.com/willi-kappler/simple_units
//!
//! License: MIT
//!
//! Data types and utility function for units:
//! Implement a poor man's unit type system for now...
//!
//! Maybe we can have s.th. like this (F#) for Rust:
//!
//! https://en.wikibooks.org/wiki/F_Sharp_Programming/Units_of_Measure
//!

// For clippy
// #![feature(plugin)]
//
// #![plugin(clippy)]

#![allow(non_upper_case_globals)]
#![allow(dead_code)]

#[macro_use] pub mod macros;
pub mod si_units;
pub mod conversion;
