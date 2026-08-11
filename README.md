# forj

[![Crates.io Version](https://img.shields.io/crates/v/forj)](https://crates.io/crates/forj)
[![Crates.io License](https://img.shields.io/crates/l/forj)](https://crates.io/crates/forj)


A suite of tools for interacting with (System)Verilog hardware designs,
fully compliant with [1800-2023](https://ieeexplore.ieee.org/document/10458102)

This project is currently under initial development - stay tuned!

## Packages

Forj is composed of many children projects to separate complexity and use cases

### `forj-parser`

[![Crates.io Version](https://img.shields.io/crates/v/forj-parser)](https://crates.io/crates/forj-parser)
[![docs.rs](https://img.shields.io/docsrs/forj-parser)](https://docs.rs/crate/forj-parser/latest)
[![Crates.io License](https://img.shields.io/crates/l/forj-parser)](https://crates.io/crates/forj-parser)

A complete preprocessor and parser for SystemVerilog source text, forming an CST as defined by `forj-syntax`

### `forj-python`

[![Crates.io Version](https://img.shields.io/crates/v/forj-python)](https://crates.io/crates/forj-python)
[![PyPI Version](https://img.shields.io/pypi/v/forj_python)](https://pypi.org/project/forj_python)
[![ReadTheDocs](https://img.shields.io/readthedocs/forj-python)](https://forj-python.readthedocs.io/en/latest/?badge=latest)
[![Crates.io License](https://img.shields.io/crates/l/forj-python)](https://crates.io/crates/forj-python)

Python bindings for the `forj` SystemVerilog tools

### `forj-syntax`

An object definition of a SystemVerilog CST

[![Crates.io Version](https://img.shields.io/crates/v/forj-syntax)](https://crates.io/crates/forj-syntax)
[![docs.rs](https://img.shields.io/docsrs/forj-syntax)](https://docs.rs/crate/forj-syntax/latest)
[![Crates.io License](https://img.shields.io/crates/l/forj-syntax)](https://crates.io/crates/forj-syntax)