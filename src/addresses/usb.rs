//! Module for USB VISA addresses.
//! Includes primarily the main struct and the errors.
use std::{
    fmt::{Display, Write},
    num::ParseIntError,
    str::FromStr,
};

use thiserror::Error;

/// Represents a USB VISA address
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct UsbAddress {
    /// Not exactly sure
    board: Option<u32>,
    /// The USB manufacturer ID. Always hex in UI.
    manufactuer_id: u16,
    /// The USB model code. Always hex in the UI.
    model_code: u16,
    /// Serial number. Not actually a number, but a string. For UI purposes only and not analyzed.
    serial_number: String,
    /// Optional interface number. If None, then lowest number that matche is used.
    interface_number: Option<u16>,
    /// USB INSTR lets the controller interact with the device associated with the resource.
    instr: bool,
}

impl UsbAddress {
    /// Creates a new UsbAddress from an address.
    /// Panics on failure. See Self::try_new for a Result
    /// > **Note:** Just because parsed does __not__ mean the resource exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fisa::addresses::usb::UsbAddress;
    /// let addr = "USB::0x1A34::0x5678::A22-5";
    /// assert_eq!(UsbAddress::new(addr).to_string(), addr);
    /// ```
    ///
    /// ```should_panic
    /// # use fisa::addresses::usb::UsbAddress;
    /// let addr = "USB::";
    /// UsbAddress::new(addr);
    /// ```
    pub fn new(addr: &str) -> UsbAddress {
        UsbAddress::from_str(addr).unwrap()
    }

    /// Failably creates a new UsbAddress from an address.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fisa::addresses::usb::{UsbAddress, UsbParseError};
    /// let addr = "USB::0x1A34::0x5678::A22-5";
    /// assert_eq!(UsbAddress::try_new(addr)?.to_string(), addr);
    /// # Ok::<(), UsbParseError>(())
    /// ```
    ///
    /// ```
    /// # use fisa::addresses::usb::{UsbAddress, UsbParseError};
    /// let addr = "USB::";
    /// assert!(UsbAddress::try_new(addr).is_err());
    /// ```
    pub fn try_new(addr: &str) -> Result<Self, UsbParseError> {
        UsbAddress::from_str(addr)
    }
}

/// Errors that can return from USB address parsing.
#[derive(Error, Debug)]
pub enum UsbParseError {
    /// When the given address does not have the USB prefix.
    #[error("Expected \"USB\" at address start, found {0:?}")]
    NotUSB(String),

    /// When parsing an integer fails.
    #[error("Found {found:?} instead of a number at position {start:?} to {end:?} of \n{addr:?}")]
    NumParseError {
        /// What was found instead of a number upon detecting an error.
        found: String,
        /// The full invalid address.
        addr: String,
        /// Start index of the address that contains the invalid integer.
        start: usize,
        /// Final index of the address that contains the invalid integer.
        end: usize,
        /// The original error returned.
        #[source]
        source: ParseIntError,
    },

    /// When a field that is supposed to be hexidecimal is not properly formatted.
    #[error("Invalid hexidecimal number: {found:?} at position {start:?} to {end:?} in\n {addr:?}\nNumber must start with '0x'")]
    NotHex {
        /// What was found instead if "0x"
        found: String,
        /// The address containing the invalid hex
        addr: String,
        /// Start of the span that was parsed before the error was detected.
        start: usize,
        /// End of the span that was parsed before the error was detected.
        end: usize,
    },

    /// When an address is detected to not be complete.
    #[error("{0:?} is an incomplete address missing: {1}")]
    IncompleteAddress(String, String),

    /// When an address indicates that is has an "INSTR" suffix, but is malformed.
    #[error("In address \"INSTR\" was indicated but instead {found:?} was found at {start:?} to {end:?} of\n {addr:?}")]
    NotInstr {
        /// What was found instead of "INSTR"
        found: String,
        /// The full invalid address
        addr: String,
        /// Start of the span containing the invalid "INSTR"
        start: usize,
        /// End fo the span containing the invalid "INSTR"
        end: usize,
    },

