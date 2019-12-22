use crate::structures::Edge;

// This code is based on http://geomalgorithms.com/a03-_inclusion.html
// which is distributed under the following license:

// Copyright 2000 softSurfer, 2012 Dan Sunday
// This code may be freely used and modified for any purpose
// providing that this copyright notice is included with it.
// SoftSurfer makes no warranty for this code, and cannot be held
// liable for any real or imagined damage resulting from its use.
// Users of this code must verify correctness for their application.

// a_z is one component of the vector cross product
// a_z < 0 for r right of the (upward) line p0-p2
// a_z > 0 for r left of the (upward) line p0-p2
// a_z = 0 if r lies on the line p0-p2
fn a_z(p1x: f64, p1y: f64, p2x: f64, p2y: f64, rx: f64, ry: f64) -> f64 {
    let b_x = p2x - p1x;
    let b_y = p2y - p1y;
    let c_x = rx - p1x;
    let c_y = ry - p1y;
    return b_x * c_y - b_y * c_x;
}

pub fn crosses(px: f64, py: f64, e: &Edge) -> bool {
    // point is above the edge so a horizontal line to the point
    // cannot crosse the edge
    if py > e.p1.y.max(e.p2.y) {
        return false;
    }

    // point is below the edge so a horizontal line to the point
    // cannot crosse the edge
    if py < e.p1.y.min(e.p2.y) {
        return false;
    }

    if e.p1.y < e.p2.y {
        // upward edge
        return a_z(e.p1.x, e.p1.y, e.p2.x, e.p2.y, px, py) > 0.0;
    } else {
        // downward edge
        return a_z(e.p1.x, e.p1.y, e.p2.x, e.p2.y, px, py) < 0.0;
    }
}
