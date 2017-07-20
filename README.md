[![Build Status](https://travis-ci.org/bast/polygons.svg?branch=master)](https://travis-ci.org/bast/polygons/builds)
[![License](https://img.shields.io/badge/license-%20GPL-blue.svg)](../master/LICENSE)


## Polygons: Fast points-in-polygon test and distances to polygons

### Installation using pip

```shell
$ pip install git+https://github.com/bast/polygons.git
```


### Capabilities

- check whether points are inside or outside polygons
- nearest distances to edges
- nearest distances to vertices
- nearest weighted distances to vertices (see below for explanation)


### Example

```python
>>> import polygons
>>> context = polygons.new_context()
>>> polygons.add_polygon(context, [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.0, 0.0)])
>>> polygons.contains_points(context, [(0.5, 0.5), (0.5, -0.5)])
[True, False]
>>> polygons.get_distances_edge(context, [(0.5, 0.5), (0.5, -0.5)])
[0.5, 0.5]
>>> polygons.get_distances_vertex(context, [(0.5, 0.5), (0.5, -0.5)])
[0.7071067811865476, 0.7071067811865476]
>>> polygons.free_context(context)
```


### Weighted distances to vertices

Instead of finding r by minimizing
```
f(r0, r) = d(r0, r)
```
it is possible to minimize
```
f(r0, r) = a * d(r0, r) + w(r)
```


### References which were used during coding

- http://geomalgorithms.com/a03-_inclusion.html
- https://en.wikipedia.org/wiki/Point_in_polygon
- https://en.wikipedia.org/wiki/Binary_space_partitioning
