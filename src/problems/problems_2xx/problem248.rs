use crate::maths::arithmetic::Arithmetic;
use crate::maths::factorial::Factorial;
use crate::maths::primes::{Primes, crible235};
use crate::register_problem;
use num_integer::Roots;
use std::collections::BTreeSet;

register_problem!(
    248,
    "Numbers for which Euler’s totient function equals 13!",
    problem248
);

fn is_prime(n: u64, primes: &BTreeSet<u64>) -> bool {
    if let Some(&last) = primes.last()
        && n < last
    {
        return primes.contains(&n);
    }

    n.miller_rabin(10)
}

fn enumerate(
    x: u64,
    phi: u64,
    factors: &Vec<u64>,
    start: usize,
    end: usize,
    mut result: &mut Vec<u64>,
) {
    if phi == 1 {
        result.push(x);
    }

    for i in start..end {
        let p = factors[i];
        if phi % (p - 1) == 0 {
            enumerate(x * p, phi / (p - 1), &factors, start, i, &mut result);

            let mut _x = x * p;
            let mut _phi = phi / (p - 1);

            while _phi % p == 0 {
                _phi /= p;
                _x *= p;

                enumerate(_x, _phi, &factors, start, i, &mut result);
            }
        }
    }
}

pub fn problem248() -> String {
    // The first number n for which φ(n)=13! is 6227180929.
    //
    // Find the 150,000th such number.
    let phi = u64::factorial(13);

    let mut primes: BTreeSet<u64> = BTreeSet::new();
    crible235(phi.sqrt() as usize, |p| {
        primes.insert(p);
    });

    let mut factors = Vec::new();
    let divisors = phi.divisors(&primes);
    for d in divisors {
        if is_prime(d + 1, &primes) {
            factors.push(d + 1);
        }
    }

    let mut result = Vec::new();
    enumerate(1, phi, &factors, 0, factors.len(), &mut result);
    result.sort();
    result[150000 - 1].to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem248::problem248;

    #[test]
    fn test_problem248() {
        let result = problem248();
        assert_eq!(result, "23507044290");
    }
}
