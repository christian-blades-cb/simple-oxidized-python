# Simple Oxidized Python

This is an example of a Rust-powered python module using milksnake. It's very contrived.

## How to use this repo

This is a reference (and scaffold for you copy-paste afficiandos). 

### Project layout:

* `/simple-rs` - The rust code you want to turn into a module
* `/cabi` - C dynamic library from Rust
* `/cabi/build.rs` - cbindgen generates a header file (`simple.h`) which we'll use for our python wheel.
* `/py` - our python library
* `/py/setup.py` - this is where milksnake does its magic
* `/py/simple/__init__.py` - the pythonic interface

### Pre-reqs

* Rust [rustup](https://rustup.rs)
* Python [python.org](https://www.python.org/downloads)

### Building the module

`cd py; python setup.py bdist_wheel`

`simple-0.1.0-py2.py3-none-linux_x86_64.whl` now lives in `py/dist`

## Useful reading

* cbindgen - generate a C header file from Rust - https://crates.io/crates/cbindgen
* milksnake - Rust to Python module - https://github.com/getsentry/milksnake
