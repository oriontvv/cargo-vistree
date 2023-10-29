# cargo-vistree
[![Actions Status](https://github.com/oriontvv/cargo-vistree/workflows/CI/badge.svg)](https://github.com/oriontvv/cargo-vistree/actions) [![dependency status](https://deps.rs/repo/github/oriontvv/cargo-vistree/status.svg)](https://deps.rs/repo/github/oriontvv/cargo-vistree) [![Crates.io](https://img.shields.io/crates/v/cargo-vistree.svg)](https://crates.io/crates/cargo-vistree)

[cargo-vistree](https://github.com/oriontvv/cargo-vistree) is a [cargo-tree](https://doc.rust-lang.org/cargo/commands/cargo-tree.html) like cargo extension for graphical visualization of dependecy tree.

## Usage:

```bash
$ cargo vistree -o Cargo.dot && dot -Tsvg Cargo.dot -o Cargo.svg
```

By default `cargo-vistree` uses `pretty` format(can be disabled with `--compact` option).
Beware of `null`s, some formats don't support them (e.g. toml).

## Installation:
```
cargo install cargo-vistree
```
