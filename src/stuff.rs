use crate::node;
use crate::structures::{Edge, IndexPoint, Node, Point};
use rayon::prelude::*;

pub fn contains_points(tree: &Vec<Node>, points: &Vec<Point>) -> Vec<bool> {
    // point is inside some polygon if the number of intersections to reach
    // the point "from left" is impair
    // FIXME clarify why we use tree[0]
    return points
        .par_iter()
        .map(|p| (node::num_intersections(&tree[0], 0, &p) % 2) != 0)
        .collect();
}

pub fn get_distances_edge(tree: &Vec<Node>, points: &Vec<Point>) -> Vec<f64> {
    let large_number = std::f64::MAX;

    return points
        .par_iter()
        .map(|p| node::get_distance_edge(&tree[0], large_number, &p).sqrt())
        .collect();
}

pub fn get_distances_vertex(tree: &Vec<Node>, points: &Vec<Point>) -> (Vec<usize>, Vec<f64>) {
    let large_number = std::f64::MAX;

    let v: Vec<(usize, f64)> = points
        .par_iter()
        .map(|p| node::get_distance_vertex(&tree[0], 0, large_number, &p))
        .collect();

    let (indices, _distances): (Vec<usize>, Vec<f64>) = v.iter().cloned().unzip();
    let distances = _distances.iter().map(|x| x.sqrt()).collect();

    return (indices, distances);
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

fn group_nodes(num_per_node: usize, input: Vec<Node>) -> Vec<Node> {
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

fn group_edges(num_per_node: usize, input: Vec<Edge>) -> Vec<Node> {
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

pub fn get_tree(polygons: &Vec<Vec<Edge>>) -> Vec<Node> {
    let mut nodes = Vec::new();

    for p in polygons.iter() {
        // group edges to nodes, 4 at the time
        nodes.append(&mut group_edges(4, p.clone()));
    }

    // we group nodes into a tree
    while nodes.len() > 1 {
        nodes = group_nodes(4, nodes);
    }

    return nodes;
}
