use polygons::io;
use polygons::structures::Edge;
use polygons::stuff;

fn floats_are_same(f1: f64, f2: f64) -> bool {
    let d = f1 - f2;
    return d.abs() < std::f64::EPSILON;
}

#[test]
fn rectangle() {
    let mut polygons: Vec<Vec<Edge>> = Vec::new();

    let (xs, ys) = io::read_polygon("tests/rectangle.txt");
    let num_points = xs.len();
    let polygon = stuff::create_polygon(num_points, &xs, 0.0, &ys, 0.0, 0);
    polygons.push(polygon);

    let mut nodes = Vec::new();
    for p in polygons.iter() {
        // group edges to nodes, 4 at the time
        nodes.append(&mut stuff::group_edges(4, p.clone()));
    }

    // we group nodes into a tree
    while nodes.len() > 1 {
        nodes = stuff::group_nodes(4, nodes);
    }

    let pxs: [f64; 2] = [0.6, 0.5];
    let pys: [f64; 2] = [0.6, -0.5];

    let mut distances: [f64; 2] = [0.0; 2];
    stuff::get_distances_edge(&nodes, 2, &pxs, &pys, &mut distances);
    assert_eq!(distances, [0.4, 0.5]);

    distances = [0.0; 2];
    stuff::get_distances_vertex(&nodes, 2, &pxs, &pys, &mut distances);
    assert!(floats_are_same(distances[0], 0.5656854249492381));
    assert!(floats_are_same(distances[1], 0.7071067811865476));

    let mut indices: [usize; 2] = [0; 2];
    stuff::get_closest_vertices(&nodes, 2, &pxs, &pys, &mut indices);
    assert_eq!(indices, [2, 0]);

    let mut contains: [bool; 2] = [false; 2];
    stuff::contains_points(&nodes, 2, &pxs, &pys, &mut contains);
    assert_eq!(contains, [true, false]);
}

#[test]
fn polygon() {
    let mut polygons: Vec<Vec<Edge>> = Vec::new();

    let (xs, ys) = io::read_polygon("tests/polygon.txt");
    let num_points = xs.len();
    let polygon = stuff::create_polygon(num_points, &xs, 0.0, &ys, 0.0, 0);
    polygons.push(polygon);
    // polygons.push(polygon);

    let mut nodes = Vec::new();
    for p in polygons.iter() {
        // group edges to nodes, 4 at the time
        nodes.append(&mut stuff::group_edges(4, p.clone()));
    }

    // we group nodes into a tree
    while nodes.len() > 1 {
        nodes = stuff::group_nodes(4, nodes);
    }

    let pxs: [f64; 5] = [0.0, -0.7, -2.0, -3.0, -0.6];
    let pys: [f64; 5] = [0.0, 0.8, -2.0, 3.0, 0.7];

    let mut distances: [f64; 5] = [0.0; 5];
    stuff::get_distances_edge(&nodes, 5, &pxs, &pys, &mut distances);
    assert!(floats_are_same(distances[0], 0.1848953575038567));
    assert!(floats_are_same(distances[1], 0.13000361865815685));
    assert!(floats_are_same(distances[2], 2.1731177869167606));
    assert!(floats_are_same(distances[3], 3.2750519267930196));
    assert!(floats_are_same(distances[4], 0.028928961600324855));

    distances = [0.0; 5];
    stuff::get_distances_vertex(&nodes, 5, &pxs, &pys, &mut distances);
    assert!(floats_are_same(distances[0], 0.1848953575038567));
    assert!(floats_are_same(distances[1], 0.13127646943315613));
    assert!(floats_are_same(distances[2], 2.1731177869167606));
    assert!(floats_are_same(distances[3], 3.2750519267930196));
    assert!(floats_are_same(distances[4], 0.029435910544000285));

    let mut indices: [usize; 5] = [0; 5];
    stuff::get_closest_vertices(&nodes, 5, &pxs, &pys, &mut indices);
    assert_eq!(indices, [41, 159, 33, 0, 156]);

    let mut contains: [bool; 5] = [false; 5];
    stuff::contains_points(&nodes, 2, &pxs, &pys, &mut contains);
    assert_eq!(contains, [true, false, false, false, false]);
}
