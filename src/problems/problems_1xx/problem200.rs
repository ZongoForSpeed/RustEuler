use crate::maths::digits::{Digits, conversion};
use crate::maths::primes::{Primes, crible235};
use crate::register_problem;

register_problem!(
    200,
    "Find the 200th prime-proof sqube containing the contiguous sub-string \"200\"",
    problem200
);

fn test200(mut n: u128) -> bool {
    while n != 0 {
        if n % 1000 == 200 {
            return true;
        }
        n /= 10;
    }

    false
}

fn test(n: u128) -> bool {
    if !test200(n) {
        return false;
    }

    let mut digits = n.extract_digits(10);
    for n in 0..digits.len() {
        let d = digits[n];
        for a in 0..10 {
            if a != d {
                digits[n] = a;
                let conv = conversion(&digits, 10);
                if conv.miller_rabin(25) {
                    return false;
                }
            }
        }
        digits[n] = d;
    }

    true
}

pub fn problem200() -> String {
    // We shall define a sqube to be a number of the form, p2q3, where p and q are distinct primes.
    // For example, 200 = 5²2^3 or 120072949 = 23²61^3.
    //
    // The first five squbes are 72, 108, 200, 392, and 500.
    //
    // Interestingly, 200 is also the first number for which you cannot change any single digit to make a prime; we
    // shall call such numbers, prime-proof. The next prime-proof sqube which contains the contiguous sub-string "200"
    // is 1992008.
    //
    // Find the 200th prime-proof sqube containing the contiguous sub-string "200".
    let mut primes: Vec<u128> = Vec::new();
    crible235(200000, |p| primes.push(p));

    let mut squbes = Vec::new();

    for &p1 in &primes {
        for &p2 in &primes {
            if p1 != p2 {
                let sqube = p1 * p1 * p1 * p2 * p2;
                if (squbes.len() < 200 || sqube < *squbes.last().unwrap()) && test(sqube) {
                    if squbes.len() == 200 {
                        squbes.pop();
                    }
                    squbes.push(sqube);
                    squbes.sort();
                }
            }
        }
    }

    squbes.last().unwrap().to_string()
}
