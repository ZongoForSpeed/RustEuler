use crate::register_problem;

register_problem!(255, "Rounded Square Roots", problem255);

fn floor(a: i64, b: i64) -> i64 {
    a/b
}

fn ceil(a: i64, b: i64) -> i64 {
    (a+b-1)/b
}

fn f(n:i64, x: i64) -> i64 {
    floor(x + ceil(n, x), 2)
}

fn algorithm(left: i64, right: i64, x: i64) -> f64 {
    if f(left, x) == x && f(right, x) == x {
        return 1.;
    }

    if f(left, x) == f(right, x) {
        return 1. + algorithm(left, right, f(left, x));
    }

    let mut result = 0.;
    let mut sum = 0;

    for k in ceil(left, x).. {
        let k1 = std::cmp::max((k - 1) * x + 1, left);
        let k2 = std::cmp::min(k * x, right);
        if k1 > k2 {
            break;
        }
        if f(k1, x) != x {
            result += (k2 - k1 + 1) as f64 * algorithm(k1, k2, f(k1, x));
        }
        sum += k2 - k1 + 1;
    }
    result = 1. + result / sum as f64;
    result
}

/// We define the rounded-square-root of a positive integer n as the square root of n rounded to the nearest integer.
///
/// The following procedure (essentially Heron's method adapted to integer arithmetic) finds the rounded-square-root
/// of n:
///
/// Let d be the number of digits of the number n.
/// If d is odd, set x0 = 2×10^(d-1)⁄2.
/// If d is even, set x0 = 7×10^(d-2)⁄2.
/// Repeat:
///
///                 xk+1 = floor(xk + ceil(n / xk), 2)
///
/// until xk+1 = xk.
///
/// As an example, let us find the rounded-square-root of n = 4321.
/// n has 4 digits, so x0 = 7×10(4-2)⁄2 = 70.
///
/// x1 = floor(70 + ceil(4321 / 70), 2) = 66
/// x2 = floor(66 + ceil(4321 / 66), 2) = 66
///
/// Since x2 = x1, we stop here.
/// So, after just two iterations, we have found that the rounded-square-root of 4321 is 66 (the actual square root
/// is 65.7343137…).
///
/// The number of iterations required when using this method is surprisingly low.
/// For example, we can find the rounded-square-root of a 5-digit integer (10,000 ≤ n ≤ 99,999) with an average of
/// 3.2102888889 iterations (the average value was rounded to 10 decimal places).
///
/// Using the procedure described above, what is the average number of iterations required to find the
/// rounded-square-root of a 14-digit number (10^13 ≤ n < 10^14)?
/// Give your answer rounded to 10 decimal places.
///
/// Note: The symbols ⌊x⌋ and ⌈x⌉ represent the floor function and ceiling function respectively.
pub fn problem255() -> String {
    let result = algorithm(10000000000000, 99999999999999, 7000000);

    format!("{:.10}", result)
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem255::problem255;

    #[test]
    fn test_problem255() {
        let result = problem255();
        assert_eq!(result, "4.4474011180");
    }
}
