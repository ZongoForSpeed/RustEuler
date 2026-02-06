use crate::register_problem;

register_problem!(246, "Tangents to an ellipse", problem246);

struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

struct Ellipse {
    a2: f64,
    b2: f64,
    c: Point,
}

impl Ellipse {
    fn new(a2: f64, b2: f64, c: Point) -> Self {
        Self { a2, b2, c }
    }

    fn in_ellipse(&self, x: i64, y: i64) -> bool {
        // x^2/a^2 + y^2/b^2 - 1 <= 0
        // x^2*b^2 + y^2*a^2 - a^2*b^2 <= 0
        let dx = x as f64 - self.c.x as f64;
        let dy = y as f64 - self.c.y as f64;
        dx * dx / self.a2 + dy * dy / self.b2 <= 1.0
    }

    fn keep(&self, mut ox: i64, mut oy: i64) -> bool {
        // change coordinates
        ox -= self.c.x;
        oy -= self.c.y;

        let x = ox as f64;
        let y = oy as f64;

        let x0: f64;
        let y0: f64;
        let x1: f64;
        let y1: f64;

        if ox == 0 {
            y0 = self.b2 / y as f64;
            y1 = y0;
            x0 = (self.a2 * (1. - y0 * y0 / self.b2)).sqrt();
            x1 = -x0;
        } else if oy == 0 {
            x0 = self.a2 / x;
            x1 = x0;
            y0 = (self.b2 * (1. - x0 * x0 / self.a2)).sqrt();
            y1 = -y0;
        } else if ox.abs() < oy.abs() {
            // 2nd degree equation - looking for y
            let a = self.a2 * self.a2 * y * y / (self.b2 * self.b2 * x * x) + self.a2 / self.b2;
            let b = -2. * self.a2 * self.a2 * y / (self.b2 * x * x);
            let c = self.a2 * self.a2 / (x * x) - self.a2;

            (y0, y1) = Self::solve(a, b, c);

            x0 = (1. - y * y0 / self.b2) * self.a2 / x;
            x1 = (1. - y * y1 / self.b2) * self.a2 / x;
        } else {
            // 2nd degree equation - looking for x
            let a = self.b2 * self.b2 * x * x / (self.a2 * self.a2 * y * y) + self.b2 / self.a2;
            let b = -2. * self.b2 * self.b2 * x / (self.a2 * y * y);
            let c = self.b2 * self.b2 / (y * y) - self.b2;

            (x0, x1) = Self::solve(a, b, c);

            y0 = (1. - x * x0 / self.a2) * self.b2 / y;
            y1 = (1. - x * x1 / self.a2) * self.b2 / y;
        }

        // scalar product
        let norm0 = f64::hypot(x - x0, y - y0);
        let norm1 = f64::hypot(x - x1, y - y1);

        let prod = ((x - x0) * (x - x1) + (y - y0) * (y - y1)) / (norm0 * norm1);

        prod < f64::sqrt(1. / 2.)
    }

    fn solve(a: f64, b: f64, c: f64) -> (f64, f64) {
        let delta = b * b - 4. * a * c;

        let y0 = (-b + delta.sqrt()) / (2. * a);
        let y1 = (-b - delta.sqrt()) / (2. * a);

        (y0, y1)
    }

    fn algorithm(&self) -> i64 {
        let mut y = self.c.y;
        let mut x_min = self.c.x;
        while self.in_ellipse(x_min, y) {
            x_min += 1;
        }

        let mut x_max = x_min;
        while self.keep(x_max + 1, y) {
            x_max += 1;
        }

        let mut result = 0;

        loop {
            if y == self.c.y {
                // counted twice
                result += (x_max - x_min + 1) * 2;
            } else {
                // counted 4 times
                result += (x_max - x_min + 1) * 4;

                // except for y axis (count twice)
                if x_min == self.c.x {
                    result -= 2;
                }
            }

            y += 1;

            while x_min > self.c.x && !self.in_ellipse(x_min - 1, y) {
                x_min -= 1;
            }

            while x_max >= self.c.x && !self.keep(x_max, y) {
                x_max -= 1;
            }

            if x_min >= x_max {
                break;
            }
        }
        result
    }
}

/// A definition for an ellipse is:
/// Given a circle c with centre M and radius r and a point G such that d(G,M)<r,
/// the locus of the points that are equidistant from c and G form an ellipse.
///
/// The construction of the points of the ellipse is shown below.
///
/// Given are the points M(-2000,1500) and G(8000,1500).
/// Given is also the circle c with centre M and radius 15000.
/// The locus of the points that are equidistant from G and c form an ellipse e.
/// From a point P outside e the two tangents t1 and t2 to the ellipse are drawn.
/// Let the points where t1 and t2 touch the ellipse be R and S.
///
/// For how many lattice points P is angle RPS greater than 45 degrees?
pub fn problem246() -> String {
    let e = Ellipse::new(7500. * 7500., 2500. * 12500., Point::new(3000, 1500));
    e.algorithm().to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem246::problem246;

    #[test]
    fn test_problem246() {
        let result = problem246();
        assert_eq!(result, "810834388");
    }
}
