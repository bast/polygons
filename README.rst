.. image:: https://travis-ci.org/bast/polygons.svg?branch=master
   :target: https://travis-ci.org/bast/polygons/builds
.. image:: https://img.shields.io/badge/license-%20GPL-blue.svg
   :target: ../master/LICENSE


Polygons: Fast points-in-polygon test and distances to polygons
===============================================================

Installation using pip
----------------------

.. code:: shell

    $ pip install git+https://github.com/bast/polygons.git


Capabilities
------------

-  check whether points are inside or outside polygons
-  nearest distances to edges
-  nearest distances to vertices
-  indices of nearest vertices


Example
-------

.. code:: python

    >>> import polygons
    >>> context = polygons.new_context()
    >>> polygon_points = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.0, 0.0)]
    >>> polygons.add_polygon(context, polygon_points, [0, 1, 2, 3, 0])
    >>> points = [(0.5, 0.5), (0.5, -0.5)]
    >>> polygons.contains_points(context, points)
    [True, False]
    >>> polygons.get_distances_edge(context, points)
    [0.5, 0.5]
    >>> polygons.get_distances_vertex(context, points)
    [0.7071067811865476, 0.7071067811865476]
    >>> polygons.get_closest_vertices(context, points)
    [0, 0]
    >>> polygons.get_closest_vertices(context, [(0.6, 0.6), (0.5, -0.5)])
    [2, 0]
    >>> polygons.free_context(context)


References which were used during coding
----------------------------------------

-  http://geomalgorithms.com/a03-_inclusion.html
-  https://en.wikipedia.org/wiki/Point_in_polygon
-  https://en.wikipedia.org/wiki/Binary_space_partitioning