    /// When the end of a token in the address is detect but is malformed.
    #[error("Double colons must seperate address portions. Found {found:?} in:\n {addr:?}.")]
    InvalidSeperator {
        /// What was found instead of "::"
        found: String,
        /// The full invalid address
        addr: String,
        /// Start of the span containing the invalid "::"
        start: usize,
        /// End fo the span containing the invalid "::"
        end: usize,
    },
}

/// State of the USB address parser state-machine
enum UsbParserState {
    /// Required, the initial state
    Usb,

    /// Optional, always transition to second.
    Board,

    /// Required, always transition to third.
    ManufactuerId,

    /// Required, always transition to fourth.
    ModelCode,

    /// Required, always transition to fifth.
    SerialNumber,

    /// Optional, may trasition to sixth or never be transitioned to is address ends.
    USBInterface,

    /// Optional, may transition to sixth, seventh, of never.
    Instr,
}

impl FromStr for UsbAddress {
    type Err = UsbParseError;

    fn from_str(address: &str) -> Result<Self, Self::Err> {
        use UsbParseError::*;
        use UsbParserState::*;

        let mut addr_iter = address.char_indices().peekable();

        // Scratch buffer for parsing.
        let mut buffer = String::with_capacity(10);
        let mut span = 0..0; // Parsing span. Primarily for errors.

        // I do not like defaults, so I will not implement the Default trait.
        // but this is an invalid value to build upon.
        let mut ret = Ok(UsbAddress {
            board: None,
            manufactuer_id: 0,
            model_code: 0,
            serial_number: String::new(),
            interface_number: None,
            instr: false,
        });
        let mut parser_state = Usb; // WOOO FSM

        // Checking for errors first before advancing the iterator is intentional.
        // Using the if/else ensures that lifetime analysis is happy. If the iterator
        // is advanced with the while let, then after it ends we still need to check
        // what we ended on. Then for error creation the scratch buffer must be moved.
        // But since we are out of the while loop, lifetime analysis sees that the buffer
        // moved while in the loop. So it makes a frowny face.
        while let Ok(resource) = &mut ret {
            if let Some((addr_index, addr_char)) = addr_iter.next() {
                // Span of the section of the address currently being parsed.
                span.end = addr_index;

                match (&parser_state, addr_char) {
                    (Usb, 'U') if addr_index == 0 => {
                        // USB[board]::manufacturer ID::model code::serial number[::USB interfacenumber][::INSTR]
                        // ↑
                        // You are here
                        continue;
                    }
                    (Usb, 'S') if addr_index == 1 => {
                        // USB[board]::manufacturer ID::model code::serial number[::USB interfacenumber][::INSTR]
                        //  ↑
                        // You are here
                        continue;
                    }
                    (Usb, 'B') if addr_index == 2 => {
                        // USB[board]::manufacturer ID::model code::serial number[::USB interfacenumber][::INSTR]
                        //   ↑
                        // You are here
                        span.start = addr_index + 1;
                        buffer.clear();

                        parser_state = Board;
                        continue;
                    }
                    (Usb, _) => {
                        // USB[board]::manufacturer ID::model code::serial number[::USB interfacenumber][::INSTR]
                        // ???
                        // You are here (Error)

                        ret = Err(NotUSB(address[0..3].to_string()));
                        break;
                    }
                    (ManufactuerId, char)
                    | (ModelCode, char)
                    | (SerialNumber, char)
                    | (Instr, char)
                        if span.start > span.end =>
                    {
                        // USB[board]::manufacturer ID::model code::serial number[::USB interfacenumber][::INSTR]
                        //            ↑       OR       ↑    OR     ↑      OR       ↑         OR           ↑
                        // You are here

                        // Since the span of the slice of the address to be analyzed is set
                        // to be two ahead when the first colon is encounterd, this ensures
                        // checks to see if the second colon exists.
                        if char == ':' {
                            continue;
                        } else {
                            ret = Err(InvalidSeperator {
                                found: format!(":{char}"),
                                addr: address.to_string(),
                                start: span.end - 1,
                                end: span.end,
                            })
                        }
                    }
                    // Careful! is_empty is true for the above case as well!
                    (Board, ':') if span.is_empty() => {
                        // USB::manufacturer ID::model code::serial number[::USB interfacenumber][::INSTR]
                        //    ↑
                        // You are here (no board)
                        resource.board = None;

                        span.start = addr_index + 2;
                        buffer.clear();

                        parser_state = ManufactuerId;
                        continue;
                    }
                    (Board, ':') => {
                        // USB[board]::manufacturer ID::model code::serial number[::USB interfacenumber][::INSTR]
                        //           ↑
                        // You are here

                        match buffer.parse() {
                            Ok(board_num) => {
                                resource.board = Some(board_num);

                                span.start = addr_index + 2;
                                buffer.clear();

                                parser_state = ManufactuerId;
                                continue;
                            }
                            Err(err) => {
                                ret = Err(NumParseError {
                                    found: buffer,
                                    addr: address.to_string(),
                                    start: span.start,
                                    end: span.end - 1,
                                    source: err,
                                });
                                break;
                            }
                        }
                    }
                    (ManufactuerId, ':') | (ModelCode, ':') => {
                        // USB[board]::manufacturer ID::model code::serial number[::USB interfacenumber][::INSTR]
                        //                            ↑     OR    ↑
                        // You are here

                        // Parses hex number
                        match u16::from_str_radix(buffer.as_str(), 16) {
                            Ok(code) => {
                                // Advanced to where the start of the modelcode or serialnumber will be.
                                span.start = addr_index + 2;
                                buffer.clear();

                                parser_state = match parser_state {
                                    ManufactuerId => {
                                        resource.manufactuer_id = code;
                                        ModelCode
                                    }
                                    ModelCode => {
                                        resource.model_code = code;
                                        SerialNumber
                                    }
                                    _ => unreachable!(),
                                };

                                continue;
                            }
                            Err(err) => {
                                ret = Err(NumParseError {
                                    found: buffer,
                                    addr: address.to_string(),
                                    start: span.start,
                                    end: span.end - 1,
                                    source: err,
                                });
                                break;
                            }
                        }
                    }
                    (ManufactuerId, char) | (ModelCode, char) if span.is_empty() => {
                        if char == '0' {
                            // USB[board]::0x<CODE>::0x<CODE>::serial number[::USB interfacenumber][::INSTR]
                            //             ↑    OR   ↑
                            // You are here

                            // Validates that this is a hex format
                            continue;
                        } else {
                            buffer.push(char);

                            ret = Err(NotHex {
                                found: 'scanning0: loop {
                                    if let Some((index, char)) = addr_iter.next() {
                                        span.end = index;
                                        if char == ':' {
                                            break buffer;
                                        } else {
                                            buffer.push(char);
                                        }
                                    } else {
                                        break 'scanning0 buffer;
                                    }
                                },
                                addr: address.to_string(),
                                start: span.start,
                                end: span.end,
                            });
                            break;
                        }
                    }
                    (ManufactuerId, char) | (ModelCode, char) if span.len() == 1 => {
                        // USB[board]::0x<CODE>::0x<CODE>::serial number[::USB interfacenumber][::INSTR]
                        //              ↑    OR   ↑
                        // You are here

                        if char == 'x' || char == 'X' {
                            continue;
                        } else {
                            buffer.push(char);

                            ret = Err(NotHex {
                                found: 'scanningX: loop {
                                    if let Some((index, char)) = addr_iter.next() {
                                        span.end = index;
                                        if char == ':' {
                                            break buffer;
                                        } else {
                                            buffer.push(char);
                                        }
                                    } else {
                                        break 'scanningX buffer;
                                    }
                                },
                                addr: address.to_string(),
                                start: span.start,
                                end: span.end,
                            });
                            break;
                        }
                    }
                    (SerialNumber, ':') => {
                        // USB[board]::0x<CODE>::0x<CODE>::serial number[::USB interfacenumber][::INSTR]
                        //                                               ↑          OR          ↑
                        // You are here

                        // Intersting thought. Is it valid for a serial number to have a colon? See fisa#7
                        resource.serial_number.clone_from(&buffer);
                        buffer.clear();

                        span.start = addr_index + 2;

                        // There are two distinct optional fields next
                        match addr_iter.next() {
                            Some((i, ':')) => {
                                parser_state = match addr_iter.peek() {
                                    Some((_, 'I')) | Some((_, 'i')) => Instr,
                                    _ => USBInterface,
                                };
                                span.end = i + 1;
                            }
                            Some((i, char)) => {
                                ret = Err(InvalidSeperator {
                                    found: format!(":{char}"),
                                    addr: address.to_string(),
                                    start: i - 1,
                                    end: i,
                                })
                            }
                            None => {
                                // Means there was one but not a second colon.
                                ret = Err(IncompleteAddress(
                                    address.to_string(),
                                    "either USB Interface or INSTR".to_string(),
                                ))
                            }
                        }
                        continue;
                    }
                    (USBInterface, ':') => {
                        // USB[board]::0x<CODE>::0x<CODE>::serial number[::USB interfacenumber][::INSTR]
                        //                                                                      ↑
                        // You are here

                        match buffer.parse() {
                            Ok(num) => {
                                resource.interface_number = Some(num);
                                buffer.clear();

                                span.start = addr_index + 2;
                                parser_state = Instr;
                                continue;
                            }
                            Err(err) => {
                                ret = Err(NumParseError {
                                    found: buffer,
                                    addr: address.to_string(),
                                    start: span.start,
                                    end: span.end - 1,
                                    source: err,
                                });
                                break;
                            }
                        }
                    }
                    (Board, char)
                    | (ManufactuerId, char)
                    | (ModelCode, char)
                    | (SerialNumber, char)
                    | (USBInterface, char)
                    | (Instr, char) => {
                        // USB[board]::0x<CODE>::0x<CODE>::serial number[::USB interfacenumber][::INSTR]
                        //    ↑-----↑ OR  ↑---↑ OR ↑----↑OR↑-----------↑ OR↑------------------↑
                        // You are here

                        buffer.push(char);
                        continue;
                    }
                }
            } else {
                // When it's the end of the str
                // Using the if/else ensures that lifetime analysis is happy
                match parser_state {
                    Usb => {
                        ret = Err(IncompleteAddress(
                            address.to_string(),
                            "USB flag, Manufacture Code, Model Number, Serial number".to_string(),
                        ))
                    }
                    Board | ManufactuerId => {
                        ret = Err(IncompleteAddress(
                            address.to_string(),
                            "Manufacture Code, Model Number, Serial number".to_string(),
                        ))
                    }
                    ModelCode => {
                        ret = Err(IncompleteAddress(
                            address.to_string(),
                            "Model Number, Serial number".to_string(),
                        ))
                    }
                    SerialNumber => {
                        // USB[board]::manufacturer ID::model code::serial number
                        //                                                       ↑
                        // You are here

                        // I do not know what the proper format of a serial number is.
                        // So I'll just accept anything that is not an empty string.
                        match buffer.as_str() {
                            "" => {
                                ret = Err(IncompleteAddress(address.into(), "Serial Number".into()))
                            }
                            _ => resource.serial_number = buffer,
                        }
                    }
                    USBInterface => {
                        // USB[board]::manufacturer ID::model code::serial number::USB interfacenumber
                        //                                                                            ↑
                        // You are here

                        match buffer.parse() {
                            Ok(num) => resource.interface_number = Some(num),
                            Err(err) => {
                                ret = Err(NumParseError {
                                    found: buffer,
                                    addr: address.to_string(),
                                    start: span.start,
                                    end: span.end - 1,
                                    source: err,
                                });
                                break;
                            }
                        }
                    }
                    Instr => {
                        // USB[board]::manufacturer ID::model code::serial number::USB interfacenumber::INSTR
                        //                                                                                   ↑
                        // You are here

                        let buff_upper = buffer.to_uppercase();

                        if buff_upper == "INSTR" {
                            resource.instr = true;
                        } else {
                            ret = Err(NotInstr {
                                found: buffer,
                                addr: address.to_string(),
                                start: span.start,
                                end: span.end - 1,
                            })
                        }
                    }
                }
                break;
            }
        }
        ret
    }
}

