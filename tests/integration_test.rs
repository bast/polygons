extern crate polygons;
use polygons::Point;

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

fn polygon_is_closed(polygon: &[Point]) -> bool {
    let first_point = polygon.first().unwrap();
    let last_point = polygon.last().unwrap();

    if !floats_are_same(first_point.x, last_point.x) {
        return false;
    }

    if !floats_are_same(first_point.y, last_point.y) {
        return false;
    }

    return true;
}

fn read_polygons(file_name: &str) -> Vec<Vec<Point>> {
    let error_message = format!("something went wrong reading file {}", file_name);
    let contents = fs::read_to_string(file_name).expect(&error_message);

    let mut polygons = Vec::new();
    let mut polygon = Vec::new();

    let mut i = 0;
    for line in contents.lines() {
        if i == 0 {
            let num_points: usize = line.parse().unwrap();
            i += num_points + 1;
            if !polygon.is_empty() {
                if !polygon_is_closed(&polygon) {
                    polygon.push(polygon.first().unwrap().clone());
                }
                polygons.push(polygon.clone());
            }
            polygon.clear();
        } else {
            let words: Vec<&str> = line.split_whitespace().collect();
            let x = words[0].parse().unwrap();
            let y = words[1].parse().unwrap();
            polygon.push(Point { x, y });
        }
        i -= 1;
    }

    if !polygon.is_empty() {
        if !polygon_is_closed(&polygon) {
            polygon.push(polygon.first().unwrap().clone());
        }
        polygons.push(polygon);
    }

    return polygons;
}

#[test]
fn basic() {
    let polygons = read_polygons("tests/islands.txt");

    let tree = polygons::build_tree(&polygons, 4, 4);

    let reference_points: Vec<Point> = read_vector("tests/reference/reference_points.txt");

    let distances = polygons::distances_nearest_edges(&tree, &reference_points);
    let reference_distances = read_vector("tests/reference/distances_nearest_edges.txt");
    for (&x, &rx) in distances.iter().zip(reference_distances.iter()) {
        assert!(floats_are_same(x, rx));
    }

    let distances = polygons::distances_nearest_vertices(&tree, &reference_points);
    let reference_distances = read_vector("tests/reference/distances_nearest_vertices.txt");
    for (&x, &rx) in distances.iter().zip(reference_distances.iter()) {
        assert!(floats_are_same(x, rx));
    }

    let contains = polygons::points_are_inside(&tree, &reference_points);
    let reference_bools = read_vector("tests/reference/points_are_inside.txt");
    for (&x, &rx) in contains.iter().zip(reference_bools.iter()) {
        assert_eq!(x, rx);
    }
}

#[ignore]
#[test]
fn benchmark() {
    let polygons = read_polygons("tests/islands.txt");

    let large_number = std::f64::MAX;

    let mut x_min = large_number;
    let mut x_max = -large_number;
    let mut y_min = large_number;
    let mut y_max = -large_number;

    for polygon in polygons.iter() {
        for point in polygon.iter() {
            x_min = x_min.min(point.x);
            x_max = x_max.max(point.x);
            y_min = y_min.min(point.y);
            y_max = y_max.max(point.y);
        }
    }

    let num_reference_points = 1_000_000;
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
    let _contains = polygons::points_are_inside(&tree, &reference_points);
    println!("time elapsed in points_are_inside: {:?}", start.elapsed());
}
