[![Build Status](https://travis-ci.org/bast/polygons.svg?branch=master)](https://travis-ci.org/bast/polygons/builds)
[![License](https://img.shields.io/badge/license-%20GPL-blue.svg)](../master/LICENSE)


## Polygons: Fast points-in-polygon test and distances to polygons

### Installation using pip

```shell
$ pip install git+https://github.com/bast/polygons.git
```


### Example

```python
>>> import polygons
>>> context = polygons.new_context()
>>> polygons.add_polygon(context, [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.0, 0.0)])
>>> polygons.contains_points(context, [(0.5, 0.5), (0.5, -0.5)])
[True, False]
>>> polygons.get_distances_to_nearest_edge(context, [(0.5, 0.5), (0.5, -0.5)])
[0.5, 0.5]
>>> polygons.free_context(context)
```


### References

- http://geomalgorithms.com/a03-_inclusion.html
- https://en.wikipedia.org/wiki/Point_in_polygon
- https://en.wikipedia.org/wiki/Binary_space_partitioning
