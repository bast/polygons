import os
from cffi import FFI
from .cffi_helpers import get_lib_handle
import numpy as np


_this_path = os.path.dirname(os.path.realpath(__file__))

_build_dir = os.getenv('POLYGONS_BUILD_DIR')
if _build_dir is None:
    _build_dir = _this_path
else:
    _build_dir = os.path.join(_build_dir, 'lib')

_include_dir = _this_path

_lib = get_lib_handle(
    ['-DPOLYGONS_API=', '-DCPP_INTERFACE_NOINCLUDE'],
    'polygons.h',
    'polygons',
    _build_dir,
    _include_dir
)

_ffi = FFI()


new_context = _lib.polygons_new_context
free_context = _lib.polygons_free_context


def add_polygon(context, points, indices):

    num_points = len(points)

    # cast a pointer which points to the numpy array data
    # we work with numpy because tree initialization with normal lists segfault
    # for lists longer than ca. 0.5 million points
    x_coordinates, y_coordinates = zip(*points)
    x_coordinates_np = np.array(x_coordinates)
    x_coordinates_p = _ffi.cast("double *", x_coordinates_np.ctypes.data)
    y_coordinates_np = np.array(y_coordinates)
    y_coordinates_p = _ffi.cast("double *", y_coordinates_np.ctypes.data)
#   fails for reason not clear to me
#   indices_np = np.array(indices)
#   indices_p = _ffi.cast("int *", indices_np.ctypes.data)

    _lib.polygons_add_polygon(context,
                              num_points,
                              x_coordinates_p,
                              y_coordinates_p,
#                             indices_p,
                              indices)


def get_distances_edge(context, points):

    num_points = len(points)

    x_coordinates, y_coordinates = zip(*points)
    x_coordinates_np = np.array(x_coordinates)
    x_coordinates_p = _ffi.cast("double *", x_coordinates_np.ctypes.data)
    y_coordinates_np = np.array(y_coordinates)
    y_coordinates_p = _ffi.cast("double *", y_coordinates_np.ctypes.data)
    distances_np = np.zeros(num_points, dtype=np.float64)
    distances_p = _ffi.cast("double *", distances_np.ctypes.data)

    _lib.polygons_get_distances_edge(context,
                                     num_points,
                                     x_coordinates_p,
                                     y_coordinates_p,
                                     distances_p)

    return distances_np.tolist()


def get_distances_vertex(context, points):

    num_points = len(points)

    x_coordinates, y_coordinates = zip(*points)
    x_coordinates_np = np.array(x_coordinates)
    x_coordinates_p = _ffi.cast("double *", x_coordinates_np.ctypes.data)
    y_coordinates_np = np.array(y_coordinates)
    y_coordinates_p = _ffi.cast("double *", y_coordinates_np.ctypes.data)
    distances_np = np.zeros(num_points, dtype=np.float64)
    distances_p = _ffi.cast("double *", distances_np.ctypes.data)

    _lib.polygons_get_distances_vertex(context,
                                       num_points,
                                       x_coordinates_p,
                                       y_coordinates_p,
                                       distances_p)

    return distances_np.tolist()


def get_closest_vertices(context, points):

    num_points = len(points)

    x_coordinates, y_coordinates = zip(*points)
    x_coordinates_np = np.array(x_coordinates)
    x_coordinates_p = _ffi.cast("double *", x_coordinates_np.ctypes.data)
    y_coordinates_np = np.array(y_coordinates)
    y_coordinates_p = _ffi.cast("double *", y_coordinates_np.ctypes.data)
    indices_np = np.zeros(num_points, dtype=np.intc)
    indices_p = _ffi.cast("int *", indices_np.ctypes.data)

    _lib.polygons_get_closest_vertices(context,
                                       num_points,
                                       x_coordinates_p,
                                       y_coordinates_p,
                                       indices_p)

    return indices_np.tolist()


def contains_points(context, points):

    num_points = len(points)

    x_coordinates, y_coordinates = zip(*points)
    x_coordinates_np = np.array(x_coordinates)
    x_coordinates_p = _ffi.cast("double *", x_coordinates_np.ctypes.data)
    y_coordinates_np = np.array(y_coordinates)
    y_coordinates_p = _ffi.cast("double *", y_coordinates_np.ctypes.data)
    contains_points_np = np.zeros(num_points, dtype=np.bool)
    contains_points_p = _ffi.cast("bool *", contains_points_np.ctypes.data)

    _lib.polygons_contains_points(context,
                                  num_points,
                                  x_coordinates_p,
                                  y_coordinates_p,
                                  contains_points_p)

    return contains_points_np.tolist()
