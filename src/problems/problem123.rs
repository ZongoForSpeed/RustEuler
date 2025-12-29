use crate::maths::power::Power;
use crate::maths::primes::crible23;
use crate::register_problem;

register_problem!(123, "Prime square remainders", problem123);

pub fn problem123() -> String {
    // Let pn be the nth prime: 2, 3, 5, 7, 11, ..., and let r be the remainder when (pn−1)^n + (pn+1)^n is divided
    // by pn^2.
    //
    // For example, when n = 3, p3 = 5, and 43 + 63 = 280 ≡ 5 mod 25.
    //
    // The least value of n for which the remainder first exceeds 10^9 is 7037.
    //
    // Find the least value of n for which the remainder first exceeds 10^10.
    let mut primes: Vec<u128> = Vec::new();
    crible23(1000000, |p| primes.push(p));

    let borne = 10u128.pow(10);
    for (i, p) in primes.iter().enumerate() {
        let n = (i + 1) as u128;
        let pp = p * p;
        if pp < borne {
            continue;
        }
        let p1 = u128::power_mod(p + 1, n, pp);
        let p2 = u128::power_mod(p - 1, n, pp);
        if (p1 + p2) % pp > borne {
            return n.to_string();
        }
    }

    panic!("No solution found");
}
