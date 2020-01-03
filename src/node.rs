use crate::edge::Edge;
use crate::intersection;
use crate::point::Point;

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
    pub fn adjust_bounds(&mut self, xmin: f64, xmax: f64, ymin: f64, ymax: f64) {
        self.xmin = self.xmin.min(xmin);
        self.xmax = self.xmax.max(xmax);
        self.ymin = self.ymin.min(ymin);
        self.ymax = self.ymax.max(ymax);
    }
    pub fn insert_node(&mut self, new_node: Node) {
        let boxed_node = Box::new(new_node);
        self.children_nodes.push(boxed_node);
    }
    pub fn insert_edge(&mut self, new_edge: Edge) {
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

pub fn get_distance_edge(node: &Node, d: f64, p: &Point) -> f64 {
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

pub fn num_intersections(node: &Node, n: i32, p: &Point) -> i32 {
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
            if intersection::crosses(&p, &edge) {
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

pub fn get_distance_vertex(node: &Node, index: usize, d: f64, p: &Point) -> (usize, f64) {
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
