use pyo3::prelude::*;
use rayon::prelude::*;

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
    hmin: f64,
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

fn box_distance(p: &Point, node: &Node) -> f64 {
    let difx = if p.x < node.xmin {
        p.x - node.xmin
    } else if p.x > node.xmax {
        p.x - node.xmax
    } else {
        0.0
    };

    let dify = if p.y < node.ymin {
        p.y - node.ymin
    } else if p.y > node.ymax {
        p.y - node.ymax
    } else {
        0.0
    };

    distance(difx, dify)
}

fn get_distance_edge(node: &Node, d: f64, p: &Point) -> f64 {
    if box_distance(&p, &node) > d {
        return d;
    }

    let mut d_ = d;

    if !node.children_nodes.is_empty() {
        for child_node in node.children_nodes.iter() {
            let temp = get_distance_edge(&child_node, d_, &p);
            d_ = d_.min(temp);
        }
        return d_;
    }

    if !node.edges.is_empty() {
        for edge in node.edges.iter() {
            d_ = d_.min(dsegment(
                p.x, p.y, edge.p1.x, edge.p1.y, edge.p2.x, edge.p2.y,
            ));
        }
        return d_;
    }

    d
}

fn get_distance_vertex(node: &Node, d: f64, p: &Point) -> f64 {
    let d_box = box_distance(&p, &node);

    let f = d_box + node.hmin;

    if f > d {
        return d;
    }

    let mut d_min = d;

    if !node.children_nodes.is_empty() {
        for child_node in node.children_nodes.iter() {
            let t = get_distance_vertex(&child_node, d_min, p);
            d_min = d_min.min(t);
        }
        return d_min;
    }

    if !node.edges.is_empty() {
        for edge in node.edges.iter() {
            let d_edge = distance(edge.p1.x - p.x, edge.p1.y - p.y);
            let f = d_edge + edge.p1.coeff;
            d_min = d_min.min(f);
        }

        let edge = node.edges.last().unwrap();
        let d_edge = distance(edge.p2.x - p.x, edge.p2.y - p.y);
        let f = d_edge + edge.p2.coeff;
        d_min = d_min.min(f);

        return d_min;
    }

    d_min
}

fn distance(x: f64, y: f64) -> f64 {
    (x * x + y * y).sqrt()
}

// this is derived from a C/C++ code
// Copyright (C) 2004-2012 Per-Olof Persson
// which was shared under GPL
fn dsegment(x0: f64, y0: f64, p1x: f64, p1y: f64, p2x: f64, p2y: f64) -> f64 {
    let v = (p2x - p1x, p2y - p1y);
    let w = (x0 - p1x, y0 - p1y);

    let c1 = v.0 * w.0 + v.1 * w.1;

    if c1 <= 0.0 {
        return distance(x0 - p1x, y0 - p1y);
    }

    let c2 = v.0 * v.0 + v.1 * v.1;

    if c1 >= c2 {
        distance(x0 - p2x, y0 - p2y)
    } else {
        distance(x0 - (p1x + c1 / c2 * v.0), y0 - (p1y + c1 / c2 * v.1))
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
        .map(|p| get_distance_edge(&tree[0], large_number, &p))
        .collect()
}

pub fn distances_nearest_vertices(tree: &[Node], points: &[Point]) -> Vec<f64> {
    let large_number = std::f64::MAX;

    points
        .par_iter()
        .map(|p| get_distance_vertex(&tree[0], large_number, &p))
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
