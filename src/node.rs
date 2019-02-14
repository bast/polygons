use crate::distance::distance_squared;
use crate::distance::dsegment;
use crate::intersection::crosses;
use crate::structures::Node;
use crate::structures::Point;

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
            if crosses(p.x, p.y, &edge) {
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
