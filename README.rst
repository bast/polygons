.. image:: https://github.com/bast/polygons/workflows/Test/badge.svg
   :target: https://github.com/bast/polygons/actions
.. image:: https://img.shields.io/badge/license-%20GPL-blue.svg
   :target: LICENSE
.. image:: https://img.shields.io/crates/v/polygons.svg
   :target: https://crates.io/crates/polygons


Polygons: Fast points-in-polygon test and distances to polygons
===============================================================

.. contents:: Table of contents


Installation using pip
----------------------

.. code:: shell

  $ pip install polygons


Capabilities
------------

- Check whether points are inside or outside polygons
- Nearest distances to edges
- Nearest distances to vertices


Recommended citation
--------------------

If you use this code in research, please contact me and remind me to add here a
recommended citation.


Python example
--------------

.. code:: python

  import polygons

  # polygon_points is a list of lists
  # the library has been developed to perform
  # with very many polygons - this is just to have a simple example
  polygon_points = [
      [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.0, 0.0)],
      [(0.0, 2.0), (1.0, 2.0), (1.0, 3.0), (0.0, 3.0), (0.0, 2.0)],
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
  tree = polygons.build_tree(polygon_points, num_edges_children, num_nodes_children)

  inside = polygons.points_are_inside(tree, points)
  print(inside)  # [True, False]

  distances = polygons.distances_nearest_vertices(tree, points)
  print(distances)  # [0.7071067811865476, 0.7071067811865476]

  distances = polygons.distances_nearest_edges(tree, points)
  print(distances)  # [0.5, 0.5]

  distances = polygons.distances_nearest_vertices(tree, [(0.6, 0.6), (0.5, -0.5)])
  print(distances)  # [0.5656854249492381, 0.7071067811865476]


References which were used during coding
----------------------------------------

-  http://geomalgorithms.com/a03-_inclusion.html
-  https://en.wikipedia.org/wiki/Point_in_polygon
-  https://en.wikipedia.org/wiki/Binary_space_partitioning


Development notes
-----------------

Running the benchmark::

  $ cargo test --release -- --ignored --nocapture

Python interface inspired by https://github.com/dev-cafe/rustafarian.

Building and testing the Python interface::

  $ cargo build --release --features pyo3
  $ maturin develop --release --cargo-extra-args="--features pyo3"
