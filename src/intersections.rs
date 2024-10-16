use crate::tree::{Edge, Node};
use float_cmp::approx_eq;

// we count each intersection twice since this makes it possible to deal with the reference point
// has the same y-coordinate as an edge point and then we can avoid double-counting the
// intersection
pub fn num_intersections(node: &Node, n: i32, p: (f64, f64)) -> i32 {
    if skip_box_intersection(p, node) {
        return n;
    }

    let mut n_ = n;

    if !node.children_nodes.is_empty() {
        for child_node in &node.children_nodes {
            n_ = num_intersections(child_node, n_, p);
        }
        return n_;
    }

    if !node.edges.is_empty() {
        for edge in &node.edges {
            if crosses(p, edge) {
                // if y-coordinate of reference point is equal to y-coordinate of edge point
                if (approx_eq!(f64, p.1, edge.p1.y, ulps = 2) && edge.p1.in_between)
                    || (approx_eq!(f64, p.1, edge.p2.y, ulps = 2) && edge.p2.in_between)
                {
                    n_ += 1;
                } else {
                    n_ += 2;
                }
            }
        }
        return n_;
    }

    n
}

fn skip_box_intersection(p: (f64, f64), node: &Node) -> bool {
    if p.0 < node.xmin {
        return true;
    }
    if p.1 > node.ymax {
        return true;
    }
    if p.1 < node.ymin {
        return true;
    }
    false
}

// a_z is one component of the vector cross product
// a_z < 0 for r right of the (upward) line p1-p2
// a_z > 0 for r left of the (upward) line p1-p2
// a_z = 0 if r lies on the line p1-p2
fn a_z(r: (f64, f64), e: &Edge) -> f64 {
    let b_x = e.p2.x - e.p1.x;
    let b_y = e.p2.y - e.p1.y;

    let c_x = r.0 - e.p1.x;
    let c_y = r.1 - e.p1.y;

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
fn crosses(r: (f64, f64), e: &Edge) -> bool {
    // reference point is above the edge so a horizontal line to the point
    // cannot crosse the edge
    if r.1 > e.p1.y.max(e.p2.y) {
        return false;
    }

    // reference point is below the edge so a horizontal line to the point
    // cannot crosse the edge
    if r.1 < e.p1.y.min(e.p2.y) {
        return false;
    }

    if e.p1.y < e.p2.y {
        // upward edge
        a_z(r, e) < 0.0
    } else {
        // downward edge
        a_z(r, e) > 0.0
    }
}
