// a point in space, with an index
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
