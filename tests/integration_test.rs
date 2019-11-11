use polygons::io;
use polygons::structures::Edge;
use polygons::stuff;

#[test]
fn check_whole_thing() {
    let mut polygons: Vec<Vec<Edge>> = Vec::new();

    let (xs, ys) = io::read_polygon("tests/polygon.txt".to_string());
    let num_points = xs.len();
    let polygon = stuff::create_polygon(num_points, &xs, &ys, 0);
    let polygon2 = stuff::create_polygon(num_points, &xs, &ys, 0);
    polygons.push(polygon);
    polygons.push(polygon2);

    let mut nodes = Vec::new();
    for p in polygons.iter() {
        //      println!("polygon starts");
        for edge in p.iter() {
            println!(
                "p1:({},{},{}) p2:({},{},{})",
                edge.p1.index, edge.p1.x, edge.p1.y, edge.p2.index, edge.p2.x, edge.p2.y
            );
        }
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
    for d in distances.iter() {
        println!("distance edge: {}", d);
    }

    distances = [0.0; 5];
    stuff::get_distances_vertex(&nodes, 5, &pxs, &pys, &mut distances);
    for d in distances.iter() {
        println!("distance vertex: {}", d);
    }

    let mut indices: [usize; 5] = [0; 5];
    stuff::get_closest_vertices(&nodes, 5, &pxs, &pys, &mut indices);
    for i in indices.iter() {
        println!("closest indices: {}", i);
    }

    let mut contains: [bool; 5] = [false; 5];
    stuff::contains_points(&nodes, 5, &pxs, &pys, &mut contains);
    for c in contains.iter() {
        println!("contains: {}", c);
    }

    assert_eq!(1, 1);
}
