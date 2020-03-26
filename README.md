[![Build Status](https://travis-ci.org/bast/polygons-rs.svg?branch=master)](https://travis-ci.org/bast/polygons-rs/builds)
[![License](https://img.shields.io/badge/license-%20GPL-blue.svg)](LICENSE)
[![polygons crate](https://img.shields.io/crates/v/polygons.svg)](https://crates.io/crates/polygons)


# polygons-rs

Computes distances to polygon edges and vertices and can check whether points are inside/outside.

Rewriting https://github.com/bast/polygons to Rust - work in progress.

So far not beautiful and not stable. Will improve while learning.


## Running the benchmark

```
$ cargo test --release -- --ignored --nocapture
```
