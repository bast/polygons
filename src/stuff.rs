use crate::node;
use crate::structures::{Edge, IndexPoint, Node, Point};

pub fn contains_points(
    tree: &Vec<Node>,
    num_points: usize,
    x: &[f64],
    y: &[f64],
    contains: &mut [bool],
) {
    for elem in contains.iter_mut() {
        *elem = false;
    }

    for i in 0..num_points {
        let p = Point { x: x[i], y: y[i] };
        // FIXME clarify why we use tree[0]
        contains[i] = (node::num_intersections(&tree[0], 0, &p) % 2) != 0;
    }
}

pub fn get_distances_edge(
    tree: &Vec<Node>,
    num_points: usize,
    x: &[f64],
    y: &[f64],
    distances: &mut [f64],
) {
    let large_number = std::f64::MAX;

    for i in 0..num_points {
        let p = IndexPoint {
            index: 0,
            x: x[i],
            y: y[i],
        };
        distances[i] = node::get_distance_edge(&tree[0], large_number, &p).sqrt();
    }
}

pub fn get_distances_vertex(
    tree: &Vec<Node>,
    num_points: usize,
    x: &[f64],
    y: &[f64],
    distances: &mut [f64],
) {
    let large_number = std::f64::MAX;

    for i in 0..num_points {
        let p = IndexPoint {
            index: 0,
            x: x[i],
            y: y[i],
        };
        let (_, d) = node::get_distance_vertex(&tree[0], 0, large_number, &p);
        distances[i] = d.sqrt();
    }
}

pub fn get_closest_vertices(
    tree: &Vec<Node>,
    num_points: usize,
    x: &[f64],
    y: &[f64],
    indices: &mut [usize],
) {
    let large_number = std::f64::MAX;

    for i in 0..num_points {
        let p = IndexPoint {
            index: 0,
            x: x[i],
            y: y[i],
        };
        let (iv, _) = node::get_distance_vertex(&tree[0], 0, large_number, &p);
        indices[i] = iv;
    }
}

pub fn create_polygon(
    num_points: usize,
    xs: &Vec<f64>,
    x_offset: f64,
    ys: &Vec<f64>,
    y_offset: f64,
    start_index: usize,
) -> Vec<Edge> {
    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..(num_points - 1) {
        let p1 = IndexPoint {
            index: start_index + i,
            x: xs[i] + x_offset,
            y: ys[i] + y_offset,
        };
        let p2 = IndexPoint {
            index: start_index + i + 1,
            x: xs[i + 1] + x_offset,
            y: ys[i + 1] + y_offset,
        };
        let e = Edge { p1: p1, p2: p2 };
        edges.push(e);
    }
    return edges;
}

pub fn group_nodes(num_per_node: usize, input: Vec<Node>) -> Vec<Node> {
    let num_input = input.len();
    let n = num_input / num_per_node;
    let num_parents = match num_input % num_per_node {
        0 => n,
        _ => n + 1,
    };

    let large_number = std::f64::MAX;

    let mut parents: Vec<Node> = Vec::new();

    let mut i = 0;
    for _k in 0..num_parents {
        let mut new_parent = Node {
            xmin: large_number,
            xmax: -large_number,
            ymin: large_number,
            ymax: -large_number,
            edges: Vec::new(),
            children_nodes: Vec::new(),
        };
        for _l in 0..num_per_node {
            if i < input.len() {
                new_parent.adjust_bounds(
                    input[i].xmin,
                    input[i].xmax,
                    input[i].ymin,
                    input[i].ymax,
                );
                new_parent.insert_node(input[i].clone());
                i += 1;
            }
        }
        parents.push(new_parent);
    }

    return parents;
}

pub fn group_edges(num_per_node: usize, input: Vec<Edge>) -> Vec<Node> {
    let num_input = input.len();
    let n = num_input / num_per_node;
    let num_parents = match num_input % num_per_node {
        0 => n,
        _ => n + 1,
    };

    let large_number = std::f64::MAX;

    let mut parents: Vec<Node> = Vec::new();

    let mut i = 0;
    for _k in 0..num_parents {
        let mut new_parent = Node {
            xmin: large_number,
            xmax: -large_number,
            ymin: large_number,
            ymax: -large_number,
            edges: Vec::new(),
            children_nodes: Vec::new(),
        };
        for _l in 0..num_per_node {
            if i < input.len() {
                new_parent.adjust_bounds(
                    input[i].p1.x,
                    input[i].p1.x,
                    input[i].p1.y,
                    input[i].p1.y,
                );
                new_parent.adjust_bounds(
                    input[i].p2.x,
                    input[i].p2.x,
                    input[i].p2.y,
                    input[i].p2.y,
                );
                new_parent.insert_edge(input[i].clone());
                i += 1;
            }
        }
        parents.push(new_parent);
    }

    return parents;
}
