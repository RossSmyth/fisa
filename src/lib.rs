#![deny(unsafe_code)]
#![warn(missing_docs)]

//! Library for interacting with VISA instruments.
//! Goals:
//! * Interact with instruments with idiomatic Rust code
//! * Safety.
//!
//! Creating a full IVI standard compliant VISA shared library is possible,
//! that would have to be much further down the line.
pub mod addresses;
