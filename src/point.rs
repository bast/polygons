use std::num::ParseFloatError;
use std::str::FromStr;

// a point in space
#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub coeff: f64,
}

impl FromStr for Point {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split_whitespace().collect();

        let x_fromstr = coords[0].parse::<f64>()?;
        let y_fromstr = coords[1].parse::<f64>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
            coeff: 0.0,
        })
    }
}
