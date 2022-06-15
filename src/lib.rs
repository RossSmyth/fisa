#![deny(
    unsafe_code,
    missing_docs,
    missing_debug_implementations,
    dead_code,
    clippy::missing_docs_in_private_items
)]
#![warn(
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::print_stdout,
    clippy::cast_sign_loss
)]
#![doc = include_str!("../README.md")]

pub mod parse;

mod sealed {
    //! Just for sealing traits so no one can be sneaky

    /// How 2 seal traits in one easy step
    pub trait Sealed {}
}
