# boxletters
[![Crates.io](https://img.shields.io/crates/v/boxletters)](https://crates.io/crates/boxletters) 
[![Docs.rs](https://docs.rs/boxletters/badge.svg)](https://docs.rs/boxletters) 
[![Build](https://github.com/Ewpratten/boxletters/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/boxletters/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/boxletters/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/boxletters/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/boxletters/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/boxletters/actions/workflows/audit.yml)


`boxletters` is a commandline tool for converting strings of text to [`regional_indicator`](https://en.wikipedia.org/wiki/Regional_indicator_symbol) emoji.

## Example input

```sh
boxletters hello world
```

## Example output

:regional_indicator_h: :regional_indicator_e: :regional_indicator_l: :regional_indicator_l: :regional_indicator_o:
:regional_indicator_w: :regional_indicator_o: :regional_indicator_r: :regional_indicator_l: :regional_indicator_d:

## Installation

This crate can be installed via `cargo` with:

```sh
cargo install boxletters
```
