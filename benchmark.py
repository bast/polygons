import sys
import random
import time
import polygons as poly
# import matplotlib.pyplot as plt
# import matplotlib.patches as patches
# from shapely.geometry import Polygon, Point

# fig = plt.figure()
# ax2 = fig.add_subplot(111, aspect='equal')


def length_squared(x, y):
    return x * x + y * y


def dsegment(point, vertex1, vertex2):
    v = (vertex2[0] - vertex1[0], vertex2[1] - vertex1[1])
    w = (point[0] - vertex1[0], point[1] - vertex1[1])
    c1 = v[0] * w[0] + v[1] * w[1]

    if (c1 <= 0.0):
        return length_squared(point[0] - vertex1[0], point[1] - vertex1[1])

    c2 = v[0] * v[0] + v[1] * v[1]

    if (c1 >= c2):
        return length_squared(point[0] - vertex2[0], point[1] - vertex2[1])
    else:
        return length_squared(point[0] - (vertex1[0] + c1 / c2 * v[0]),
                              point[1] - (vertex1[1] + c1 / c2 * v[1]))


def vdsegment(points, polygons):
    huge = sys.float_info.max
    distances_squared = []
    for point in points:
        d = huge
        for polygon in polygons:
            for edge in polygon:
                _d = dsegment(point, edge.point1, edge.point2)
                d = min(d, _d)
        distances_squared.append(d)
    return distances_squared


def read_polygon(file_name, xshift, yshift):
    vertices = []
    with open(file_name, 'r') as f:
        for line in f:
            x = float(line.split()[0]) + xshift
            y = float(line.split()[1]) + yshift
            vertices.append((x, y))
    return vertices, [Edge(p1, p2) for (p1, p2) in zip(vertices, vertices[1:])]


def init_bounds():
    large = sys.float_info.max
    bounds = [[large, -large],
              [large, -large]]
    return bounds


def adjust_bounds(bounds, point):
    (x, y) = point
    return [[min(bounds[0][0], x),
             max(bounds[0][1], x)],
            [min(bounds[1][0], y),
             max(bounds[1][1], y)]]


class Edge():
    def __init__(self, point1, point2):
        self.point1 = point1
        self.point2 = point2
        self.bounds = init_bounds()
        self.bounds = adjust_bounds(self.bounds, point1)
        self.bounds = adjust_bounds(self.bounds, point2)

    def get_distance(self, _, point):
        return dsegment(point, self.point1, self.point2)


class Node():
    def __init__(self):
        self.children = []
        self.bounds = init_bounds()

    def skip_box(self, distance, point):
        """
        If best case distance is larger than currently optimum distance, this box is rejected.
        """
        #    |   |
        #  1 | 2 | 3
        # ___|___|___
        #    |   |
        #  4 | 5 | 6
        # ___|___|___
        #    |   |
        #  7 | 8 | 9
        #    |   |
        if point[1] > self.bounds[1][1]:
            # 1, 2, 3
            if point[0] < self.bounds[0][0]:
                # 1
                return length_squared(point[0] - self.bounds[0][0], point[1] - self.bounds[1][1]) > distance
            elif point[0] > self.bounds[0][1]:
                # 3
                return length_squared(point[0] - self.bounds[0][1], point[1] - self.bounds[1][1]) > distance
            else:
                # 2
                return (point[1] - self.bounds[1][1])**2.0 > distance
        elif point[1] < self.bounds[1][0]:
            # 7, 8, 9
            if point[0] < self.bounds[0][0]:
                # 7
                return length_squared(point[0] - self.bounds[0][0], point[1] - self.bounds[1][0]) > distance
            elif point[0] > self.bounds[0][1]:
                # 9
                return length_squared(point[0] - self.bounds[0][1], point[1] - self.bounds[1][0]) > distance
            else:
                # 8
                return (point[1] - self.bounds[1][0])**2.0 > distance
        else:
            # 4, 5, 6
            if point[0] < self.bounds[0][0]:
                # 4
                return (point[0] - self.bounds[0][0])**2.0 > distance
            elif point[0] > self.bounds[0][1]:
                # 6
                return (point[0] - self.bounds[0][1])**2.0 > distance
            else:
                # 5
                return False

    def add_child(self, child):
        self.children.append(child)
        self.bounds = adjust_bounds(self.bounds, (child.bounds[0][0], child.bounds[1][0]))
        self.bounds = adjust_bounds(self.bounds, (child.bounds[0][1], child.bounds[1][1]))

    def get_distance(self, distance, point):
        # if type(self.children[0]).__name__ == 'Edge':
        #     color = "blue"
        # else:
        #     color = "red"
        # ax2.add_patch(
        #     patches.Rectangle(
        #         (self.bounds[0][0], self.bounds[1][0]),
        #         self.bounds[0][1] - self.bounds[0][0],
        #         self.bounds[1][1] - self.bounds[1][0],
        #         fill=False,
        #         edgecolor=color,
        #     )
        # )
        if self.skip_box(distance, point):
            return distance
        for child in self.children:
            distance = min(distance, child.get_distance(distance, point))
        return distance


