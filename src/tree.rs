use pyo3::prelude::*;
use rayon::prelude::*;

use crate::distance;
use crate::intersections;
use crate::point::Point;

// edge connects two points
#[derive(Clone)]
pub struct Edge {
    pub p1: Point,
    pub p2: Point,
}

// node is a box which has dimensions
// it contains either other nodes
// or it contains edges
#[pyclass]
#[derive(Clone)]
pub struct Node {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub hmin: f64,
    pub children_nodes: Vec<Box<Node>>,
    pub edges: Vec<Edge>,
}

pub type Tree = Vec<Node>;

impl Node {
    fn adjust_bounds(&mut self, xmin: f64, xmax: f64, ymin: f64, ymax: f64, hmin: f64) {
        self.xmin = self.xmin.min(xmin);
        self.xmax = self.xmax.max(xmax);
        self.ymin = self.ymin.min(ymin);
        self.ymax = self.ymax.max(ymax);
        self.hmin = self.hmin.min(hmin);
    }
    fn insert_node(&mut self, new_node: Node) {
        let boxed_node = Box::new(new_node);
        self.children_nodes.push(boxed_node);
    }
    fn insert_edge(&mut self, new_edge: Edge) {
        self.edges.push(new_edge);
    }
}

#[pyfunction]
pub fn build_search_tree(
    polygons: Vec<Vec<(f64, f64)>>,
    num_edges_children: usize,
    num_nodes_children: usize,
) -> Vec<Node> {
    let polygons_h = pad(polygons);
    build_search_tree_h(polygons_h, num_edges_children, num_nodes_children)
}

#[pyfunction]
pub fn build_search_tree_h(
    polygons: Vec<Vec<(f64, f64, f64)>>,
    num_edges_children: usize,
    num_nodes_children: usize,
) -> Vec<Node> {
    let mut nodes = Vec::new();

    for (i, polygon) in polygons.iter().enumerate() {
        if !polygon_is_closed(&polygon) {
            panic!("ERROR: polygon {} is not closed", i + 1);
        }

        // group edges to nodes, num_edges_children at the time
        nodes.append(&mut group_edges(
            num_edges_children,
            points_to_edges(&polygon),
        ));
    }

    // we group nodes into a tree
    while nodes.len() > 1 {
        nodes = group_nodes(num_nodes_children, nodes);
    }

    nodes
}

fn floats_are_same(f1: f64, f2: f64) -> bool {
    return (f1 - f2).abs() < std::f64::EPSILON;
}

fn polygon_is_closed(polygon: &[(f64, f64, f64)]) -> bool {
    let first_point = polygon.first().unwrap();
    let last_point = polygon.last().unwrap();

    if !floats_are_same(first_point.0, last_point.0) {
        return false;
    }

    if !floats_are_same(first_point.1, last_point.1) {
        return false;
    }

    true
}

fn pad(input: Vec<Vec<(f64, f64)>>) -> Vec<Vec<(f64, f64, f64)>> {
    let mut output = Vec::new();
    for v in input {
        output.push(v.iter().map(|(x, y)| (*x, *y, 0.0)).collect());
    }
    output
}

pub fn points_are_inside(tree: &Tree, points: &[(f64, f64)]) -> Vec<bool> {
    // point is inside some polygon if the number of intersections to reach
    // the point "from left" is impair
    points
        .par_iter()
        .map(|p| (intersections::num_intersections(&tree[0], 0, *p) % 2) != 0)
        .collect()
}

pub fn distances_nearest_edges(tree: &Tree, points: &[(f64, f64)]) -> Vec<f64> {
    let large_number = std::f64::MAX;

    points
        .par_iter()
        .map(|p| distance::get_distance_edge(&tree[0], large_number, *p))
        .collect()
}

pub fn distances_nearest_vertices(tree: &Tree, points: &[(f64, f64)]) -> Vec<f64> {
    let large_number = std::f64::MAX;

    points
        .par_iter()
        .map(|p| distance::get_distance_vertex(&tree[0], large_number, *p))
        .collect()
}

fn group_nodes(num_nodes_children: usize, input: Vec<Node>) -> Vec<Node> {
    let num_input = input.len();
    let n = num_input / num_nodes_children;
    let num_parents = match num_input % num_nodes_children {
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
            hmin: large_number,
            edges: Vec::new(),
            children_nodes: Vec::new(),
        };
        for _l in 0..num_nodes_children {
            if i < input.len() {
                new_parent.adjust_bounds(
                    input[i].xmin,
                    input[i].xmax,
                    input[i].ymin,
                    input[i].ymax,
                    input[i].hmin,
                );
                new_parent.insert_node(input[i].clone());
                i += 1;
            }
        }
        parents.push(new_parent);
    }

    parents
}

fn group_edges(num_edges_children: usize, input: Vec<Edge>) -> Tree {
    let num_input = input.len();
    let n = num_input / num_edges_children;
    let num_parents = match num_input % num_edges_children {
        0 => n,
        _ => n + 1,
    };

    let large_number = std::f64::MAX;

    let mut parents = Vec::new();

    let mut i = 0;
    for _k in 0..num_parents {
        let mut new_parent = Node {
            xmin: large_number,
            xmax: -large_number,
            ymin: large_number,
            ymax: -large_number,
            hmin: large_number,
            edges: Vec::new(),
            children_nodes: Vec::new(),
        };
        for _l in 0..num_edges_children {
            if i < input.len() {
                new_parent.adjust_bounds(
                    input[i].p1.x,
                    input[i].p1.x,
                    input[i].p1.y,
                    input[i].p1.y,
                    input[i].p1.h,
                );
                new_parent.adjust_bounds(
                    input[i].p2.x,
                    input[i].p2.x,
                    input[i].p2.y,
                    input[i].p2.y,
                    input[i].p2.h,
                );
                new_parent.insert_edge(input[i].clone());
                i += 1;
            }
        }
        parents.push(new_parent);
    }

    parents
}

fn points_to_edges(points: &[(f64, f64, f64)]) -> Vec<Edge> {
    points
        .windows(2)
        .map(|pair| Edge {
            p1: Point {
                x: pair[0].0,
                y: pair[0].1,
                h: pair[0].2,
            },
            p2: Point {
                x: pair[1].0,
                y: pair[1].1,
                h: pair[1].2,
            },
        })
        .collect()
}
