use crate::register_problem;

register_problem!(
    282,
    "The Ackermann function",
    problem282
);

fn ackermann(m: u64, n: u64) -> u64 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ackermann(m - 1, 1)
    } else {
        ackermann(m - 1, ackermann(m, n - 1))
    }
}

fn factorisation(mut m: u64) -> Vec<(u64, u32)> {
    let mut facteurs = Vec::new();
    let mut p = 2;
    while p * p <= m {
        if m % p == 0 {
            let mut e = 0;
            while m % p == 0 {
                m /= p;
                e += 1;
            }
            facteurs.push((p, e));
        }
        p += 1;
    }
    if m > 1 {
        facteurs.push((m, 1));
    }

    facteurs
}

fn phi(facteurs: &[(u64, u32)]) -> u64 {
    facteurs.iter().map(|&(p, e)| p.pow(e - 1) * (p - 1)).product()
}

fn puissance_modulaire(base: u64, exposant: u64, modulo: u64) -> u64 {
    let mut resultat: u128 = 1;
    let mut b = base as u128 % modulo as u128;
    let mut e = exposant;
    while e > 0 {
        if e & 1 == 1 {
            resultat = resultat * b % modulo as u128;
        }
        b = b * b % modulo as u128;
        e >>= 1;
    }

    resultat as u64
}

fn inverse_modulaire(a: u64, m: u64) -> u64 {
    // algorithme d'Euclide étendu, a et m premiers entre eux
    let (mut r0, mut r1) = (m as i128, a as i128 % m as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }

    t0.rem_euclid(m as i128) as u64
}

fn tetration(a: u64, m: u64) -> u64 {
    // calcule 2↑↑a mod m
    if m < 2 {
        return 0;
    }

    if a == 1 {
        return 2 % m;
    }

    let facteurs = factorisation(m);
    if facteurs.len() == 1 {
        let (p, e) = facteurs[0];
        if p == 2 {
            const F: [u64; 5] = [0, 2, 4, 16, 65536];
            const LOG_F: [u32; 6] = [0, 1, 2, 4, 16, 65536];
            let a = a as usize;
            if a < F.len() && e > LOG_F[a] {
                F[a]
            } else {
                0
            }
        } else {
            // si m premier avec 2
            // alors 2↑↑a mod m = 2**(2↑↑(a-1) mod phi(m)) mod m
            let x = tetration(a - 1, phi(&facteurs));
            puissance_modulaire(2, x, m)
        }
    } else {
        // théorème des restes chinois sur les puissances de premiers de m
        let mut modulo_total: u64 = 1;
        let mut reste: u64 = 0;
        for &(p, e) in &facteurs {
            let mi = p.pow(e);
            let ri = tetration(a, mi);
            let t = (ri + mi - reste % mi) % mi * inverse_modulaire(modulo_total % mi, mi) % mi;
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

    let mut resultat = 0;
    resultat += ackermann(0, 0);
    resultat += ackermann(1, 1);
    resultat += ackermann(2, 2);
    resultat += ackermann(3, 3);
    // A(4, 4) = 2↑↑7 - 3
    resultat += (tetration(7, modulo) + modulo - 3) % modulo;
    // A(5, 5) = 2↑↑↑8 - 3
    //         = 2↑↑N - 3
    resultat += 2 * ((tetration(100, modulo) + modulo - 3) % modulo);
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
