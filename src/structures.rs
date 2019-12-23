// a point in space
#[derive(Clone)]
pub struct IndexPoint {
    pub index: usize,
    pub x: f64,
    pub y: f64,
}

// edge connects two points
#[derive(Clone)]
pub struct Edge {
    pub p1: IndexPoint,
    pub p2: IndexPoint,
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
