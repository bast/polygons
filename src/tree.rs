use crate::point::Point;
use rayon::prelude::*;

// a point in space, with an index
#[derive(Clone)]
struct IndexPoint {
    index: usize,
    x: f64,
    y: f64,
}

// edge connects two points
#[derive(Clone)]
pub struct Edge {
    p1: IndexPoint,
    p2: IndexPoint,
}

// node is a box which has dimensions
// it contains either other nodes
// or it contains edges
#[derive(Clone)]
pub struct Node {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub children_nodes: Vec<Box<Node>>,
    pub edges: Vec<Edge>,
}

impl Node {
    fn adjust_bounds(&mut self, xmin: f64, xmax: f64, ymin: f64, ymax: f64) {
        self.xmin = self.xmin.min(xmin);
        self.xmax = self.xmax.max(xmax);
        self.ymin = self.ymin.min(ymin);
        self.ymax = self.ymax.max(ymax);
    }
    fn insert_node(&mut self, new_node: Node) {
        let boxed_node = Box::new(new_node);
        self.children_nodes.push(boxed_node);
    }
    fn insert_edge(&mut self, new_edge: Edge) {
        self.edges.push(new_edge);
    }
}

fn box_distance(p: &Point, xmin: f64, xmax: f64, ymin: f64, ymax: f64) -> f64 {
    let difx = if p.x < xmin {
        p.x - xmin
    } else if p.x > xmax {
        p.x - xmax
    } else {
        0.0
    };

    let dify = if p.y < ymin {
        p.y - ymin
    } else if p.y > ymax {
        p.y - ymax
    } else {
        0.0
    };

    return distance_squared(difx, dify);
}

fn get_distance_edge(node: &Node, d: f64, p: &Point) -> f64 {
    if box_distance(&p, node.xmin, node.xmax, node.ymin, node.ymax) > d {
        return d;
    }

    let mut d_ = d;

    if node.children_nodes.len() > 0 {
        for child_node in node.children_nodes.iter() {
            let temp = get_distance_edge(&child_node, d_, &p);
            d_ = d_.min(temp);
        }
        return d_;
    }

    if node.edges.len() > 0 {
        for edge in node.edges.iter() {
            d_ = d_.min(dsegment(
                p.x, p.y, edge.p1.x, edge.p1.y, edge.p2.x, edge.p2.y,
            ));
        }
        return d_;
    }

    return d;
}

fn num_intersections(node: &Node, n: i32, p: &Point) -> i32 {
    if skip_box_intersection(p, node.xmax, node.ymin, node.ymax) {
        return n;
    }

    let mut n_ = n;

    if node.children_nodes.len() > 0 {
        for child_node in node.children_nodes.iter() {
            n_ = num_intersections(&child_node, n_, &p);
        }
        return n_;
    }

    if node.edges.len() > 0 {
        for edge in node.edges.iter() {
            if crosses(&p, &edge) {
                n_ += 1;
            }
        }
        return n_;
    }

    return n;
}

fn skip_box_intersection(p: &Point, xmax: f64, ymin: f64, ymax: f64) -> bool {
    if p.x > xmax {
        return true;
    }
    if p.y > ymax {
        return true;
    }
    if p.y < ymin {
        return true;
    }
    return false;
}

fn get_distance_vertex(node: &Node, index: usize, d: f64, p: &Point) -> (usize, f64) {
    if box_distance(&p, node.xmin, node.xmax, node.ymin, node.ymax) > d {
        return (index, d);
    }

    let mut d_ = d;
    let mut index_ = index;

    if node.children_nodes.len() > 0 {
        for child_node in node.children_nodes.iter() {
            let (it, dt) = get_distance_vertex(&child_node, index_, d_, p);
            if dt < d_ {
                d_ = dt;
                index_ = it;
            }
        }
        return (index_, d_);
    }

    if node.edges.len() > 0 {
        for edge in node.edges.iter() {
            let t = distance_squared(edge.p1.x - p.x, edge.p1.y - p.y);
            if t < d_ {
                d_ = t;
                index_ = edge.p1.index;
            }
        }

        let i = node.edges.len() - 1;
        let d_temp = distance_squared(node.edges[i].p2.x - p.x, node.edges[i].p2.y - p.y);
        if d_temp < d_ {
            d_ = d_temp;
            index_ = node.edges[i].p2.index;
        }
        return (index_, d_);
    }

    return (index_, d_);
}

