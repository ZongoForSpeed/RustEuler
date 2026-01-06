use crate::maths::arithmetic::Arithmetic;
use crate::register_problem;
use crate::utils::epsilon::{compare_epsilon, equal_epsilon};
use num_integer::Roots;
use std::cmp::Ordering;

register_problem!(184, "Triangles containing the origin", problem184);

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        equal_epsilon(other.x, self.x) && equal_epsilon(other.y, self.y)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if !equal_epsilon(self.x, other.x) {
            compare_epsilon(self.x, other.x)
        } else {
            compare_epsilon(self.y, other.y)
        }
    }
}

pub fn problem184() -> String {
    // Consider the set Ir of points (x,y) with integer co-ordinates in the interior of the circle  with radius r,
    // centered at the origin, i.e. x2 + y2 < r2.
    //
    // For a radius of 2, I2 contains the nine points (0,0), (1,0), (1,1), (0,1), (-1,1), (-1,0), (-1,-1), (0,-1) and
    // (1,-1). There are eight triangles having all three vertices in I2 which contain the origin in the interior. Two
    // of them are shown below, the others are obtained from these by rotation.
    //
    // For a radius of 3, there are 360 triangles containing the origin in the interior and having all vertices in I3
    // and for I5 the number is 10600.
    //
    // How many triangles are there containing the origin in the interior and having all three vertices in I105?
    let limit = 105;

    let mut points:Vec<(Point, u64)> = Vec::new();
    let ll = limit * limit;
    for x in 0..limit {
        for y in 1..limit {
            let r2 = x * x + y * y;
            if r2 > 0 && r2 < ll && u64::gcd(x, y) == 1 {
                points.push((Point::new(x as f64, y as f64), ((ll - 1) / r2).sqrt()));
            }
        }
    }

    points.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let mut result = 0;
    let len = points.len();
    for i in 0..points.len() {
        let mut count = 0;
        for j in (i+1..).take_while(|&j| j < i + 2 * len) {
            result += points[i].1 * points[j % len].1 * count;
            count += points[j % len].1;
        }
    }

    result = result * 4 / 3;
    result.to_string()
}
