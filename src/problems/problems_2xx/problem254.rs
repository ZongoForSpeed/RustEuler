use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;
use std::collections::HashMap;
use std::ops::{AddAssign, MulAssign};
use num_traits::One;
use crate::maths::digits::Digits;

register_problem!(254, "Sums of Digit Factorials", problem254);

struct Result {
    sg: u32,
    g: MpzNumber
}

fn algorithm(cache: &mut HashMap<u32, Result>, factorials: &Vec<u32>, mut f: u32) {
    let sf = f.sum_digits(10);

    let mut sg = 0;
    let mut digits = vec![0; 10];
    for i in (1..10).rev() {
        digits[i] = f / factorials[i];
        f = f % factorials[i];
        sg += i as u32 * digits[i];
    }

    let mut n = MpzNumber::new();
    for i in 0..10 {
        for _ in 0..digits[i] {
            n.mul_assign(10);
            n.add_assign(i);
        }
    }

    let result = Result { sg, g: n };
    if let Some(found) = cache.get(&sf) {
        if result.g < found.g {
            cache.insert(sf, result);
        }
    } else {
        cache.insert(sf, result);
    }
}

/// Define f(n) as the sum of the factorials of the digits of n. For example, f(342) = 3! + 4! + 2! = 32.
///
/// Define sf(n) as the sum of the digits of f(n). So sf(342) = 3 + 2 = 5.
///
/// Define g(i) to be the smallest positive integer n such that sf(n) = i.
/// Though sf(342) is 5, sf(25) is also 5, and it can be verified that g(5) is 25.
///
/// Define sg(i) as the sum of the digits of g(i). So sg(5) = 2 + 5 = 7.
///
/// Further, it can be verified that g(20) is 267 and ∑ sg(i) for 1 ≤ i ≤ 20 is 156.
///
/// What is ∑ sg(i) for 1 ≤ i ≤ 150?
pub fn problem254() -> String {
    let factorial = vec![1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

    let mut cache = HashMap::new();

    let borne = 999999;
    let limite = 9999999;
    for f in 1..borne {
        algorithm(&mut cache, &factorial, f);
    }

    for f in (borne..limite).step_by(100) {
        algorithm(&mut cache, &factorial, f);
        algorithm(&mut cache, &factorial, f + 90);
    }

    let mut sum = MpzNumber::new();
    for r in cache.values() {
        sum += r.sg;
    }

    let one = MpzNumber::one();

    let mut prefix = MpzNumber::from_u64(9);
    let mut nines = MpzNumber::from_u64(999999);

    for _ in 63..=150 {
        let mut lf = MpzNumber::concat_numbers(&prefix, &nines, 10);
        for i in (1..10).rev() {
            sum += i * (&lf / factorial[i]);
            lf %= factorial[i];
        }
        prefix = (prefix % 9) + 1;
        if prefix == one {
            nines = nines * 10 + 9;
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem254::problem254;

    #[test]
    fn test_problem254() {
        let result = problem254();
        assert_eq!(result, "8184523820510");
    }
}
