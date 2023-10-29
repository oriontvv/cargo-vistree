# cargo-vistree
[![Actions Status](https://github.com/oriontvv/cargo-vistree/workflows/CI/badge.svg)](https://github.com/oriontvv/cargo-vistree/actions) [![dependency status](https://deps.rs/repo/github/oriontvv/cargo-vistree/status.svg)](https://deps.rs/repo/github/oriontvv/cargo-vistree) [![Crates.io](https://img.shields.io/crates/v/cargo-vistree.svg)](https://crates.io/crates/cargo-vistree)

[cargo-vistree](https://github.com/oriontvv/cargo-vistree) is a [cargo-tree](https://doc.rust-lang.org/cargo/commands/cargo-tree.html) like cargo extension for graphical visualization of dependecy tree.

## Usage:

```bash
$ cargo vistree -o Cargo.dot && dot -Tsvg Cargo.dot -o Cargo.svg
```


![Output of `cargo modules generate tree …`](Cargo.svg)

## Installation:
```
cargo install cargo-vistree
```

Install graphviz:
```
$ brew install graphviz
OR
$ sudo apt -y install graphviz
```


### KEEP YOUR DEPENDENCIES FRESH AND CLEAN!
I would like to mention here awesome utils:
* [machete](https://crates.io/crates/cargo-machete) [remove unused deps: fast analyze source code]
* [udeps](https://crates.io/crates/cargo-udeps) [remove unused deps: more accurately, slower]
* [dependabot](https://github.com/dependabot) [update deps]
* [deny](https://crates.io/crates/cargo-deny) [linter for deps]