// we compute the sqrt at the very end to save time
fn distance_squared(x: f64, y: f64) -> f64 {
    return x * x + y * y;
}

// this is derived from a C/C++ code
// Copyright (C) 2004-2012 Per-Olof Persson
// which was shared under GPL
fn dsegment(x0: f64, y0: f64, p1x: f64, p1y: f64, p2x: f64, p2y: f64) -> f64 {
    let v = (p2x - p1x, p2y - p1y);
    let w = (x0 - p1x, y0 - p1y);

    let c1 = v.0 * w.0 + v.1 * w.1;

    if c1 <= 0.0 {
        return distance_squared(x0 - p1x, y0 - p1y);
    }

    let c2 = v.0 * v.0 + v.1 * v.1;

    if c1 >= c2 {
        return distance_squared(x0 - p2x, y0 - p2y);
    } else {
        return distance_squared(x0 - (p1x + c1 / c2 * v.0), y0 - (p1y + c1 / c2 * v.1));
    }
}

pub fn contains_points(tree: &Vec<Node>, points: &Vec<Point>) -> Vec<bool> {
    // point is inside some polygon if the number of intersections to reach
    // the point "from left" is impair
    // FIXME clarify why we use tree[0]
    return points
        .par_iter()
        .map(|p| (num_intersections(&tree[0], 0, &p) % 2) != 0)
        .collect();
}

pub fn distances_nearest_edges(tree: &Vec<Node>, points: &Vec<Point>) -> Vec<f64> {
    let large_number = std::f64::MAX;

    return points
        .par_iter()
        .map(|p| get_distance_edge(&tree[0], large_number, &p).sqrt())
        .collect();
}

pub fn nearest_vertices(tree: &Vec<Node>, points: &Vec<Point>) -> (Vec<usize>, Vec<f64>) {
    let large_number = std::f64::MAX;

    let v: Vec<(usize, f64)> = points
        .par_iter()
        .map(|p| get_distance_vertex(&tree[0], 0, large_number, &p))
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
                );
                new_parent.insert_node(input[i].clone());
                i += 1;
            }
        }
        parents.push(new_parent);
    }

    return parents;
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

pub fn build_tree(
    polygons: &Vec<Vec<Edge>>,
    num_edges_children: usize,
    num_nodes_children: usize,
) -> Vec<Node> {
    let mut nodes = Vec::new();

    for p in polygons.iter() {
        // group edges to nodes, 4 at the time
        nodes.append(&mut group_edges(num_edges_children, p.clone()));
    }

    // we group nodes into a tree
    while nodes.len() > 1 {
        nodes = group_nodes(num_nodes_children, nodes);
    }

    return nodes;
}

// a_z is one component of the vector cross product
// a_z < 0 for r right of the (upward) line p1-p2
// a_z > 0 for r left of the (upward) line p1-p2
// a_z = 0 if r lies on the line p1-p2
fn a_z(r: &Point, e: &Edge) -> f64 {
    let b_x = e.p2.x - e.p1.x;
    let b_y = e.p2.y - e.p1.y;
    let c_x = r.x - e.p1.x;
    let c_y = r.y - e.p1.y;
    return b_x * c_y - b_y * c_x;
}

// The function "crosses" is based on http://geomalgorithms.com/a03-_inclusion.html
// which is distributed under the following license:

// Copyright 2000 softSurfer, 2012 Dan Sunday
// This code may be freely used and modified for any purpose
// providing that this copyright notice is included with it.
// SoftSurfer makes no warranty for this code, and cannot be held
// liable for any real or imagined damage resulting from its use.
// Users of this code must verify correctness for their application.
fn crosses(r: &Point, e: &Edge) -> bool {
    // reference point is above the edge so a horizontal line to the point
    // cannot crosse the edge
    if r.y > e.p1.y.max(e.p2.y) {
        return false;
    }

    // reference point is below the edge so a horizontal line to the point
    // cannot crosse the edge
    if r.y < e.p1.y.min(e.p2.y) {
        return false;
    }

    if e.p1.y < e.p2.y {
        // upward edge
        return a_z(&r, &e) > 0.0;
    } else {
        // downward edge
        return a_z(&r, &e) < 0.0;
    }
}
