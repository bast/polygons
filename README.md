[![Build Status](https://travis-ci.org/bast/polygons-rs.svg?branch=master)](https://travis-ci.org/bast/polygons-rs/builds)
![](https://github.com/bast/polygons-rs/workflows/Test/badge.svg)
[![License](https://img.shields.io/badge/license-%20GPL-blue.svg)](LICENSE)
[![polygons crate](https://img.shields.io/crates/v/polygons.svg)](https://crates.io/crates/polygons)


# polygons-rs

Computes distances to polygon edges and vertices and can check whether points
are inside/outside polygons.

Rewriting https://github.com/bast/polygons to Rust - work in progress.

So far not beautiful and not stable. Will improve while learning.


## Running the benchmark

```
$ cargo test --release -- --ignored --nocapture
```


## Python interface

Inspired by https://github.com/dev-cafe/rustafarian.

```
$ cargo build --release --features pyo3
$ maturin develop --release --cargo-extra-args="--features pyo3"
```
