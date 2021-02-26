use crate::tree::Node;

fn box_distance(p: (f64, f64), node: &Node) -> f64 {
    let difx = if p.0 < node.xmin {
        p.0 - node.xmin
    } else if p.0 > node.xmax {
        p.0 - node.xmax
    } else {
        0.0
    };

    let dify = if p.1 < node.ymin {
        p.1 - node.ymin
    } else if p.1 > node.ymax {
        p.1 - node.ymax
    } else {
        0.0
    };

    distance(difx, dify)
}

pub fn get_distance_edge(node: &Node, d: f64, p: (f64, f64)) -> f64 {
    if box_distance(p, &node) > d {
        return d;
    }

    let mut d_min = d;

    if !node.children_nodes.is_empty() {
        for child_node in node.children_nodes.iter() {
            let temp = get_distance_edge(&child_node, d_min, p);
            d_min = d_min.min(temp);
        }
        return d_min;
    }

    if !node.edges.is_empty() {
        for edge in &node.edges {
            d_min = d_min.min(dsegment(
                p.0, p.1, edge.p1.x, edge.p1.y, edge.p2.x, edge.p2.y,
            ));
        }
        return d_min;
    }

    d_min
}

pub fn get_distance_vertex(node: &Node, d: f64, p: (f64, f64)) -> f64 {
    if box_distance(p, &node) + node.hmin > d {
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
        for edge in &node.edges {
            let t = distance(edge.p1.x - p.0, edge.p1.y - p.1);
            d_min = d_min.min(t + edge.p1.h);
        }

        let edge = node.edges.last().unwrap();
        let t = distance(edge.p2.x - p.0, edge.p2.y - p.1);
        d_min = d_min.min(t + edge.p2.h);

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
