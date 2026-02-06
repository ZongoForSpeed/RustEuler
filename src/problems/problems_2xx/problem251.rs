use num_integer::Roots;
use crate::maths::arithmetic::Arithmetic;
use crate::register_problem;

register_problem!(251, "Cardano Triplets", problem251);

/// A triplet of positive integers (a,b,c) is called a Cardano Triplet if it
/// satisfies the condition:
///
///          cqrt(a + b.sqrt(c)) + cqrt(a - b.sqrt(c)) = 1
///
/// For example, (2,1,5) is a Cardano Triplet.
///
/// There exist 149 Cardano Triplets for which a+b+c ≤ 1000.
///
/// Find how many Cardano Triplets exist such that a+b+c ≤ 110,000,000.
pub fn problem251() -> String {
    let u = 110000000;

    let mut result = 0;

    let n_max = (u / 5).sqrt();
    for n in 1..=n_max {
        let d_max = (u - 5 * n * n - n).sqrt();
        for d in (1..=d_max).step_by(2) {
            if i64::gcd(n, d) != 1 {
                continue;
            }

            let (q, _, _) = i64::bezout(d, 8 * n);
            let mut l = -3 * q * q % (8 * n);
            l += if l < 0 {8 * n} else {0};
            let c = u + 1 - (3 * n + d) * (d * d * l + 3) / (8 * n) - n * n * l;
            if c > 0 {
                result += c / (8 * n * n * n + (3 * n + d) * d * d) + 1;
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem251::problem251;

    #[test]
    fn test_problem251() {
        let result = problem251();
        assert_eq!(result, "18946051");
    }
}
