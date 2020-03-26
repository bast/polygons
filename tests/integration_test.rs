extern crate polygons;
use polygons::{Edge, Point};

use std::time::Instant;
extern crate rand;
use rand::Rng;

use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

fn read_vector<T: FromStr>(file_name: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let error_message = format!("something went wrong reading file {}", file_name);
    let contents = fs::read_to_string(file_name).expect(&error_message);
    let v = contents.lines().map(|s| s.parse().unwrap()).collect();

    return v;
}

fn get_reference_points(
    num_points: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let mut reference_points = Vec::new();

    for _ in 0..num_points {
        reference_points.push(Point {
            x: rng.gen_range(x_min, x_max),
            y: rng.gen_range(y_min, y_max),
        });
    }
    return reference_points;
}

fn floats_are_same(f1: f64, f2: f64) -> bool {
    let d = f1 - f2;
    return d.abs() < std::f64::EPSILON;
}

#[test]
fn basic() {
    let mut polygons: Vec<Vec<Edge>> = Vec::new();

    let points: Vec<Point> = read_vector("tests/polygon.txt");
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

    let reference_points: Vec<Point> = read_vector("tests/reference/reference_points.txt");

    let distances = polygons::distances_nearest_edges(&tree, &reference_points);
    let reference_distances = read_vector("tests/reference/distances_edge.txt");
    for (&x, &rx) in distances.iter().zip(reference_distances.iter()) {
        assert!(floats_are_same(x, rx));
    }

    let distances = polygons::distances_nearest_vertices(&tree, &reference_points);
    let reference_distances = read_vector("tests/reference/distances_vertex.txt");
    for (&x, &rx) in distances.iter().zip(reference_distances.iter()) {
        assert!(floats_are_same(x, rx));
    }

    let contains = polygons::contains_points(&tree, &reference_points);
    let reference_bools = read_vector("tests/reference/contains_points.txt");
    for (&x, &rx) in contains.iter().zip(reference_bools.iter()) {
        assert_eq!(x, rx);
    }
}

#[ignore]
#[test]
fn benchmark() {
    let points: Vec<Point> = read_vector("tests/polygon.txt");
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for p in points.iter() {
        xs.push(p.x);
        ys.push(p.y);
    }

    let offset = 5.0;

    let num_points = xs.len();

    let num_blocks = 50;
    let num_reference_points = 500_000;

    let mut polygons: Vec<Vec<Edge>> = Vec::new();
    for i in 0..num_blocks {
        let polygon = polygons::create_polygon(num_points, &xs, i as f64 * offset, &ys, 0.0);
        polygons.push(polygon);
    }

    let (x_min, x_max) = (-1.0, (num_blocks - 1) as f64 * offset + 2.0);
    let (y_min, y_max) = (-1.0, 2.0);
    let reference_points = get_reference_points(num_reference_points, x_min, x_max, y_min, y_max);

    let start = Instant::now();
    let tree = polygons::build_tree(&polygons, 16, 16);
    println!("time elapsed in building tree: {:?}", start.elapsed());

    let start = Instant::now();
    let _distances = polygons::distances_nearest_edges(&tree, &reference_points);
    println!(
        "time elapsed in distances_nearest_edges: {:?}",
        start.elapsed()
    );

    let start = Instant::now();
    let _distances = polygons::distances_nearest_vertices(&tree, &reference_points);
    println!(
        "time elapsed in distances_nearest_vertices: {:?}",
        start.elapsed()
    );

    let start = Instant::now();
    let _contains = polygons::contains_points(&tree, &reference_points);
    println!("time elapsed in contains_points: {:?}", start.elapsed());
}
