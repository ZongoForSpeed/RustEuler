use crate::maths::primes::Primes;
use crate::register_problem;

register_problem!(304, "Primonacci", problem304);

const MODULO: u64 = 1_234_567_891_011;

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

/// Calcule f(n) mod MODULO par doublement rapide.
fn fibonacci_modulo(n: u64) -> u64 {
    let (mut a, mut b) = (0, 1);
    for bit in (0..64).rev() {
        // F(2k) = F(k)·(2F(k+1) − F(k)) et F(2k+1) = F(k)² + F(k+1)²
        let c = mulmod(a, (2 * b + MODULO - a) % MODULO, MODULO);
        let d = (mulmod(a, a, MODULO) + mulmod(b, b, MODULO)) % MODULO;
        if (n >> bit) & 1 == 1 {
            a = d;
            b = (c + d) % MODULO;
        } else {
            a = c;
            b = d;
        }
    }

    a
}

/// For any positive integer n the function next_prime(n) returns the smallest prime p such that p>n.
///
/// The sequence a(n) is defined by: a(1)=next_prime(10**14) and a(n)=next_prime(a(n-1)) for n>1.
///
/// The fibonacci sequence f(n) is defined by: f(0)=0, f(1)=1 and f(n)=f(n-1)+f(n-2) for n>1.
///
/// The sequence b(n) is defined as f(a(n)).
///
/// Find ∑b(n) for 1≤n≤100 000. Give your answer mod 1234567891011.
pub fn problem304() -> String {
    let mut a = 10u64.pow(14);
    let mut resultat = 0;
    for _ in 1..=100_000 {
        a = a.next_prime(25);
        resultat = (resultat + fibonacci_modulo(a)) % MODULO;
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem304() {
        assert_eq!(problem304(), "283988410192");
    }
}
