use polygons;

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
    contents.lines().map(|s| s.parse().unwrap()).collect()
}

fn read_tuples(file_name: &str) -> Vec<(f64, f64)> {
    let error_message = format!("something went wrong reading file {}", file_name);
    let contents = fs::read_to_string(file_name).expect(&error_message);

    let mut tuples = Vec::new();

    for line in contents.split("\n") {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() == 2 {
            let x = words[0].parse().unwrap();
            let y = words[1].parse().unwrap();
            tuples.push((x, y));
        }
    }

    tuples
}

fn get_random_points(
    num_points: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
) -> Vec<(f64, f64)> {
    let mut rng = rand::thread_rng();
    let mut reference_points = Vec::new();
    for _ in 0..num_points {
        reference_points.push((rng.gen_range(x_min..x_max), rng.gen_range(y_min..y_max)));
    }
    reference_points
}

fn floats_are_same(f1: f64, f2: f64) -> bool {
    return (f1 - f2).abs() < std::f64::EPSILON;
}

fn read_polygons(file_name: &str) -> Vec<Vec<(f64, f64, f64)>> {
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
                polygons.push(polygon.clone());
            }
            polygon.clear();
        } else {
            let words: Vec<&str> = line.split_whitespace().collect();
            let x = words[0].parse().unwrap();
            let y = words[1].parse().unwrap();
            let h = words[2].parse().unwrap();
            polygon.push((x, y, h));
        }
        i -= 1;
    }

    if !polygon.is_empty() {
        polygons.push(polygon);
    }

    polygons
}

fn get_bounds(polygons: &[Vec<(f64, f64, f64)>]) -> (f64, f64, f64, f64) {
    let large_number = std::f64::MAX;

    let mut x_min = large_number;
    let mut x_max = -large_number;
    let mut y_min = large_number;
    let mut y_max = -large_number;

    for polygon in polygons {
        for (x, y, _) in polygon {
            x_min = x_min.min(*x);
            x_max = x_max.max(*x);
            y_min = y_min.min(*y);
            y_max = y_max.max(*y);
        }
    }

    (x_min, x_max, y_min, y_max)
}

fn distances_nearest_vertices_custom_naive(
    polygons: &[Vec<(f64, f64, f64)>],
    reference_points: &[(f64, f64)],
) -> (Vec<usize>, Vec<f64>) {
    let large_number = std::f64::MAX;

    let mut indices = Vec::new();
    let mut distances = Vec::new();

    for (rx, ry) in reference_points {
        let mut index = 0;
        let mut distance = large_number;

        let mut i = 0;
        for polygon in polygons {
            for (x, y, h) in polygon {
                let dx = x - rx;
                let dy = y - ry;
                let d = (dx * dx + dy * dy).sqrt();
                if d + h < distance {
                    distance = d + h;
                    index = i;
                }
                i += 1;
            }
        }

        indices.push(index);
        distances.push(distance);
    }

    (indices, distances)
}

fn zero_out_h(polygons: Vec<Vec<(f64, f64, f64)>>) -> Vec<Vec<(f64, f64, f64)>> {
    let mut polygons_zeroed = Vec::new();
    for polygon in polygons {
        polygons_zeroed.push(polygon.iter().map(|(x, y, _)| (*x, *y, 0.0)).collect());
    }
    polygons_zeroed
}

#[test]
fn basic() {
    let polygons = read_polygons("tests/islands.txt");
    let polygons = zero_out_h(polygons);

    let tree = polygons::build_search_tree_h(polygons, 4, 4);

    let reference_points = read_tuples("tests/reference/reference_points.txt");

    let distances = polygons::distances_nearest_edges(&tree, &reference_points);
    let reference_distances = read_vector("tests/reference/distances_nearest_edges.txt");
    for (&x, &rx) in distances.iter().zip(reference_distances.iter()) {
        assert!(floats_are_same(x, rx));
    }

    let (_, distances) = polygons::distances_nearest_vertices(&tree, &reference_points);
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

#[test]
fn custom_distance() {
    let polygons = read_polygons("tests/islands.txt");
    let (x_min, x_max, y_min, y_max) = get_bounds(&polygons);

    let num_reference_points = 10_000;
    let reference_points = get_random_points(num_reference_points, x_min, x_max, y_min, y_max);

    let (indices_naive, distances_naive) =
        distances_nearest_vertices_custom_naive(&polygons, &reference_points);

    let tree = polygons::build_search_tree_h(polygons, 4, 4);
    let (indices, distances) = polygons::distances_nearest_vertices(&tree, &reference_points);

    assert_eq!(indices, indices_naive);
    for (&x, &rx) in distances.iter().zip(distances_naive.iter()) {
        assert!(floats_are_same(x, rx));
    }
}

#[ignore]
#[test]
fn benchmark() {
    let polygons = read_polygons("tests/islands.txt");
    let polygons = zero_out_h(polygons);

    let num_reference_points = 1_000_000;
    let (x_min, x_max, y_min, y_max) = get_bounds(&polygons);
    let reference_points = get_random_points(num_reference_points, x_min, x_max, y_min, y_max);

    let start = Instant::now();
    let tree = polygons::build_search_tree_h(polygons, 16, 16);
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
