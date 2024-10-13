#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::distance;
use crate::intersections;

// a polygon point
// x and y are coordinates
// h is added to the distance to this point for custom distances
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub h: f64,
    pub index: usize,
}

// edge connects two points
#[derive(Debug, Clone)]
pub struct Edge {
    pub p1: Point,
    pub p2: Point,
}

// node is a box which has dimensions
// it contains either other nodes
// or it contains edges
#[cfg_attr(feature = "pyo3", pyclass)]
#[derive(Clone)]
pub struct Node {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub hmin: f64,
    pub children_nodes: Vec<Node>,
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
        self.children_nodes.push(*boxed_node);
    }
    fn insert_edge(&mut self, new_edge: Edge) {
        self.edges.push(new_edge);
    }
}

#[cfg_attr(feature = "pyo3", pyfunction)]
pub fn build_search_tree(
    polygons: Vec<Vec<(f64, f64)>>,
    num_edges_children: usize,
    num_nodes_children: usize,
) -> Vec<Node> {
    let polygons_h = pad(polygons);
    build_search_tree_h(polygons_h, num_edges_children, num_nodes_children)
}

#[cfg_attr(feature = "pyo3", pyfunction)]
pub fn build_search_tree_h(
    polygons: Vec<Vec<(f64, f64, f64)>>,
    num_edges_children: usize,
    num_nodes_children: usize,
) -> Vec<Node> {
    let mut nodes = Vec::new();

    let mut offset = 0;
    for polygon in polygons {
        // group edges to nodes, num_edges_children at the time
        nodes.append(&mut group_edges(
            num_edges_children,
            points_to_edges(&polygon, offset),
        ));

        offset += polygon.len();
    }

    // we group nodes into a tree
    while nodes.len() > 1 {
        nodes = group_nodes(num_nodes_children, nodes);
    }

    nodes
}

fn pad(input: Vec<Vec<(f64, f64)>>) -> Vec<Vec<(f64, f64, f64)>> {
    let mut output = Vec::new();
    for v in input {
        output.push(v.iter().map(|(x, y)| (*x, *y, 0.0)).collect());
    }
    output
}

pub fn points_are_inside(tree: &Tree, points: &[(f64, f64)]) -> Vec<bool> {
    // point is inside some polygon if the number of intersections to reach
    // the point "from left" is impair

    #[cfg(feature = "rayon")]
    let iter = points.par_iter();

    #[cfg(not(feature = "rayon"))]
    let iter = points.iter();

    iter.map(|p| (intersections::num_intersections(&tree[0], 0, *p) % 2) != 0)
        .collect()
}

pub fn distances_nearest_edges(tree: &Tree, points: &[(f64, f64)]) -> Vec<f64> {
    let large_number = f64::MAX;

    #[cfg(feature = "rayon")]
    let iter = points.par_iter();

    #[cfg(not(feature = "rayon"))]
    let iter = points.iter();

    iter.map(|p| distance::get_distance_edge(&tree[0], large_number, *p))
        .collect()
}

pub fn distances_nearest_vertices(tree: &Tree, points: &[(f64, f64)]) -> (Vec<usize>, Vec<f64>) {
    let large_number = f64::MAX;

    #[cfg(feature = "rayon")]
    let iter = points.par_iter();

    #[cfg(not(feature = "rayon"))]
    let iter = points.iter();

    let tuples: Vec<(usize, f64)> = iter
        .map(|p| distance::get_distance_vertex(&tree[0], 0, large_number, *p))
        .collect();

    let mut indices = Vec::new();
    let mut distances = Vec::new();

    for (i, d) in tuples {
        indices.push(i);
        distances.push(d);
    }

    (indices, distances)
}

fn group_nodes(num_nodes_children: usize, input: Vec<Node>) -> Vec<Node> {
    let num_input = input.len();
    let n = num_input / num_nodes_children;
    let num_parents = match num_input % num_nodes_children {
        0 => n,
        _ => n + 1,
    };

    let large_number = f64::MAX;

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

fn group_edges(num_edges_children: usize, input: Vec<Edge>) -> Tree {
    let num_input = input.len();
    let n = num_input / num_edges_children;
    let num_parents = match num_input % num_edges_children {
        0 => n,
        _ => n + 1,
    };

    let large_number = f64::MAX;

    let mut parents = Vec::new();

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
                    input[i].p1.h,
                );
                new_parent.adjust_bounds(
                    input[i].p2.x,
                    input[i].p2.x,
                    input[i].p2.y,
                    input[i].p2.y,
                    input[i].p2.h,
                );
                new_parent.insert_edge(input[i].clone());
                i += 1;
            }
        }
        parents.push(new_parent);
    }

    parents
}

fn points_to_edges(points: &[(f64, f64, f64)], offset: usize) -> Vec<Edge> {
    let mut edges: Vec<Edge> = points
        .windows(2)
        .enumerate()
        .map(|(i, t)| Edge {
            p1: Point {
                x: t[0].0,
                y: t[0].1,
                h: t[0].2,
                index: offset + i,
            },
            p2: Point {
                x: t[1].0,
                y: t[1].1,
                h: t[1].2,
                index: offset + i + 1,
            },
        })
        .collect();

    let n = points.len() - 1;
    edges.push(Edge {
        p1: Point {
            x: points[n].0,
            y: points[n].1,
            h: points[n].2,
            index: offset + n,
        },
        p2: Point {
            x: points[0].0,
            y: points[0].1,
            h: points[0].2,
            index: offset,
        },
    });

    edges
}
