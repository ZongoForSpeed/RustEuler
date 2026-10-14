use crate::maths::primes::crible2;
use crate::register_problem;

register_problem!(
    272,
    "Modular Cubes, part 2",
    problem272
);

fn algorithme(premiers: &[(u64, bool)], limite: u64, m: i64, n0: u64) -> u64 {
    if m < 0 {
        return 0;
    }

    let mut total = 0;
    let mut it = 0;
    loop {
        let mut n = n0;
        if it == premiers.len() || n * premiers[it].0 > limite {
            if m != 0 {
                break;
            }

            total += n;
            break;
        }

        let (p, cubique) = premiers[it];
        if n.saturating_mul(p.saturating_pow(m as u32)) > limite {
            break;
        }
        let mut m2 = m;
        if cubique {
            m2 -= 1;
        }
        while p * n < limite {
            n *= p;
            total += algorithme(&premiers[it + 1..], limite, m2, n);
            if p == 3 {
                m2 -= 1;
                while p * n < limite {
                    n *= p;
                    total += algorithme(&premiers[it + 1..], limite, m2, n);
                }
                break;
            }
        }

        it += 1;
    }

    total
}

/// For a positive number n, define C(n) as the number of the integers x, for
/// which 1<x<n and x3≡1 mod n.
///
/// When n=91, there are 8 possible values for x, namely : 9, 16, 22, 29, 53,
/// 74, 79, 81.
/// Thus, C(91)=8.
///
/// Find the sum of the positive numbers n≤10**11 for which C(n)=242.
pub fn problem272() -> String {
    let limite = 10u64.pow(11);
    let mut premiers = Vec::new();
    crible2::<u64, _>(100_000_000, |p| premiers.push(p));

    let premiers_cubiques: Vec<(u64, bool)> = premiers.into_iter().map(|p| (p, p % 3 == 1)).collect();

    algorithme(&premiers_cubiques, limite, 5, 1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem272() {
        assert_eq!(problem272(), "8495585919506151122");
    }
}
