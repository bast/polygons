// we compute the sqrt at the very end to save time
pub fn distance_squared(x: f64, y: f64) -> f64 {
    return x * x + y * y;
}

// this is derived from a C/C++ code
// Copyright (C) 2004-2012 Per-Olof Persson
pub fn dsegment(x0: f64, y0: f64, p1x: f64, p1y: f64, p2x: f64, p2y: f64) -> f64 {
    let v = (p2x - p1x, p2y - p1y);
    let w = (x0 - p1x, y0 - p1y);

    let c1 = v.0 * w.0 + v.1 * w.1;

    if c1 <= 0.0 {
        return distance_squared(x0 - p1x, y0 - p1y);
    }

    let c2 = v.0 * v.0 + v.1 * v.1;

    if c1 >= c2 {
        return distance_squared(x0 - p2x, y0 - p2y);
    } else {
        return distance_squared(x0 - (p1x + c1 / c2 * v.0), y0 - (p1y + c1 / c2 * v.1));
    }
}
