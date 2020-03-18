#![allow(clippy::needless_return)]

use std::time::Instant;
extern crate rand;
use rand::Rng;

extern crate polygons;
use polygons::{Edge, Point};

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

fn run_benchmark() {
    let points: Vec<Point> = polygons::read_vector("tests/polygon.txt");
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for p in points.iter() {
        xs.push(p.x);
        ys.push(p.y);
    }

    let offset = 5.0;

    let num_points = xs.len();

    let num_blocks = 10;
    let num_reference_points = 100_000;

    let mut polygons: Vec<Vec<Edge>> = Vec::new();
    for i in 0..num_blocks {
        let polygon =
            polygons::create_polygon(num_points, &xs, i as f64 * offset, &ys, 0.0, i * num_points);
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
    let (_indices, _distances) = polygons::nearest_vertices(&tree, &reference_points);
    let _contains = polygons::contains_points(&tree, &reference_points);
    println!("time elapsed in benchmark: {:?}", start.elapsed());
}

fn main() {
    run_benchmark();
}
