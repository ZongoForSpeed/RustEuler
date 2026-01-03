use crate::register_problem;

use num_traits::FloatConst;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet};

const EPSILON: f64 = 1e-9;

fn equal_epsilon(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[derive(Debug, Copy, Clone)]
struct EpsF64(f64);

impl From<f64> for EpsF64 {
    fn from(value: f64) -> Self {
        assert!(!value.is_nan(), "NaN not allowed in EpsF64");
        EpsF64(value)
    }
}

impl PartialEq for EpsF64 {
    fn eq(&self, other: &Self) -> bool {
        equal_epsilon(self.0, other.0)
    }
}

impl Eq for EpsF64 {}

impl PartialOrd for EpsF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EpsF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.0;
        let b = other.0;

        if equal_epsilon(a, b) {
            Ordering::Equal
        } else if a < b {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }
}

fn to_rad(deg: f64) -> f64 {
    f64::PI() * deg / 180.0
}

fn to_deg(rad: f64) -> f64 {
    rad * 180. / f64::PI()
}

fn to_usize(x: f64) -> usize {
    (x * 10000.).round() as usize
}

register_problem!(177, "Integer angled Quadrilaterals", problem177);

pub fn problem177() -> String {
    // Let ABCD be a convex quadrilateral, with diagonals AC and BD. At each vertex the diagonal makes an angle with
    // each of the two sides, creating eight corner angles.
    //
    // For example, at vertex A, the two angles are CAD, CAB.
    //
    // We call such a quadrilateral for which all eight corner angles have integer values when measured in degrees an
    // "integer angled quadrilateral". An example of an integer angled quadrilateral is a square, where all eight corner
    // angles are 45°. Another example is given by DAC = 20°, BAC = 60°, ABD = 50°, CBD = 30°, BCA = 40°, DCA = 30°,
    // CDB = 80°, ADB = 50°.
    //
    // What is the total number of non-similar integer angled quadrilaterals?
    //
    // Note: In your calculations you may assume that a calculated angle is integral if it is within a tolerance of 10-9
    // of an integer value.
    let mut sinus = vec![0.; 180];
    let mut cosinus = vec![0.; 180];
    let mut rad = vec![0.; 180];
    let mut prevalue = vec![false; 10000 + 1];

    let mut solution = vec![0; 8 + 1];

    let mut fs: BTreeSet<EpsF64> = BTreeSet::new();

    for i in 1..180 {
        rad[i] = to_rad(i as f64);
        sinus[i] = rad[i].sin();
        cosinus[i] = rad[i].cos();
        prevalue[to_usize(sinus[i])] = true;
        fs.insert(sinus[i].into());
    }

    prevalue[10000] = true; // sin90
    prevalue[10000 - 1] = true; // sin90
    prevalue[5000] = true; // sin30
    prevalue[5000 - 1] = true; //sin30

    for a in 1..=45 {
        for b in a..180 - 1 - a {
            for c in 1..180 - a - b {
                let d = 180 - a - b - c;
                for e in 1..b + d {
                    let f = b + d - e;
                    let m = (sinus[b] * sinus[c] * sinus[f]) / (sinus[a] * sinus[e] * sinus[d]);
                    let n = cosinus[a + c];
                    let mut sin_y = ((1. - n * n) / (m * m + 2. * m * n + 1.)).sqrt();
                    let j = to_usize(sin_y);
                    if !prevalue[j] {
                        continue;
                    }
                    if fs.contains(&sin_y.into()) {
                        if sin_y > 1. {
                            sin_y = 1.;
                        }

                        let mut y = (to_deg(sin_y.asin()) + 0.01).round() as usize;
                        let sin_x = m * sin_y;
                        let x_angle = if equal_epsilon(sin_x, 1.) {
                            90.
                        } else {
                            to_deg(sin_x.asin())
                        };
                        let mut x = (x_angle + 0.01).round() as usize;
                        if equal_epsilon(x_angle, x as f64) {
                            x = if 180 - x + y == a + c { 180 - x } else { x };
                            y = if 180 + x - y == a + c { 180 - y } else { y };

                            let quadrilaterals = [
                                [a, b, d, y, x, f, e, c],
                                [d, y, x, f, e, c, a, b],
                                [x, f, e, c, a, b, d, y],
                                [e, c, a, b, d, y, x, f],
                                [b, a, c, e, f, x, y, d],
                                [c, e, f, x, y, d, b, a],
                                [f, x, y, d, b, a, c, e],
                                [y, d, b, a, c, e, f, x],
                            ];

                            let mut s = HashSet::new();
                            for q in quadrilaterals {
                                if q[0] <= q[1] && q[0] <= 45 {
                                    s.insert(q.to_vec());
                                }
                            }

                            solution[s.len()] += 1;
                        };
                    }
                }
            }
        }
    }

    solution
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, &v)| v / i)
        .sum::<usize>()
        .to_string()
}
