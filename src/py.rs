mod py {
    use pyo3::prelude::*;
    use pyo3::wrap_pyfunction;

    use crate::point::Point;
    use crate::tree;
    use crate::tree::{Node, Tree};

    #[pyfunction]
    fn build_tree(
        polygons_in: Vec<Vec<(f64, f64)>>,
        num_edges_children: usize,
        num_nodes_children: usize,
    ) -> Tree {
        let mut polygons = Vec::new();

        for polygon_in in polygons_in.iter() {
            let mut polygon = Vec::new();
            for &(x, y) in polygon_in.iter() {
                polygon.push(Point { x, y, coeff: 0.0 });
            }
            polygons.push(polygon);
        }

        return tree::build_tree(&polygons, num_edges_children, num_nodes_children);
    }

    fn tuples_to_points(vector_in: Vec<(f64, f64)>) -> Vec<Point> {
        let mut vector_out = Vec::new();
        for &(x, y) in vector_in.iter() {
            vector_out.push(Point::new(x, y));
        }
        return vector_out;
    }

    #[pyfunction]
    fn points_are_inside(tree: Vec<Node>, points_in: Vec<(f64, f64)>) -> Vec<bool> {
        let points = tuples_to_points(points_in);
        return tree::points_are_inside(&tree, &points);
    }

    #[pyfunction]
    fn distances_nearest_vertices(tree: Vec<Node>, points_in: Vec<(f64, f64)>) -> Vec<f64> {
        let points = tuples_to_points(points_in);
        return tree::distances_nearest_vertices(&tree, &points);
    }

    #[pyfunction]
    fn distances_nearest_edges(tree: Vec<Node>, points_in: Vec<(f64, f64)>) -> Vec<f64> {
        let points = tuples_to_points(points_in);
        return tree::distances_nearest_edges(&tree, &points);
    }

    #[pymodule]
    fn polygons(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add("__version__", env!("CARGO_PKG_VERSION"))?;

        m.add_wrapped(wrap_pyfunction!(build_tree))?;
        m.add_wrapped(wrap_pyfunction!(points_are_inside))?;
        m.add_wrapped(wrap_pyfunction!(distances_nearest_vertices))?;
        m.add_wrapped(wrap_pyfunction!(distances_nearest_edges))?;

        Ok(())
    }
}
