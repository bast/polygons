extern crate polygons;
use polygons::{Edge, Point};

fn floats_are_same(f1: f64, f2: f64) -> bool {
    let d = f1 - f2;
    return d.abs() < std::f64::EPSILON;
}

#[test]
fn polygon() {
    let mut polygons: Vec<Vec<Edge>> = Vec::new();

    let points: Vec<Point> = polygons::read_vector("tests/polygon.txt");
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for p in points.iter() {
        xs.push(p.x);
        ys.push(p.y);
    }

    let num_points = points.len();
    let polygon = polygons::create_polygon(num_points, &xs, 0.0, &ys, 0.0);
    polygons.push(polygon);
    let polygon = polygons::create_polygon(num_points, &xs, 5.0, &ys, 0.0);
    polygons.push(polygon);
    let polygon = polygons::create_polygon(num_points, &xs, 10.0, &ys, 0.0);
    polygons.push(polygon);
    let polygon = polygons::create_polygon(num_points, &xs, 15.0, &ys, 0.0);
    polygons.push(polygon);
    let polygon = polygons::create_polygon(num_points, &xs, 20.0, &ys, 0.0);
    polygons.push(polygon);

    let tree = polygons::build_tree(&polygons, 4, 4);

    let reference_points: Vec<Point> =
        polygons::read_vector("tests/reference/reference_points.txt");

    let distances = polygons::distances_nearest_edges(&tree, &reference_points);
    let reference_distances = polygons::read_vector("tests/reference/distances_edge.txt");
    for (&x, &rx) in distances.iter().zip(reference_distances.iter()) {
        assert!(floats_are_same(x, rx));
    }

    let distances = polygons::distances_nearest_vertices(&tree, &reference_points);
    let reference_distances = polygons::read_vector("tests/reference/distances_vertex.txt");
    for (&x, &rx) in distances.iter().zip(reference_distances.iter()) {
        assert!(floats_are_same(x, rx));
    }

    let contains = polygons::contains_points(&tree, &reference_points);
    let reference_bools = polygons::read_vector("tests/reference/contains_points.txt");
    for (&x, &rx) in contains.iter().zip(reference_bools.iter()) {
        assert_eq!(x, rx);
    }
}
