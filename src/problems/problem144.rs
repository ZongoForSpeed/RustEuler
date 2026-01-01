use crate::register_problem;
use std::fmt::{Display, Formatter};

register_problem!(
    144,
    "Investigating multiple reflections of a laser beam",
    problem144
);

#[derive(Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x={}, y={})", self.x, self.y)
    }
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

fn next(a: Point, b: Point) -> Point {
    let m = f64::tan(-f64::atan((a.y - b.y) / (a.x - b.x)) + 2. * f64::atan(-4. * b.x / b.y));
    let c = b.y - m * b.x;

    // solution of 4x^2 + (mx+c)^2 = 100:
    let x1 = (-2. * m * c + f64::sqrt(400. * m * m + 1600. - 16. * c * c)) / (8. + 2. * m * m);
    let x2 = (-2. * m * c - f64::sqrt(400. * m * m + 1600. - 16. * c * c)) / (8. + 2. * m * m);

    if f64::abs(b.x - x1) < 0.0001 {
        Point::new(x2, m * x2 + c)
    } else {
        Point::new(x1, m * x1 + c)
    }
}

pub fn problem144() -> String {
    // In laser physics, a "white cell" is a mirror system that acts as a delay line for the laser beam.
    // The beam enters the cell, bounces around on the mirrors, and eventually works its way back out.
    //
    // The specific white cell we will be considering is an ellipse with the equation 4x2 + y2 = 100
    //
    // The section corresponding to −0.01 ≤ x ≤ +0.01 at the top is missing, allowing the light to enter and exit
    // through the hole.
    //
    // The light beam in this problem starts at the point (0.0,10.1) just outside the white cell, and the beam first
    // impacts the mirror at (1.4,-9.6).
    //
    // Each time the laser beam hits the surface of the ellipse, it follows the usual law of reflection "angle of
    // incidence equals angle of reflection." That is, both the incident and reflected beams make the same angle with
    // the normal line at the point of incidence.
    //
    // In the figure on the left, the red line shows the first two points of contact between the laser beam and the wall
    // of the white cell; the blue line shows the line tangent to the ellipse at the point of incidence of the first
    // bounce.
    //
    // The slope m of the tangent line at any point (x,y) of the given ellipse is: m = −4x/y
    //
    // The normal line is perpendicular to this tangent line at the point of incidence.
    //
    // The animation on the right shows the first 10 reflections of the beam.
    //
    // How many times does the beam hit the internal surface of the white cell before exiting?
    let mut a = Point::new(0.0, 10.1);
    let mut b = Point::new(1.4, -9.6);

    let mut result = 0;
    while !(f64::abs(b.x) < 0.01 && b.y > 0.) {
        // println!("{a} -> {b}");
        (a, b) = (b.clone(), next(a, b));
        result += 1;
    }
    result.to_string()
}
