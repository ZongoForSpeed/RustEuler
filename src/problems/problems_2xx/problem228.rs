use crate::register_problem;
use itertools::Itertools;
use num_traits::FloatConst;
use std::cmp::Ordering;
use std::collections::BTreeSet;

register_problem!(228, "Minkowski Sums", problem228);

struct Coordinate {
    x: f64,
    y: f64,
}

impl Coordinate {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn norm(&self) -> f64 {
        f64::hypot(self.x, self.y)
    }

    fn dot_product(p1: &Coordinate, p2: &Coordinate) -> f64 {
        (p1.x * p2.x + p1.x * p2.x) / (p1.norm() * p2.norm())
    }

    fn normalize(&self) -> Coordinate {
        let n = self.norm();
        // self.x /= n;;
        // self.y /= n;;
        Self {
            x: self.x / n,
            y: self.y / n,
        }
    }

    fn minus(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn round(self, n: i32) -> Coordinate {
        Self {
            x: round(self.x, n),
            y: round(self.y, n),
        }
    }
}

impl Eq for Coordinate {}

impl PartialEq<Self> for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        let product = Coordinate::dot_product(self, other);
        if (1. - product).abs() < 0.0000000000001 {
            return Ordering::Equal;
        }

        match cmp(self.x, other.x) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => cmp(self.y, other.y),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

fn cmp(f1: f64, f2: f64) -> Ordering {
    if f1 < f2 {
        Ordering::Less
    } else if f1 > f2 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn round(f: f64, n: i32) -> f64 {
    let x = f64::powi(10.0, n);
    (f * x).round() / x
}

fn convert(n: i64) -> Vec<Coordinate> {
    let v = s(n);
    if v.is_empty() {
        return v;
    }

    let len = v.len();
    let mut result =
        vec![v[0].minus(&v[len - 1]).normalize()];

    for (i, j) in v.iter().tuple_windows() {
        let p = j.minus(i).normalize();
        result.push(p);
    }
    result
}

fn s(n: i64) -> Vec<Coordinate> {
    let n_f = n as f64;

    let mut result = Vec::new();
    for k in 0..n {
        let x = (2 * k - 1) as f64 / n_f + f64::PI();
        result.push(Coordinate::new(x.cos(), x.sin()));
    }
    result
}

pub fn problem228() -> String {
    // Let Sn be the regular n-sided polygon – or shape – whose vertices vk (k = 1,2,…,n) have
    // coordinates:
    //
    // 			x_k = cos( 2*(k-1)/n × 180° )
    // 			y_k = sin( 2*(k-1)/n × 180° )
    //
    // Each Sn is to be interpreted as a filled shape consisting of all points on the perimeter and
    // in the interior.
    //
    // The Minkowski sum, S+T, of two shapes S and T is the result of adding every point in S to
    // every point in T, where point addition is performed coordinate-wise:
    // (u, v) + (x, y) = (u+x, v+y).
    //
    // For example, the sum of S3 and S4 is the six-sided shape shown in pink below:
    //
    //							picture showing S_3 + S_4
    //
    // How many sides does S1864 + S1865 + … + S1909 have?
    let mut minkowski = BTreeSet::new();

    for d in 1864..=1909 {
        let coord = convert(d);
        for c in coord {
            minkowski.insert(c.round(10));
        }
    }

    minkowski.len().to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem228::problem228;

    #[test]
    fn test_problem228() {
        let result = problem228();
        assert_eq!(result, "86226");
    }
}
