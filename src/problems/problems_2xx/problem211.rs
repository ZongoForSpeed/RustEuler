use crate::register_problem;
use crate::maths::polygonal::Polygonal;

register_problem!(211, "Divisor Square Sum", problem211);

pub fn problem211() -> String {
    // For a positive integer n, let σ2(n) be the sum of the squares of its divisors. For example,
    //
    // σ2(10) = 1 + 4 + 25 + 100 = 130.
    // Find the sum of all n, 0 < n < 64,000,000 such that σ2(n) is a perfect square.
    let limit = 64000000;
    let mut result = 1;

    let mut dp = vec![1; limit];
    for d in 2..limit {
        let d2 = d * d;
        for q in (d..limit).step_by(d) {
            dp[q] += d2;
        }

        if dp[d].is_square() {
            result += d;
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem211::problem211;

    #[test]
    fn test_problem211() {
        let result = problem211();
        assert_eq!(result, "1922364685");
    }
}
