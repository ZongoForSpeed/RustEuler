use crate::maths::digits::{concat_numbers, conversion, Digits};
use crate::register_problem;
use crate::utils::permutations::permutations;

register_problem!(
    170,
    "Find the largest 0 to 9 pandigital that can be formed by concatenating products",
    problem170
);

fn pandigital(n: u64) -> bool {
    let mut digits = vec![0; 10];
    n.loop_digits(10, |d| digits[d as usize] += 1);
    for d in digits {
        if d > 1 {
            return false;
        }
    }
    true
}

fn pandigital_complete(n: u64) -> bool {
    let mut digits = vec![0; 10];
    n.loop_digits(10, |d| digits[d as usize] += 1);
    for d in digits {
        if d != 1 {
            return false;
        }
    }
    true
}

pub fn problem170() -> String {
    // Take the number 6 and multiply it by each of 1273 and 9854:
    //
    //          6 × 1273 = 7638
    //          6 × 9854 = 59124
    //
    // By concatenating these products we get the 1 to 9 pandigital 763859124. We will call 763859124 the "concatenated
    // product of 6 and (1273,9854)". Notice too, that the concatenation of the input numbers, 612739854, is also 1 to 9
    // pandigital.
    //
    // The same can be done for 0 to 9 pandigital numbers.
    //
    // What is the largest 0 to 9 pandigital 10-digit concatenated product of an integer with two or more other integers,
    // such that the concatenation of the input numbers is also a 0 to 9 pandigital 10-digit number?
    let digits: Vec<u64> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let size = digits.len();

    let mut resultat = 0;
    for permutation in permutations(digits) {
        if permutation[0] == 0 {
            continue;
        }

        for n0 in 1..size - 1 {
            if permutation[n0] == 0 {
                continue;
            }

            let facteur = conversion(&permutation[..n0], 10);
            for n1 in n0 + 1..size - 1 {
                if permutation[n1] == 0 {
                    continue;
                }
                let a = conversion(&permutation[n0..n1], 10);
                if a == 0 || !pandigital(facteur * a) {
                    continue;
                }
                let b = conversion(&permutation[n1..], 10);
                if b == 0 || !pandigital(facteur * b) {
                    continue;
                }
                let n = concat_numbers(facteur * a, facteur * b, 10);
                if pandigital_complete(n) && resultat < n {
                    resultat = n;
                }
            }
        }
    }

    resultat.to_string()
}
