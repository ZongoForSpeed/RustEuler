use crate::maths::arithmetic::Arithmetic;
use crate::register_problem;

register_problem!(
    195,
    "Inscribed circles of triangles with one angle of 60 degrees",
    problem195
);

fn area(a: f64, b: f64, c: f64) -> f64 {
    ((a + b + c) * (b + c - a) * (a + c - b) * (a + b - c)).sqrt() / 4.
}

pub fn problem195() -> String {
    // Let's call an integer sided triangle with exactly one angle of 60 degrees a 60-degree triangle.
    // Let r be the radius of the inscribed circle of such a 60-degree triangle.
    //
    // There are 1234 60-degree triangles for which r ≤ 100.
    // Let T(n) be the number of 60-degree triangles for which r ≤ n, so
    // T(100) = 1234,  T(1000) = 22767, and  T(10000) = 359912.
    //
    // Find T(1053779).
    let limite: u64 = 1053779;
    let borne = 2 * limite.isqrt();

    // a = m² - mn + n²
    // b = 2mn - n²
    // c = m² - n²
    // 0 < 2*n < m
    let f_limit = limite as f64;

    let mut result = 0;
    for n in 1..borne {
        for m in 2 * n + 1.. {
            if u64::gcd(n, m) == 1 {
                let mut a = m * m - m * n + n * n;
                let mut b = 2 * m * n - n * n;
                let mut c = m * m - n * n;
                let d = u64::gcd(a, u64::gcd(b, c));
                a /= d;
                b /= d;
                c /= d;

                // r = 2.S / (a+b+c)
                let r = 2. * area(a as f64, b as f64, c as f64) / (a + b + c) as f64;
                if r > 3. * f_limit {
                    break;
                }

                if r < f_limit {
                    result += (f_limit / r).floor() as u64;
                }
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem195() {
        let result = problem195();
        assert_eq!(result, "75085391");
    }
}
