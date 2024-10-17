[![test status](https://github.com/bast/polygons/workflows/Test/badge.svg)](https://github.com/bast/polygons/actions)
[![license badge](https://img.shields.io/badge/license-%20GPL-blue.svg)](LICENSE)
[![link to Crates](https://img.shields.io/crates/v/polygons.svg)](https://crates.io/crates/polygons)
[![link to PyPI](https://badge.fury.io/py/polygons.svg)](https://badge.fury.io/py/polygons)
[![link to Zenodo/DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.3825616.svg)](https://doi.org/10.5281/zenodo.3825616)


# Polygons: Fast points-in-polygon test and distances to polygons

Computes distances to polygon edges and vertices and can check whether
points are inside/outside.

This library is optimized to perform well with hundreds or thousands of
polygons and thousands or millions of points.

Example timings (190 polygons, 1 M reference points, run on i7-10710U):
- distances to nearest edges: 0.7 s
- distances to nearest vertices: 0.6 s
- check whether points are inside or outside: 0.1 s


## Installation using pip

```
$ pip install polygons
```

## Supported versions

- Python: 3.8 - 3.12
- Operating systems: Linux, macOS, and Windows


## Capabilities

- Check whether points are inside or outside polygons
- Nearest distances to edges
- Nearest distances to vertices


## Recommended citation

If you use this tool in a program or publication, please acknowledge its
author(s):

```bibtex
@misc{polygons,
  author    = {Bast, Radovan},
  title     = {Polygons: Fast points-in-polygon test and distances to polygons},
  month     = {10},
  year      = {2024},
  publisher = {Zenodo},
  version   = {v0.3.4},
  doi       = {10.5281/zenodo.3825616},
  url       = {https://doi.org/10.5281/zenodo.3825616}
}
```


## Python example

```python
import polygons

# polygon_points is a list of lists
# the library has been developed to perform
# with very many polygons - this is just to have a simple example
# in this example the polygons have the same number of points but there
# is no restriction like this, this is only an example
polygon_points = [
    [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
    [(0.0, 2.0), (1.0, 2.0), (1.0, 3.0), (0.0, 3.0)],
]

# the more points you compute in one go, the better
# here using two points to make a simple example but if you have many points
# then compute a thousand or a million in one go
# so that the library can parallelize over the points
points = [(0.5, 0.5), (0.5, -0.5)]

# parameters for the tree construction:
#  - each tree node has 4 children nodes
#  - each leaf collects 4 edges
# you can try different parameters and check the timing
# they (should) have no effect on the results apart from timing
num_edges_children = 4
num_nodes_children = 4
tree = polygons.build_search_tree(
    polygon_points, num_edges_children, num_nodes_children
)

inside = polygons.points_are_inside(tree, points)
print(inside)  # [True, False]

# indices are the indices of the nearest polygon vertices (counted
# consecutively)
indices, distances = polygons.distances_nearest_vertices(tree, points)
print(indices)  # [0, 0]
print(distances)  # [0.7071067811865476, 0.7071067811865476]

distances = polygons.distances_nearest_edges(tree, points)
print(distances)  # [0.5, 0.5]

indices, distances = polygons.distances_nearest_vertices(
    tree, [(0.6, 0.6), (0.5, -0.5)]
)
print(indices)  # [2, 0]
print(distances)  # [0.5656854249492381, 0.7071067811865476]
```

## References which were used during coding

- <http://geomalgorithms.com/a03-_inclusion.html>
- <https://en.wikipedia.org/wiki/Point_in_polygon>
- <https://en.wikipedia.org/wiki/Binary_space_partitioning>


## Development notes

Running the benchmark:
```
$ cargo test --release -- --ignored --nocapture
```

Python interface inspired by https://github.com/dev-cafe/rustafarian.

Building and testing the Python interface:
```
$ maturin develop
```


## Image

Social media preview generated using https://github.com/qrohlf/trianglify.
