use crate::maths::digits::Digits;
use crate::maths::primes::crible23;
use crate::register_problem;
use fraction::Fraction;

register_problem!(70, "Totient permutation", problem070);

pub fn problem070() -> String {
    // Euler's Totient function, φ(n) [sometimes called the phi function], is used to determine the
    // number of positive numbers less than or equal to n which are relatively prime to n. For
    // example, as 1, 2, 4, 5, 7, and 8, are all less than nine and relatively prime to nine, φ(9)=6.
    //
    // The number 1 is considered to be relatively prime to every positive number, so φ(1)=1.
    //
    // Interestingly, φ(87109)=79180, and it can be seen that 87109 is a permutation of 79180.
    //
    // Find the value of n, 1 < n < 10^7, for which φ(n) is a permutation of n and the ratio n/φ(n)
    // produces a minimum.
    let mut primes:Vec<u32> = Vec::new();
    crible23(5000, |p| primes.push(p));

    let mut min_ratio: Fraction = Fraction::new(u64::MAX, 1u64);
    let mut min_n = 0;

    let size = primes.len();
    for pi in 0..size {
        let p = primes[pi];
        for qi in 0..=pi {
            let q = primes[qi];
            let n = p * q;
            if n > 10000000 {
                break;
            }

            let phi = (p - 1) * (q - 1);
            let ratio = Fraction::new(n, phi);
            if ratio < min_ratio && u32::is_permutation(n, phi, 10) {
                min_ratio = ratio;
                min_n = n;
            }
        }
    }

    println!("min_ratio: {}, min_n: {}", min_ratio, min_n);

    min_n.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem070() {
        let result = problem070();
        assert_eq!(result, "8319823");
    }
}
