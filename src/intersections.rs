use crate::point::Point;
use crate::tree::{Edge, Node};

pub fn num_intersections(node: &Node, n: i32, p: &Point) -> i32 {
    if skip_box_intersection(&p, &node) {
        return n;
    }

    let mut n_ = n;

    if !node.children_nodes.is_empty() {
        for child_node in &node.children_nodes {
            n_ = num_intersections(&child_node, n_, &p);
        }
        return n_;
    }

    if !node.edges.is_empty() {
        for edge in &node.edges {
            if crosses(&p, &edge) {
                n_ += 1;
            }
        }
        return n_;
    }

    n
}

fn skip_box_intersection(p: &Point, node: &Node) -> bool {
    // shouldn't this be p.x < node.xmin?
    if p.x > node.xmax {
        return true;
    }
    if p.y > node.ymax {
        return true;
    }
    if p.y < node.ymin {
        return true;
    }
    false
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

    b_x * c_y - b_y * c_x
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
        a_z(&r, &e) > 0.0
    } else {
        // downward edge
        a_z(&r, &e) < 0.0
    }
}
