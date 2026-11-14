use crate::register_problem;
use crate::utils::epsilon::equal_epsilon;
use std::f64;

register_problem!(264, "Triangle Centres", problem264);

fn f(xa: i64, xb: i64) -> f64 {
    let xa = xa as f64;
    let xb = xb as f64;
    -f64::sqrt(
        (-xa * xa
            + 2. * f64::sqrt(
                xa * xa * xa * xa + 2. * xa * xa * xa * xb - 10. * xa * xa * xa
                    + 3. * xa * xa * xb * xb
                    - 50. * xa * xa * xb
                    + 125. * xa * xa
                    + 2. * xa * xb * xb * xb
                    - 50. * xa * xb * xb
                    + 300. * xa * xb
                    - 500. * xa
                    + xb * xb * xb * xb
                    - 10. * xb * xb * xb
                    + 125. * xb * xb
                    - 500. * xb
                    + 625.,
            )
            + (2. * xa * xb)
            - (10. * xa)
            + (2. * xb * xb)
            - (10. * xb)
            + 25.)
            / 3.,
    )
}

fn norm(xa: i64, xb: i64) -> i64 {
    xa * xa + xb * xb
}

fn to_integer(f: f64) -> Option<i64> {
    let fi = f.round();
    if equal_epsilon(f, fi) {
        return Some(fi as i64);
    }
    None
}

fn perimeter(xa: i64, ya: i64, xb: i64, yb: i64, xc: i64, yc: i64) -> f64 {
    f64::hypot((xa - xb) as f64, (ya - yb) as f64)
        + f64::hypot((xa - xc) as f64, (ya - yc) as f64)
        + f64::hypot((xb - xc) as f64, (yb - yc) as f64)
}

/// Consider all the triangles having:
///
/// All their vertices on lattice points.
/// Circumcentre at the origin O.
/// Orthocentre at the point H(5, 0).
/// There are nine such triangles having a perimeter ≤ 50.
/// Listed and shown in ascending order of their perimeter, they are:
///
///      A(-4, 3), B(5, 0), C(4, -3)
///      A(4, 3), B(5, 0), C(-4, -3)
///      A(-3, 4), B(5, 0), C(3, -4)
///
///      A(3, 4), B(5, 0), C(-3, -4)
///      A(0, 5), B(5, 0), C(0, -5)
///      A(1, 8), B(8, -1), C(-4, -7)
///
///      A(8, 1), B(1, -8), C(-4, 7)
///      A(2, 9), B(9, -2), C(-6, -7)
///      A(9, 2), B(2, -9), C(-6, 7)
///
/// The sum of their perimeters, rounded to four decimal places, is 291.0089.
///
/// Find all such triangles with a perimeter ≤ 10^5.
/// Enter as your answer the sum of their perimeters rounded to four decimal places.
pub fn problem264() -> String {
    let limite = 100000.0;

    // xa² + ya² = xb² + yb² = xc² + yc², xa + xb + xc = 5, ya + yb + yc = 0

    let mut result = 0.;

    let gr = 16000;
    let mut xa = 0;
    while xa + gr > 0 {
        for xb in xa..=(-xa + 5) / 2 {
            let fya = f(xa, xb);
            if let Some(ya) = to_integer(fya) {
                let fyb = ((xa * xa + ya * ya - xb * xb) as f64).sqrt();
                if let Some(yb) = to_integer(fyb) {
                    let yc = -ya - yb;
                    let xc = -xa - xb + 5;
                    let p = perimeter(xa, ya, xb, yb, xc, yc);
                    if norm(xa, ya) == norm(xb, yb) && norm(xb, yb) == norm(xc, yc) && p <= limite {
                        if (xa != xb || ya != yb)
                            && (xc != xb || yc != yb)
                            && (xa != xc || ya != yc)
                        {
                            if (xa == xb && ya == -yb && yc == 0)
                                || (xb == xc && yb == -yc && ya == 0)
                            {
                                result += p;
                            } else {
                                result += 2. * p;
                            }
                        }
                    }
                }
            }
        }
        xa -= 1;
    }

    format!("{:.4}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem264() {
        assert_eq!(problem264(), "2816417.1055");
    }
}
