//! Module for parsing VISA ressource addresses.
//! See Section 4.3.1.1 on page 77 of [this document](https://www.ivifoundation.org/downloads/Architecture%20Specifications/vpp43_2020-11-20.pdf)
//!
//! All addresses do not rely upon or store the string provided, and they are able to create the address just from the information within them.
pub mod usb;

mod r#trait;
pub use r#trait::Address;

// All taken from Table 4.3.2 in
// https://www.ivifoundation.org/downloads/Architecture%20Specifications/vpp43_2020-11-20.pdf

// Primary interface that needs to work.
/*
test_address!(#[ignore], test_tcpip_raw,              "TCPIP0::1.2.3.4::5025::SOCKET");
test_address!(#[ignore], test_tcpip_address,          "TCPIP::devicename.company.com::INSTR");
test_address!(#[ignore], test_tcpip_raw_vxi,          "TCPIP::1.2.3.4::inst0::INSTR");
test_address!(#[ignore], test_tcpip_ipv6_hislip,      "TCPIP::[fe80::1]::hislip0::INSTR");
test_address!(#[ignore], test_tcpip_ipv6_secure,      "TCPIP::@[fe80::1]::hislip0::INSTR");
test_address!(#[ignore], test_tcpip_ipv6_credentials, "TCPIP::@[fe80::1]::hislip0::INSTR");
test_address!(#[ignore], test_tcpip_ipv6_port_cred,   "TCPIP::SecureCreds@[fe80::1]::5025::SOCKET");
test_address!(#[ignore], test_tcpip_visa_login,       "TCPIP::$$john:Hoopla%212@1.2.3.4::hislip0::INSTR");
*/

// Maybe get working. Would need to be feature gated and have some GPIB bindings created.
/*
test_address!(#[ignore], test_gpib_sec,               "GPIB::1::0::INSTR");
test_address!(#[ignore], test_gpib_servant,           "GPIB1::SERVANT");
*/

// Either not sure how to interface with these, or what they are.
// Deprioritized.
/*
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
*/
