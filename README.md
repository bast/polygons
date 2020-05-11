[![Build status](https://github.com/bast/polygons-rs/workflows/Test/badge.svg)](https://github.com/bast/polygons-rs/actions)
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


## Example

```python
import polygons

# polygon_points is a list of lists
polygon_points = [
    [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.0, 0.0)],
    [(0.0, 2.0), (1.0, 2.0), (1.0, 3.0), (0.0, 3.0), (0.0, 2.0)],
]

points = [(0.5, 0.5), (0.5, -0.5)]

num_edges_children = 4
num_nodes_children = 4
tree = polygons.build_tree(polygon_points, num_edges_children, num_nodes_children)

inside = polygons.points_are_inside(tree, points)
print(inside)  # [True, False]

distances = polygons.distances_nearest_vertices(tree, points)
print(distances)  # [0.7071067811865476, 0.7071067811865476]

distances = polygons.distances_nearest_edges(tree, points)
print(distances)  # [0.5, 0.5]

distances = polygons.distances_nearest_vertices(tree, [(0.6, 0.6), (0.5, -0.5)])
print(distances)  # [0.5656854249492381, 0.7071067811865476]
```
