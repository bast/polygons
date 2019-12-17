use crate::structures::Edge;

// This code is based on http://geomalgorithms.com/a03-_inclusion.html
// which is distributed under the following license:

// Copyright 2000 softSurfer, 2012 Dan Sunday
// This code may be freely used and modified for any purpose
// providing that this copyright notice is included with it.
// SoftSurfer makes no warranty for this code, and cannot be held
// liable for any real or imagined damage resulting from its use.
// Users of this code must verify correctness for their application.

// tests if a point is left|on|right of an infinite line
// input:  three points p0, p1, and p2
// return: > 0 for p2 left of the line through p0 and p1
//         = 0 for p2 on the line
//         < 0 for p2 right of the line
fn is_left(p0x: f64, p0y: f64, p1x: f64, p1y: f64, p2x: f64, p2y: f64) -> f64 {
    return (p1x - p0x) * (p2y - p0y) - (p2x - p0x) * (p1y - p0y);
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
        return is_left(e.p1.x, e.p1.y, e.p2.x, e.p2.y, px, py) > 0.0;
    } else {
        // downward edge
        return is_left(e.p1.x, e.p1.y, e.p2.x, e.p2.y, px, py) < 0.0;
    }
}
