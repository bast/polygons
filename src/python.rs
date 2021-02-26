use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::tree;
use crate::tree::Tree;
use crate::tree::__pyo3_get_function_build_search_tree;
use crate::tree::__pyo3_get_function_build_search_tree_h;

#[pyfunction]
fn points_are_inside(tree: Tree, points: Vec<(f64, f64)>) -> Vec<bool> {
    tree::points_are_inside(&tree, &points)
}

#[pyfunction]
fn distances_nearest_vertices(tree: Tree, points: Vec<(f64, f64)>) -> Vec<f64> {
    tree::distances_nearest_vertices(&tree, &points)
}

#[pyfunction]
fn distances_nearest_edges(tree: Tree, points: Vec<(f64, f64)>) -> Vec<f64> {
    tree::distances_nearest_edges(&tree, &points)
}

#[pymodule]
fn polygons(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_function(wrap_pyfunction!(build_search_tree, m)?)?;
    m.add_function(wrap_pyfunction!(build_search_tree_h, m)?)?;
    m.add_function(wrap_pyfunction!(points_are_inside, m)?)?;
    m.add_function(wrap_pyfunction!(distances_nearest_vertices, m)?)?;
    m.add_function(wrap_pyfunction!(distances_nearest_edges, m)?)?;

    Ok(())
}
