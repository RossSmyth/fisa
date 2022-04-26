# FISA
(Ferris Instrument Software Architecture)  
[![GitHub Actions](https://img.shields.io/github/workflow/status/RossSmyth/FISA/CI?style=for-the-badge)](https://github.com/RossSmyth/fisa/blob/main/.github/workflows/main.yml)
[![MPL-2.0](https://img.shields.io/github/license/RossSmyth/FISA?style=for-the-badge)](https://www.mozilla.org/en-US/MPL/2.0/)

FISA is a Rust library for interfacing with IVI-VISA hardware interfaces. The goal is not to make an IVI-compliant shared library, but rather to let Rust code send and receive VISA messages.

In the PyVISA realm of the world, the equivlent of this library would be [PyVISA-py](https://github.com/pyvisa/pyvisa-py), and some of [PyVISA](https://github.com/pyvisa/pyvisa).

FISA is still in a very alpha state, but contributions are always welcome.

## How to use
Sometime in the future add to your `cargo.toml`, but this isn't published yet.

## Goals
* Safety
* Idiomatic Rust
* Speed
* Open-source
* Easy to use
* Well-tested
* Well documented

## Non-goals
* IVI-VISA compliance

## Contribution
Please do. All code at this time will be considered to be contributed under the Mozilla Public License (see below). If it is not, please specify in your PR.

## License
Currently under the Mozilla Public License (MPL). I'm sort of undecided at the moment. But if it changes in the future it will be to more permissive. The MPL allows for seamless integration with any codebase without any licensing worries. The main thing is that the library's source itself (defined by the source files), must be released. [Check the FAQ at Mozilla's page as well](https://www.mozilla.org/en-US/MPL/2.0/FAQ/). If you have issues or questions with this please open an issue sooner than later as I can provide clarification or possibly relicense if the argument is good enough.

The MPL header is not on the source files, but it is distributed under those terms at this time. If I affirm my decision then I will add the headers.

As always if you have serious questions ask your company's legal counsel. If you are individual licensing doesn't effect you much so don't worry about it unless you plan on distribution.
