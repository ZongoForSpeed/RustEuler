use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;

register_problem!(267, "Billionaire", problem267);

/// Calculates the number of heads needed to reach a capital of N
/// given a fixed proportion `f` of capital to bet on a fair coin toss
/// for `n` tosses.
///
/// If capital is C_k after k heads and (n-k) tails:
/// C_k = (1 + 2f)^k * (1 - f)^(n - k) >= N
/// k ln(1 + 2f) + (n - k) ln(1 - f) >= ln(N)
/// k (ln(1 + 2f) - ln(1 - f)) >= ln(N) - n ln(1 - f)
/// k >= (ln(N) - n ln(1 - f)) / (ln(1 + 2f) - ln(1 - f))
fn heads_needed(f: f64, n: u64, target: f64) -> f64 {
    (target.ln() - (n as f64) * (1.0 - f).ln()) / ((1.0 + 2.0 * f).ln() - (1.0 - f).ln())
}

/// Ternary search to find the value of f in [left, right] that minimizes `func`.
fn ternary_search<F>(mut left: f64, mut right: f64, epsilon: f64, func: F) -> f64
where
    F: Fn(f64) -> f64,
{
    while right - left > epsilon {
        let m1 = left + (right - left) / 3.0;
        let m2 = right - (right - left) / 3.0;
        if func(m1) < func(m2) {
            right = m2;
        } else {
            left = m1;
        }
    }
    (left + right) / 2.0
}

/// You are given a unique investment opportunity.
///
/// Starting with £1 of capital, you can choose a fixed proportion, f, of your capital to bet on a fair coin toss
/// repeatedly for 1000 tosses.
///
/// Your return is double your bet for heads and you lose your bet for tails.
///
/// For example, if f = 1/4, for the first toss you bet £0.25, and if heads comes up you win £0.5 and so then have
/// £1.5. You then bet £0.375 and if the second toss is tails, you have £1.125.
///
/// Choosing f to maximize your chances of having at least £1,000,000,000 after 1,000 flips, what is the chance that
/// you become a billionaire?
///
/// All computations are assumed to be exact (no rounding), but give your answer rounded to 12 digits behind the
/// decimal point in the form 0.abcdefghijkl.
pub fn problem267() -> String {
    let n = 1000usize;
    let target = 1_000_000_000.0;

    let min_heads_needed = |f: f64| heads_needed(f, n as u64, target);
    let best_f = ternary_search(0.0, 1.0, 1e-12, min_heads_needed);
    let k_min = heads_needed(best_f, n as u64, target).ceil() as usize;

    // Calculate sum of binomial coefficients C(n, k) for k from k_min to n
    // Use the property C(n, k) = C(n, k-1) * (n - k + 1) / k
    let mut combinations = vec![MpzNumber::from(1)];
    for k in 1..=n {
        let current = &combinations[k - 1] * (n - k + 1) / k;
        combinations.push(current);
    }

    let mut favorable_outcomes = MpzNumber::from(0);
    for k in k_min..=n {
        favorable_outcomes += &combinations[k];
    }

    let precision_mask = 10u64.pow(13);
    let result_mpz = (&favorable_outcomes * precision_mask) / MpzNumber::power_ui(2, n as u64);
    let result = result_mpz.get_f64() / (precision_mask as f64);

    format!("{:.12}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem267() {
        assert_eq!(problem267(), "0.999992836187");
    }
}
