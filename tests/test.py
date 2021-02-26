import polygons
import os
import sys


def floats_are_same(f1, f2):
    d = f1 - f2
    return abs(d) < sys.float_info.epsilon


def points_are_same(p1, p2):
    (x1, y1) = p1
    (x2, y2) = p2
    if not floats_are_same(x1, x2):
        return False
    if not floats_are_same(y1, y2):
        return False
    return True


def read_polygons(file_name):
    polygons = []

    with open(file_name, "r") as f:
        while True:
            try:
                num_points = int(next(f))
            except StopIteration:
                break

            polygon = []
            for _ in range(num_points):
                line = next(f)
                t = tuple(map(float, line.split()))
                polygon.append((t[0], t[1]))
            if not points_are_same(polygon[0], polygon[-1]):
                polygon.append(polygon[0])
            polygons.append(polygon)

    return polygons


def read_data(file_name, g):
    data = []
    with open(file_name, "r") as f:
        for line in f:
            data.append(g(line.strip()))
    return data


def test_interface():
    here = os.path.dirname(os.path.realpath(__file__))

    ps = read_polygons(os.path.join(here, "islands.txt"))
    num_edges_children = 4
    num_nodes_children = 4

    tree = polygons.build_search_tree(ps, num_edges_children, num_nodes_children)

    points = read_data(
        os.path.join(here, "reference", "reference_points.txt"),
        lambda x: tuple(map(float, x.split())),
    )

    inside = polygons.points_are_inside(tree, points)
    inside_reference = read_data(
        os.path.join(here, "reference", "points_are_inside.txt"),
        lambda x: x.startswith("true"),
    )
    assert inside == inside_reference

    distances = polygons.distances_nearest_vertices(tree, points)
    distances_reference = read_data(
        os.path.join(here, "reference", "distances_nearest_vertices.txt"),
        lambda x: float(x),
    )
    assert all([floats_are_same(a, b) for a, b in zip(distances, distances_reference)])

    distances = polygons.distances_nearest_edges(tree, points)
    distances_reference = read_data(
        os.path.join(here, "reference", "distances_nearest_edges.txt"),
        lambda x: float(x),
    )
    assert all([floats_are_same(a, b) for a, b in zip(distances, distances_reference)])
