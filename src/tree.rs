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

pub fn points_are_inside(tree: &[Node], points: &[Point]) -> Vec<bool> {
    // point is inside some polygon if the number of intersections to reach
    // the point "from left" is impair
    // FIXME clarify why we use tree[0]
    points
        .par_iter()
        .map(|p| (intersections::num_intersections(&tree[0], 0, &p) % 2) != 0)
        .collect()
}

pub fn distances_nearest_edges(tree: &[Node], points: &[Point]) -> Vec<f64> {
    let large_number = std::f64::MAX;

    points
        .par_iter()
        .map(|p| distance::get_distance_edge(&tree[0], large_number, &p))
        .collect()
}

pub fn distances_nearest_vertices(tree: &[Node], points: &[Point]) -> Vec<f64> {
    let large_number = std::f64::MAX;

    points
        .par_iter()
        .map(|p| distance::get_distance_vertex(&tree[0], large_number, &p))
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

fn group_edges(num_edges_children: usize, input: Vec<Edge>) -> Vec<Node> {
    let num_input = input.len();
    let n = num_input / num_edges_children;
    let num_parents = match num_input % num_edges_children {
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
        for _l in 0..num_edges_children {
            if i < input.len() {
                new_parent.adjust_bounds(
                    input[i].p1.x,
                    input[i].p1.x,
                    input[i].p1.y,
                    input[i].p1.y,
                    input[i].p1.coeff,
                );
                new_parent.adjust_bounds(
                    input[i].p2.x,
                    input[i].p2.x,
                    input[i].p2.y,
                    input[i].p2.y,
                    input[i].p2.coeff,
                );
                new_parent.insert_edge(input[i].clone());
                i += 1;
            }
        }
        parents.push(new_parent);
    }

    parents
}

fn points_to_edges(points: &[Point]) -> Vec<Edge> {
    points
        .windows(2)
        .map(|pair| Edge {
            p1: pair[0],
            p2: pair[1],
        })
        .collect()
}

pub fn build_tree(
    polygons: &[Vec<Point>],
    num_edges_children: usize,
    num_nodes_children: usize,
) -> Vec<Node> {
    let mut nodes = Vec::new();

    for polygon in polygons.iter() {
        // group edges to nodes, num_edges_children at the time
        nodes.append(&mut group_edges(
            num_edges_children,
            points_to_edges(polygon),
        ));
    }

    // we group nodes into a tree
    while nodes.len() > 1 {
        nodes = group_nodes(num_nodes_children, nodes);
    }

    nodes
}
