# MOS-6502
A library providing an emulator of the incredible 
[MOS 6502 processor](https://en.wikipedia.org/wiki/MOS_Technology_6502) in Rust.

## Motivation
Coming from previous interpreter projects, such as the [CHIP-8](https://github.com/R0mbertus/crisps-8), I wanted to dive
deeper into emulating processors. The MOS 6502 seemed like a perfect project as it is a widely used processor (used in
e.g. Commodore 64, Apple II or NES), and as such is also well documented.

## Usage
Building the library should be as simple as running `cargo build --release`. You can then use the library in your own
Rust projects. The tests will run with `cargo test`. Some example usage of reading in files and running them is shown
in the tests at the bottom of the `cpu.rs` file.

## Project Status
The current state of the project provides the functionality needed to pass the 
[6502 functional test](https://github.com/Klaus2m5/6502_65C02_functional_tests) (without decimal mode) and AllSuiteA 
tests, so it should provide the needed functionality for mosts mos 6502 usages.

Or as a short list:
- [x] "Legal" Instructions
- [x] Running loaded files
    - [x] passing 6502 functional and AllSuiteA tests
- [ ] Decimal mode
- [ ] Cycle accuracy
- [ ] "Illegal" Instructions

## Feedback and Contributing
If you encounter any problem(s) using this project or have any feedback to give on the projects code, feel free to leave
an issue or open a pull request. Make sure that any (proposed) changes pass the CI tests!

## Used sources
These are some of the sources used for this project:
- [6502.org](http://www.6502.org/)
- [Ultimate Commodore 64 Reference](https://www.pagetable.com/c64ref/6502/#)
- [NESDEV](https://www.nesdev.org/wiki/6502_instructions)
