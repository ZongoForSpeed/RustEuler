use num_traits::{One, Zero};
use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;

register_problem!(266, "Pseudo Square Root", problem266);

fn get_subset_products(primes: &[u64]) -> Vec<MpzNumber> {
    let mut products = Vec::with_capacity(1 << primes.len());
    products.push(MpzNumber::one());
    for &p in primes {
        let p_mpz = MpzNumber::from(p);
        let len = products.len();
        for i in 0..len {
            products.push(&products[i] * &p_mpz);
        }
    }
    products.sort();
    products
}

/// The divisors of 12 are: 1,2,3,4,6 and 12.
///
/// The largest divisor of 12 that does not exceed the square root of 12 is 3. We shall call the largest divisor of
/// an integer n that does not exceed the square root of n the pseudo square root (PSR) of n.
/// It can be seen that PSR(3102)=47.
///
/// Let p be the product of the primes below 190.
/// Find PSR(p) mod 10^16.
pub fn problem266() -> String {
    let modulo = 10_000_000_000_000_000u128;
    let primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
    ];

    let mid = primes.len() / 2;
    let d1 = get_subset_products(&primes[..mid]);
    let d2 = get_subset_products(&primes[mid..]);

    let p: MpzNumber = primes.iter().map(|&x| MpzNumber::from(x)).product();
    let limit = p.sqrt();

    let mut max_psr = MpzNumber::zero();

    let mut iterator = d2.iter().rev().peekable();
    for x in d1.iter() {
        while let Some(&y) = iterator.peek() {
            if x * y > limit {
                iterator.next();
            } else {
                let current = x * y;
                if current > max_psr {
                    max_psr = current;
                }
                break;
            }
        }
        if iterator.peek().is_none() {
            break;
        }
    }

    (max_psr % modulo).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem266() {
        assert_eq!(problem266(), "1096883702440585");
    }
}
