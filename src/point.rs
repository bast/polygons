use std::num::ParseFloatError;
use std::str::FromStr;

// a polygon point
// x and y are coordinates
// h is added to the distance to this point for custom distances
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub h: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y, h: 0.0 }
    }
}

impl FromStr for Point {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split_whitespace().collect();

        let x = coords[0].parse()?;
        let y = coords[1].parse()?;

        Ok(Point::new(x, y))
    }
}
