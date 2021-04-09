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
