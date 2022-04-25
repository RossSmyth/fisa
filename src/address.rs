//! Module for parsing VISA ressource addresses.
//! See Section 4.3.1.1 on page 77 of [this document](https://www.ivifoundation.org/downloads/Architecture%20Specifications/vpp43_2020-11-20.pdf)
mod usb;

use std::{fmt::Display, str::FromStr};
use thiserror::Error;
use usb::UsbAddress;

fn parse(addr: &str) -> Result<Address, AddressError> {
    use Address::*;

    let address = match addr {
        _ if addr.starts_with("VXI") || addr.starts_with("vxi") => todo!(),
        _ if addr.starts_with("GPIB-VXI") || addr.starts_with("gpic-vxi") => todo!(),
        _ if addr.starts_with("GPIB") || addr.starts_with("gpib") => todo!(),
        _ if addr.starts_with("TCPIP") || addr.starts_with("tcpip") => todo!(),
        _ if addr.starts_with("USB") || addr.starts_with("usb") => Usb(UsbAddress::from_str(addr)?),
        _ if addr.starts_with("PXI") || addr.starts_with("pxi") => todo!(),
        _ => panic!(),
    };
    Ok(address)
}

/// If an error is found in any functions from the address module, this error will be returned.
/// This wraps errors propogated from the specific addresses.
#[derive(Error, Debug)]
pub enum AddressError {
    /// Error parsing an address identified as a USB resource.
    #[error(transparent)]
    UsbError(#[from] usb::UsbParseError),
}

/// Represents a VISA address.
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Address {
    /// Representing a USB address.
    Usb(UsbAddress),
}

impl Address {
    /// Constructs new Address object from an address.
    /// Panics on failure.
    /// Note: Just because parsed does __not__ mean the resource exists.
    pub fn new(address: &str) -> Address {
        Address::try_new(address).unwrap()
    }
    /// Constructs new Address object from an address.
    /// Returns a Result.
    /// Note: Just because parsed does __not__ mean the resource exists.
    pub fn try_new(address: &str) -> Result<Address, AddressError> {
        parse(address)
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Address::Usb(addr) => addr.fmt(f),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Macro for throwing addresses and seeing what stick.
    /// There is an optional "false" literal at the end.
    /// If provided the test will be ignored.
    /// ($test_name, address_literal)
    #[macro_export]
    macro_rules! test_address {
        ($name:ident, $addr:literal) => {
            #[test]
            fn $name() {
                let addr = Address::new($addr);
                assert_eq!(addr.to_string(), $addr);
            }
        };
        (#[ignore], $name:ident, $addr:literal) => {
            #[test]
            #[ignore]
            fn $name() {
                panic!();
            }
        };
    }

    // All taken from Table 4.3.2 in
    // https://www.ivifoundation.org/downloads/Architecture%20Specifications/vpp43_2020-11-20.pdf

    // Primary interface that needs to work.
    test_address!(#[ignore], test_tcpip_raw,              "TCPIP0::1.2.3.4::5025::SOCKET");
    test_address!(#[ignore], test_tcpip_address,          "TCPIP::devicename.company.com::INSTR");
    test_address!(#[ignore], test_tcpip_raw_vxi,          "TCPIP::1.2.3.4::inst0::INSTR");
    test_address!(#[ignore], test_tcpip_ipv6_hislip,      "TCPIP::[fe80::1]::hislip0::INSTR");
    test_address!(#[ignore], test_tcpip_ipv6_secure,      "TCPIP::@[fe80::1]::hislip0::INSTR");
    test_address!(#[ignore], test_tcpip_ipv6_credentials, "TCPIP::@[fe80::1]::hislip0::INSTR");
    test_address!(#[ignore], test_tcpip_ipv6_port_cred,   "TCPIP::SecureCreds@[fe80::1]::5025::SOCKET");
    test_address!(#[ignore], test_tcpip_visa_login,       "TCPIP::$$john:Hoopla%212@1.2.3.4::hislip0::INSTR");

    // PRobably feature gated.
    test_address!(usb_test, "USB34::0x1234::0x5678::A22-5::12314::INSTR");
    test_address!(#[ignore], test_serial,                 "ASRL1::INSTR");

    // Maybe get working. Would need to be feature gated and have some GPIB bindings created.
    test_address!(#[ignore], test_gpib_sec,               "GPIB::1::0::INSTR");
    test_address!(#[ignore], test_gpib_servant,           "GPIB1::SERVANT");

    // Either not sure how to interface with these, or what they are.
    // Deprioritized.
    test_address!(#[ignore], test_pxi,                    "PXI0::3-18::INSTR");
    test_address!(#[ignore], test_pxi_function,           "PXI0::3-18.2::INSTR");
    test_address!(#[ignore], test_pxi_bus,                "PXI0::21::INSTR");
    test_address!(#[ignore], test_pxi_slow,               "PXI0::CHASSIS1::SLOT4::INSTR");
    test_address!(#[ignore], test_pxi_endpoint,           "PXI0::CHASSIS1::SLOT4INDEX1::INSTR");
    test_address!(#[ignore], test_pxi_memcont,            "PXI0::MEMACC");
    test_address!(#[ignore], test_pxi_mainframe,          "PXI0::1::BACKPLANE");

    test_address!(#[ignore], test_vxi,                    "VXI0::1::INSTR");
    test_address!(#[ignore], test_vxi_board,              "VXI::MEMACC");
    test_address!(#[ignore], test_vxi_chassis,            "VXI::1::BACKPLANE");
    test_address!(#[ignore], test_vxi_servant,            "VXI0::SERVANT");

    test_address!(#[ignore], test_gpib_vxi,               "GPIB-VXI::9::INSTR ");
    test_address!(#[ignore], test_gpic_vxi_board,         "GPIB-VXI1::MEMACC");
    test_address!(#[ignore], test_gpib_vxi_chassis,       "GPIB-VXI2::BACKPLANE");
}