impl Display for UsbAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Reference:
        // USB[board]::manufacturer ID::model code::serial number[::USB interfacenumber][::INSTR]

        let mut board_str = String::with_capacity(2);
        let mut interface_str = String::with_capacity(5);
        let mut instr_str = String::with_capacity(5);

        if let Some(num) = self.board {
            write!(board_str, "{}", num)?
        }
        if let Some(num) = self.interface_number {
            write!(interface_str, "::{}", num)?
        }
        if self.instr {
            instr_str.write_str("::INSTR")?
        }

        write!(
            f,
            "USB{}::{:#X}::{:#X}::{}{}{}",
            board_str,
            self.manufactuer_id,
            self.model_code,
            self.serial_number,
            interface_str,
            instr_str
        )
    }
}

#[cfg(test)]
mod test {
    //! Different permutations of USB addresses to parse.
    use super::*;

    /// Helper macro
    /// test_parse!(function_identifier, address_to_parse);
    macro_rules! test_parse {
        ($name:ident, $addr:literal) => {
            #[test]
            fn $name() -> Result<(), UsbParseError> {
                const ADDR: &str = $addr;
                match UsbAddress::from_str(ADDR) {
                    Ok(address) => {
                        assert_eq!(address.to_string(), ADDR);
                        Ok(())
                    }
                    Err(err) => Err(err),
                }
            }
        };
    }

