use crate::maths::polygonal::Polygonal;
use crate::register_problem;
use num_integer::Roots;
use std::collections::HashSet;

register_problem!(261, "Pivotal Square Sums", problem261);

/// Let us call a positive integer k a square-pivot, if there is a pair of integers m > 0 and n ≥ k, such that the
/// sum of the (m+1) consecutive squares up to k equals the sum of the m consecutive squares from (n+1) on:
///
///              (k-m)² + ... + k² = (n+1)² + ... + (n+m)².
///
/// Some small square-pivots are
///
/// 4: 3² + 4² = 5²
/// 21: 20² + 21² = 29²
/// 24: 21² + 22² + 23² + 24² = 25² + 26² + 27²
/// 110: 108² + 109² + 110² = 133² + 134²
///
/// Find the sum of all distinct square-pivots ≤ 10^10.
pub fn problem261() -> String {
    let limit = 10_000_000_000;
    let limit_sqrt = limit.sqrt();

    let mut factors = vec![1; limit_sqrt + 1];
    let mut is_prime = bit_vec::BitVec::from_elem(limit_sqrt + 1, true);
    is_prime.set(0, false);
    is_prime.set(1, false);

    for p in 2..=limit_sqrt {
        if is_prime[p] {
            for j in (p..=limit_sqrt).step_by(p) {
                is_prime.set(j, false);
                let mut k = j;
                let mut d = 0;
                while k % p == 0 {
                    k /= p;
                    d += 1;
                }
                factors[j] *= p.pow(((d + 1) / 2) as u32);
            }
        }
    }

    let mut pivots = HashSet::new();
    for m in 1.. {
        if m * (m + 1) > limit {
            break;
        }

        let m_plus_1 = m + 1;
        let big_m = 2 * m + 1;
        let max_y = 2 * m * m_plus_1.sqrt();
        let fm = factors[m];

        for y in (fm..=max_y).step_by(fm) {
            let val = m_plus_1 * (y * y / m + 1);
            if let Some(x) = val.get_sqrt() {
                if x % 2 == m_plus_1 % 2 && y % 2 == m % 2 {
                    let mut curr_x = x;
                    let mut curr_y = y;
                    loop {
                        if curr_x >= curr_y + big_m && (curr_y + m) / 2 <= limit {
                            pivots.insert((curr_y + m) / 2);
                        }

                        let next_x = big_m * curr_x + (big_m + 1) * curr_y;
                        let next_y = (big_m - 1) * curr_x + big_m * curr_y;
                        curr_x = next_x;
                        curr_y = next_y;

                        if (curr_y + m) / 2 > limit {
                            break;
                        }
                    }
                }
            }
        }
    }

    let result: usize = pivots.iter().sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem261::problem261;

    #[test]
    fn test_problem261() {
        let result = problem261();
        assert_eq!(result, "238890850232021");
    }
}
