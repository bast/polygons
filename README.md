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


### Example

```python
>>> import polygons
>>> context = polygons.new_context()
>>> polygon_points = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.0, 0.0)]
>>> coefficients = [1.0]*5
>>> polygons.add_polygon(context, polygon_points, coefficients)
>>> points = [(0.5, 0.5), (0.5, -0.5)]
>>> polygons.contains_points(context, points)
[True, False]
>>> polygons.get_distances_edge(context, points)
[0.5, 0.5]
>>> polygons.get_distances_vertex(context, points)
[0.7071067811865476, 0.7071067811865476]
>>> polygons.free_context(context)
```


### Weighted distances to vertices

OK these are a bit weird but I needed these for [another project](https://github.com/bast/smeshing).

Instead of finding r by minimizing
```
f(r0, r) = distance(r0, r)
```
it is possible to minimize
```
f(r0, r) = scale_factor * distance(r0, r) + weight(r)
```

Polygon coefficients have no effect for non-weighted distance functions.


### References which were used during coding

- http://geomalgorithms.com/a03-_inclusion.html
- https://en.wikipedia.org/wiki/Point_in_polygon
- https://en.wikipedia.org/wiki/Binary_space_partitioning
