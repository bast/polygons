import random
import polygons

def test_contains():
    num_points = 1000

    context = polygons.new_context()

    polygons.add_polygon(context, [(2.0, 1.0), (3.0, 1.5), (2.5, 2.0), (2.0, 1.0)])
    polygons.add_polygon(context, [(0.0, 0.0), (1.0, 0.5), (0.5, 1.0), (0.0, 0.0)])
    polygons.add_polygon(context, [(0.0, 2.0), (1.0, 2.5), (0.5, 3.0), (0.0, 2.0)])

    random.seed(0)
    points = [(random.uniform(0.0, 3.0), random.uniform(0.0, 3.0)) for _ in range(num_points)]

    result = polygons.contains_points(context, points)

    checksum = sum(i for i in range(num_points) if result[i])
    assert checksum == 63726

    polygons.free_context(context)
