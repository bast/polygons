use polygons::io;
use polygons::structures::{Edge, Point};
use polygons::stuff;

fn floats_are_same(f1: f64, f2: f64) -> bool {
    let d = f1 - f2;
    return d.abs() < std::f64::EPSILON;
}

#[test]
fn polygon() {
    let mut polygons: Vec<Vec<Edge>> = Vec::new();

    let points: Vec<Point> = io::read_vector("tests/polygon.txt");
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for p in points.iter() {
        xs.push(p.x);
        ys.push(p.y);
    }

    let num_points = points.len();
    let polygon = stuff::create_polygon(num_points, &xs, 0.0, &ys, 0.0, 0);
    polygons.push(polygon);
    let polygon = stuff::create_polygon(num_points, &xs, 5.0, &ys, 0.0, num_points);
    polygons.push(polygon);
    let polygon = stuff::create_polygon(num_points, &xs, 10.0, &ys, 0.0, 2 * num_points);
    polygons.push(polygon);
    let polygon = stuff::create_polygon(num_points, &xs, 15.0, &ys, 0.0, 3 * num_points);
    polygons.push(polygon);
    let polygon = stuff::create_polygon(num_points, &xs, 20.0, &ys, 0.0, 4 * num_points);
    polygons.push(polygon);

    let tree = stuff::get_tree(&polygons);

    let reference_points: Vec<Point> = io::read_vector("tests/reference/reference_points.txt");

    let distances = stuff::get_distances_edge(&tree, &reference_points);
    let reference_distances = io::read_vector("tests/reference/distances_edge.txt");
    for (&x, &rx) in distances.iter().zip(reference_distances.iter()) {
        assert!(floats_are_same(x, rx));
    }

    let (indices, distances) = stuff::get_distances_vertex(&tree, &reference_points);

    let reference_distances = io::read_vector("tests/reference/distances_vertex.txt");
    for (&x, &rx) in distances.iter().zip(reference_distances.iter()) {
        assert!(floats_are_same(x, rx));
    }

    let reference_indices = io::read_vector("tests/reference/closest_indices.txt");
    for (&x, &rx) in indices.iter().zip(reference_indices.iter()) {
        assert_eq!(x, rx);
    }

    let contains = stuff::contains_points(&tree, &reference_points);
    let reference_bools = io::read_vector("tests/reference/contains_points.txt");
    for (&x, &rx) in contains.iter().zip(reference_bools.iter()) {
        assert_eq!(x, rx);
    }
}
