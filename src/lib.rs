//! Computes distances to polygon edges and vertices and can check whether points are
//! inside/outside polygons.

mod distance;
mod intersections;
mod point;
mod python;
mod tree;

pub use crate::tree::Tree;

pub use crate::tree::build_search_tree;
pub use crate::tree::build_search_tree_h;

pub use crate::tree::distances_nearest_edges;
pub use crate::tree::distances_nearest_vertices;
pub use crate::tree::points_are_inside;
