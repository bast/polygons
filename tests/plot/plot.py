import matplotlib.pyplot as plt
import sys


def read_data(file_name):
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


def parse_command_line():

    if len(sys.argv) != 3:
        sys.stderr.write(
            "Usage: {0} [in-txt-file] [out-png-file]\n".format(sys.argv[0])
        )
        sys.exit(1)

    in_file_name = sys.argv[1]
    out_file_name = sys.argv[2]

    return in_file_name, out_file_name


if __name__ == "__main__":
    in_file_name, out_file_name = parse_command_line()

    polygons = read_data(in_file_name)

    plt.figure()
    plt.gca().set_aspect("equal")

    for polygon in polygons:
        x, y = zip(*polygon)
        plt.plot(x, y, "b-", markersize=0.2, linewidth=0.3)

    plt.savefig(out_file_name, dpi=300)