    test_parse!(usb_parse_address, "USB::0x1A34::0x5678::A22-5");
    test_parse!(usb_parse_board, "USB1::0x12B4::0x56F8::A22-5::INSTR");
    test_parse!(usb_parse_instr, "USB::0xFFA1::0x56C8::A22-5::INSTR");
    test_parse!(usb_parse_interface, "USB::0x1234::0x5D78::A22-5::123");
    test_parse!(usb_parse_all, "USB34::0x12A4::0xFF1A::A22-5::12314::INSTR");

    mod ui {
        //! USB Address UI tests.
        use super::*;

        /// Helper macro
        /// test_ui!(function_identifier, address_to_parse, expected_error);
        macro_rules! test_ui {
            ($name:ident, $addr:literal, $expected:literal) => {
                #[test]
                fn $name() -> Result<(), String> {
                    const ADDR: &str = $addr;
                    const EXPECT: &str = $expected;
                    if let Err(err) = UsbAddress::from_str(ADDR) {
                        if err.to_string() == EXPECT {
                            Ok(())
                        } else {
                            Err(format!("Incorrect error returned:\n {err}"))
                        }
                    } else {
                        Err(format!("Accepted invalid USB address: {ADDR}").into())
                    }
                }
            };
        }

        test_ui!(
            usb_ui_not_usb,
            "TCPIP::1.2.3.4::inst0::INSTR",
            "Expected \"USB\" at address start, found \"TCP\""
        );
        test_ui!(usb_ui_cut_usb, "US", "\"US\" is an incomplete address missing: USB flag, Manufacture Code, Model Number, Serial number");
        test_ui!(usb_ui_cut_manu, "USB::0x", "\"USB::0x\" is an incomplete address missing: Manufacture Code, Model Number, Serial number");
        test_ui!(
            usb_ui_cut_model,
            "USB::0x321::0x1",
            "\"USB::0x321::0x1\" is an incomplete address missing: Model Number, Serial number"
        );
        test_ui!(
            usb_ui_cut_serial,
            "USB::0x321::0x132::",
            "\"USB::0x321::0x132::\" is an incomplete address missing: Serial Number"
        );
        test_ui!(usb_ui_manu_hex, "USB34::x1H34::0x5678::A22-5::12314::INSTR", "Invalid hexidecimal number: \"x1H34\" at position 7 to 12 in\n \"USB34::x1H34::0x5678::A22-5::12314::INSTR\"\nNumber must start with '0x'");
        test_ui!(usb_ui_model_hex, "USB34::0x1B34::x56A8::A22-5::12314::INSTR", "Invalid hexidecimal number: \"x56A8\" at position 15 to 20 in\n \"USB34::0x1B34::x56A8::A22-5::12314::INSTR\"\nNumber must start with '0x'");
        test_ui!(usb_ui_wrong_inst_long, "USB34::0x12C4::0x5678::A22-5::12314::INSTRfdss", "In address \"INSTR\" was indicated but instead \"INSTRfdss\" was found at 37 to 44 of\n \"USB34::0x12C4::0x5678::A22-5::12314::INSTRfdss\"");
        test_ui!(usb_ui_wrong_inst_short, "USB34::0x1234::0x5D78::A22-5::INST", "In address \"INSTR\" was indicated but instead \"INST\" was found at 30 to 32 of\n \"USB34::0x1234::0x5D78::A22-5::INST\"");
        test_ui!(usb_ui_num_err_model, "USB34::0x1234::0x56Z8::A22-5::12314::INSTR", "Found \"56Z8\" instead of a number at position 15 to 20 of \n\"USB34::0x1234::0x56Z8::A22-5::12314::INSTR\"");
        test_ui!(usb_ui_num_err_manu, "USB34::0xTEST::0x568::A22-5::12314::INSTR", "Found \"TEST\" instead of a number at position 7 to 12 of \n\"USB34::0xTEST::0x568::A22-5::12314::INSTR\"");
        test_ui!(usb_ui_colon, "USB:0x1A34::0x5678::A22-5", "Double colons must seperate address portions. Found \":0\" in:\n \"USB:0x1A34::0x5678::A22-5\".");
        test_ui!(usb_ui_board_colon, "USB1:0x1A34::0x5678::A22-5", "Double colons must seperate address portions. Found \":0\" in:\n \"USB1:0x1A34::0x5678::A22-5\".");
        test_ui!(usb_ui_manu_colon, "USB1::0x1A34:0x5678::A22-5", "Double colons must seperate address portions. Found \":0\" in:\n \"USB1::0x1A34:0x5678::A22-5\".");
        test_ui!(usb_ui_model_colon, "USB1::0x1A34::0x5678:A22-5", "Double colons must seperate address portions. Found \":A\" in:\n \"USB1::0x1A34::0x5678:A22-5\".");
        test_ui!(usb_ui_serial_colon, "USB1::0x1A34::0x5678::A22-5:01", "Double colons must seperate address portions. Found \":0\" in:\n \"USB1::0x1A34::0x5678::A22-5:01\".");
        test_ui!(usb_ui_instr_colon, "USB1::0x1A34::0x5678::A22-5::01:INSTR", "Double colons must seperate address portions. Found \":I\" in:\n \"USB1::0x1A34::0x5678::A22-5::01:INSTR\".");
        test_ui!(usb_ui_instr_colon2, "USB1::0x1A34::0x5678::A22-5:INSTR", "Double colons must seperate address portions. Found \":I\" in:\n \"USB1::0x1A34::0x5678::A22-5:INSTR\".");
    }
}
