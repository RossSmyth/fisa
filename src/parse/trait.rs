//! Module for address trait and its implementations
use std::str::FromStr;

use super::usb::{UsbAddress, UsbParseError};
use crate::sealed::Sealed;

/// Trait for VISA address types
/// TODO: Decide if this is actually needed or not
pub trait Address: Sized + Sealed {
    /// Errors that can be returned by the parser.
    type Err: std::error::Error;

    /// For parsing a VISA address string in to
    /// an address object
    fn parse(addr_str: &str) -> Result<Self, Self::Err>;
}

impl Sealed for UsbAddress {}
impl Address for UsbAddress {
    type Err = UsbParseError;

    #[inline]
    fn parse(addr_str: &str) -> Result<Self, Self::Err> {
        UsbAddress::from_str(addr_str)
    }
}