def build_nodes(n, children):
    _children = iter(children)

    nodes = []
    done = False
    while True:
        node = Node()
        for _ in range(n):
            try:
                node.add_child(_children.__next__())
            except StopIteration:
                done = True
        if len(node.children) > 0:
            nodes.append(node)
        if done:
            break
    return nodes


def generate_random_points(num_points, bounds):
    random.seed(1)
    points = [(random.uniform(bounds[0][0], bounds[0][1]),
               random.uniform(bounds[1][0], bounds[1][1])) for _ in range(num_points)]
    return points


num_points = 1000
num_polygons = 1
max_num_edges = 4
max_num_children = 4

polygons = []
for i in range(num_polygons):
    _, polygon = read_polygon('data/polygon.txt', xshift=float(i) * 5.0, yshift=float(i) * 5.0)
    polygons.append(polygon)

bounds = init_bounds()
for polygon in polygons:
    for edge in polygon:
        bounds = adjust_bounds(bounds, edge.point1)
        bounds = adjust_bounds(bounds, edge.point2)

points = generate_random_points(num_points, bounds)

# for edge in edges:
#     plt.plot([edge.point1[0], edge.point2[0]], [edge.point1[1], edge.point2[1]], 'k-')

t0 = time.time()
distances_squared_naive = vdsegment(points, polygons)
print('time used in naive search: {}'.format(time.time() - t0))

t0 = time.time()
nodes = []
for polygon in polygons:
    # FIXME each polygon should be one box
    nodes += build_nodes(max_num_edges, polygon)
while len(nodes) > 1:
    nodes = build_nodes(max_num_children, nodes)
root = nodes[0]
print('time used in building tree: {}'.format(time.time() - t0))

t0 = time.time()
huge = sys.float_info.max
distances_squared_tree = []
for i, point in enumerate(points):
    distances_squared_tree.append(root.get_distance(huge, point))
print('time used in tree search: {}'.format(time.time() - t0))

for i, point in enumerate(points):
    diff = abs(distances_squared_tree[i] - distances_squared_naive[i])
    assert diff < 1.0e-7

# fig.savefig('rect.png', dpi=90, bbox_inches='tight')

context = poly.new_context()

vertices, _ = read_polygon('data/polygon.txt', xshift=0.0, yshift=0.0)

poly.add_polygon(context, vertices)

t0 = time.time()
distances_poly = poly.get_distances(context, points)  # FIXME so far only one polygon
print('time used in polygons: {}'.format(time.time() - t0))
poly.free_context(context)

for i, point in enumerate(points):
    diff = abs(distances_squared_tree[i] - distances_poly[i]**2.0)
    assert diff < 1.0e-7

sys.exit()
polygons = []
for i in range(num_polygons):
    vertices, _ = read_polygon('data/polygon.txt', xshift=float(i) * 5.0, yshift=float(i) * 5.0)
    polygons.append(Polygon(vertices))

t0 = time.time()
distances_shapely = []
for polygon in polygons:
    for point in points:
        distances_shapely.append(polygon.distance(Point(point[0], point[1])))
print('time used in shapely: {}'.format(time.time() - t0))
