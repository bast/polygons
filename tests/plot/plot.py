import matplotlib.pyplot as plt
import sys


def read_data(file_name):
    points = []
    with open(file_name, 'r') as f:
        for line in f.readlines():
            x = float(line.split()[0])
            y = float(line.split()[1])
            points.append((x, y))
    return points


def parse_command_line():

    if len(sys.argv) != 3:
        sys.stderr.write('Usage: {0} [in-txt-file] [out-png-file]\n'.format(sys.argv[0]))
        sys.exit(1)

    in_file_name = sys.argv[1]
    out_file_name = sys.argv[2]

    return in_file_name, out_file_name


def generate_plot(points, out_file_name):
    x, y = zip(*points)

    plt.figure()
    plt.gca().set_aspect('equal')
    plt.plot(x, y, 'g-', markersize=0.2, linewidth=0.2)
    plt.savefig(out_file_name, dpi=300)


if __name__ == '__main__':
    in_file_name, out_file_name = parse_command_line()
    points = read_data(in_file_name)
    generate_plot(points, out_file_name)
