import sys
import random
import click
import matplotlib.pyplot as plt
import polygons


def read_polygons(file_name):
    polygons = []
    points = []
    with open(file_name, "r") as f:
        for line in f:
            num_points = int(line)
            for _ in range(num_points):
                xy = next(f)
                x = float(xy.split()[0])
                y = float(xy.split()[1])
                points.append((x, y))
            polygons.append(points)
            points = []
    return polygons


@click.command()
@click.option("--polygons-file", required=True, help="File name containing polygons.")
@click.option("--figure", required=True, help="Output file name.")
def main(polygons_file, figure):
    plt.figure()
    plt.gca().set_aspect("equal")

    large_number = sys.float_info.max
    xmin = large_number
    xmax = -large_number
    ymin = large_number
    ymax = -large_number

    ps = read_polygons(polygons_file)

    for polygon in ps:
        xs, ys = zip(*polygon)
        plt.plot(xs, ys, "b-", markersize=0.2, linewidth=0.3)
        xmin = min(xmin, min(xs))
        xmax = max(xmax, max(xs))
        ymin = min(ymin, min(ys))
        ymax = max(ymax, max(ys))

    num_edges_children = 4
    num_nodes_children = 4
    tree = polygons.build_tree(ps, num_edges_children, num_nodes_children)

    num_points = 50000
    points = [
        (random.uniform(xmin, xmax), random.uniform(ymin, ymax))
        for _ in range(num_points)
    ]
    inside = polygons.points_are_inside(tree, points)

    xs_inside = []
    ys_inside = []
    xs_outside = []
    ys_outside = []
    for i, (xs, ys) in enumerate(points):
        if inside[i]:
            xs_inside.append(xs)
            ys_inside.append(ys)
        else:
            xs_outside.append(xs)
            ys_outside.append(ys)

    plt.scatter(xs_inside, ys_inside, s=0.01, color="red")
    plt.scatter(xs_outside, ys_outside, s=0.01, color="blue")

    plt.savefig(figure, dpi=600)


if __name__ == "__main__":
    main()
