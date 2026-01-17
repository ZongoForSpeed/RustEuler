use crate::maths::arithmetic::Arithmetic;
use crate::register_problem;
use num_traits::FloatConst;

register_problem!(183, "Maximum product of parts", problem183);

fn infinity(n: i64, mut k: i64) -> bool {
    k /= i64::gcd(n, k);
    while k % 2 == 0 {
        k /= 2;
    }
    while k % 5 == 0 {
        k /= 5;
    }
    k == 1
}

fn terminating(n: i64) -> bool {
    let k0 = n as f64 / f64::E();
    let k_max = k0.ceil();
    let k_min = k0.floor();

    let p_max = k_max * ((n as f64) / k_max).ln();
    let p_min = k_min * ((n as f64) / k_min).ln();
    if p_max > p_min {
        infinity(n, k_max as i64)
    } else {
        infinity(n, k_min as i64)
    }
}

pub fn problem183() -> String {
    // Let N be a positive integer and let N be split into k equal parts, r = N/k, so that N = r + r + ... + r.
    // Let P be the product of these parts, P = r × r × ... × r = rk.
    //
    // For example, if 11 is split into five equal parts, 11 = 2.2 + 2.2 + 2.2 + 2.2 + 2.2,
    // then P = 2.2**5 = 51.53632.
    //
    // Let M(N) = Pmax for a given value of N.
    //
    // It turns out that the maximum for N = 11 is found by splitting eleven into four equal parts
    // which leads to Pmax = (11/4)4; that is, M(11) = 14641/256 = 57.19140625, which is a terminating decimal.
    //
    // However, for N = 8 the maximum is achieved by splitting it into three equal parts, so M(8) = 512/27,
    // which is a non-terminating decimal.
    //
    // Let D(N) = N if M(N) is a non-terminating decimal and D(N) = -N if M(N) is a terminating decimal.
    //
    // For example, ΣD(N) for 5 ≤ N ≤ 100 is 2438.
    //
    // Find ΣD(N) for 5 ≤ N ≤ 10000.
    let limite = 10000;

    println!("terminating(8) = {}", terminating(8));
    println!("terminating(11) = {}", terminating(11));

    let mut result = 0;
    for n in 5..=limite {
        if terminating(n) {
            result -= n;
        } else {
            result += n;
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem183() {
        let result = problem183();
        assert_eq!(result, "48861552");
    }
}
