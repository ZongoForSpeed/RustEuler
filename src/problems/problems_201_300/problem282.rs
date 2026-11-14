use crate::maths::arithmetic::Arithmetic;
use crate::maths::power::Power;
use crate::maths::primes::crible2;
use crate::register_problem;

register_problem!(282, "The Ackermann function", problem282);

fn ackermann(m: u64, n: u64) -> u64 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ackermann(m - 1, 1)
    } else {
        ackermann(m - 1, ackermann(m, n - 1))
    }
}

fn tetration(a: u64, m: u64, primes: &[u64]) -> u64 {
    // calcule 2↑↑a mod m
    if m < 2 {
        return 0;
    }

    if a == 1 {
        return 2 % m;
    }

    let mut facteurs = Vec::new();
    m.factorization(primes, |p, e| facteurs.push((p, e as u32)));
    if facteurs.len() == 1 {
        let (p, e) = facteurs[0];
        if p == 2 {
            const F: [u64; 5] = [0, 2, 4, 16, 65536];
            const LOG_F: [u32; 6] = [0, 1, 2, 4, 16, 65536];
            let a = a as usize;
            if a < F.len() && e > LOG_F[a] { F[a] } else { 0 }
        } else {
            // si m premier avec 2
            // alors 2↑↑a mod m = 2**(2↑↑(a-1) mod phi(m)) mod m
            let x = tetration(a - 1, m.phi(primes), primes);
            2u64.power_mod(x, m)
        }
    } else {
        // théorème des restes chinois sur les puissances de premiers de m
        let mut modulo_total: u64 = 1;
        let mut reste: u64 = 0;
        for &(p, e) in &facteurs {
            let mi = p.pow(e);
            let ri = tetration(a, mi, primes);
            // inverse de modulo_total modulo mi (modulo_total et mi sont premiers entre eux)
            let inverse = i64::bezout(modulo_total as i64 % mi as i64, mi as i64)
                .0
                .rem_euclid(mi as i64) as u64;
            let t = (ri + mi - reste % mi) % mi * inverse % mi;
            reste += modulo_total * t;
            modulo_total *= mi;
        }

        reste % modulo_total
    }
}

/// For non-negative integers m, n, the Ackermann function A(m, n) is defined
/// as follows:
///               | n + 1                     if m = 0
///      A(m,n) = | A(m - 1, 1)               if m > 0 and n = 0
///               | A(m - 1, A(m, n - 1))     if m > 0 and n > 0
///
/// For example A(1, 0) = 2, A(2, 2) = 7 and A(3, 4) = 125.
///
/// Find ∑ A(n, n), n=0..6 and give your answer mod 14**8.
pub fn problem282() -> String {
    let modulo = 14u64.pow(8);

    // les diviseurs de modulo et les modules intermédiaires sont inférieurs à 14**8,
    // donc il suffit de cribler jusqu'à sqrt(14**8) ≈ 38 800
    let mut primes = Vec::new();
    crible2::<u64, _>(40_000, |p| primes.push(p));

    let mut resultat = 0;
    resultat += ackermann(0, 0);
    resultat += ackermann(1, 1);
    resultat += ackermann(2, 2);
    resultat += ackermann(3, 3);
    // A(4, 4) = 2↑↑7 - 3
    resultat += (tetration(7, modulo, &primes) + modulo - 3) % modulo;
    // A(5, 5) = 2↑↑↑8 - 3
    //         = 2↑↑N - 3
    resultat += 2 * ((tetration(100, modulo, &primes) + modulo - 3) % modulo);
    resultat %= modulo;

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem282() {
        assert_eq!(problem282(), "1098988351");
    }
}
