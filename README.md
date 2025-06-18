# indexing_fmt

![Crates.io Version](https://img.shields.io/crates/v/indexing_fmt?style=flat-square)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/jonaspleyer/indexing_fmt/test.yml?style=flat-square)
[![License MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg?style=flat-square)]()
[![License Apache](https://img.shields.io/badge/License-Apache%202.0-brightgreen.svg?style=flat-square)](https://opensource.org/licenses/Apache-2.0)
[![Docs](https://img.shields.io/docsrs/indexing_fmt?style=flat-square)](https://docs.rs/indexing_fmt)

This crate allows the formatting of integer types as superscripts or subscripts.
It is written in pure safe Rust and `no_std` compatible.

```rust
use indexing_fmt::*;

let index = 12;
let name = format!("Ship{}", index.to_superscript());
assert_eq!(name, "Ship¹²");

let index = 840;
let name = format!("Docking-Bay{}", index.to_subscript());
assert_eq!(name, "Docking-Bay₈₄₀");
```
