use crate::maths::power::Power;
use crate::register_problem;
use num_integer::Roots;
use std::ops::RemAssign;

register_problem!(
    171,
    "Finding numbers for which the sum of the squares of the digits is a square",
    problem171
);

pub fn problem171() -> String {
    // For a positive integer n, let f(n) be the sum of the squares of the digits (in base 10) of n, e.g.
    //
    //      f(3) = 3² = 9,
    //      f(25) = 2² + 5² = 4 + 25 = 29,
    //      f(442) = 4² + 4² + 2² = 16 + 16 + 4 = 36
    //
    // Find the last nine digits of the sum of all n, 0 < n < 10^20, such that f(n) is a perfect square.
    let mask = 1000000000;
    let limit = 20;

    let mut combination: Vec<u64> = vec![1];
    let mut sums: Vec<u64> = vec![0];

    for n in 0..limit {
        let mut n_sums = vec![0; sums.len() + 81];

        if n > 8 {
            for i in 0..sums.len() {
                for j in 0..10 {
                    n_sums[i + j * j] += sums[i];
                }
            }
        } else {
            let mut n_combination = vec![0; combination.len() + 81];
            for i in 0..sums.len() {
                if combination[i] != 0 {
                    for j in 0..10 {
                        n_combination[i + j * j] += combination[i];
                        n_sums[i + j * j] +=
                            sums[i] + combination[i] * j as u64 * u64::power(10, n);
                    }
                }
            }
            combination = n_combination;
        }

        for s in n_sums.iter_mut() {
            s.rem_assign(mask);
        }

        sums = n_sums;
    }

    let mut result = 0;
    for i in 1..sums.len().sqrt() {
        result += sums[i * i] % mask;
    }

    result %= mask;
    result.to_string()
}
