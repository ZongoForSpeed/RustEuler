use crate::maths::power::Power;
use crate::maths::primes::crible235;
use crate::register_problem;
use num_traits::PrimInt;

register_problem!(233, "Lattice points on a circle", problem233);

pub fn problem233() -> String {
    // Let f(N) be the number of points with integer coordinates that are on a circle passing through (0,0), (N,0),
    // (0,N), and (N,N).
    //
    // It can be shown that f(10000) = 36
    //
    // What is the sum of all positive integers N ≤ 10^11 such that f(N) = 420 ?
    let limit = usize::power(10, 11);

    let mut primes: Vec<usize> = Vec::new();
    crible235(limit / 5.pow(3) / 13.pow(2), |p| primes.push(p));

    let mut multi = Vec::from_iter(0..(limit / (5.pow(3) * 13.pow(2) * 17) + 1));

    let mut primes4 = Vec::new();
    for p in primes {
        if p % 4 == 1 {
            primes4.push(p);
            for j in (p..multi.len()).step_by(p) {
                multi[j] = 0;
            }
        }
    }

    for c in 1..multi.len() {
        multi[c] += multi[c - 1];
    }

    let mut resultat = 0;

    // n = p * q^2 * r^3
    for &r in &primes4 {
        let r3 = r * r * r;
        if 13 * 5 * 5 * r3 > limit {
            break;
        }

        for &q in &primes4 {
            let qq = q * q;
            if 5 * qq * r3 > limit {
                break;
            }

            if r == q {
                continue;
            }

            for &p in &primes4 {
                let n = p * qq * r3;
                if n > limit {
                    break;
                }

                if r == p || q == p {
                    continue;
                }

                resultat += n * multi[limit / n];
            }
        }
    }

    // n = q^3 * r^7
    for &r in &primes4 {
        let r7 = r.pow(7);
        if 5 * 5 * 5 * r7 > limit {
            break;
        }

        for &q in &primes4 {
            let n = q * q * q * r7;
            if n > limit {
                break;
            }

            if r == q {
                continue;
            }
            resultat += n * multi[limit / n];
        }
    }

    // n = q^2 * r^10
    for &r in &primes4 {
        let r10 = r.pow(10);
        if 5 * 5 * r10 > limit {
            break;
        }

        for &q in &primes4 {
            let n = q * q * r10;
            if n > limit {
                break;
            }

            if r == q {
                continue;
            }
            resultat += n * multi[limit / n];
        }
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem233::problem233;

    #[test]
    fn test_problem233() {
        let result = problem233();
        assert_eq!(result, "271204031455541309");
    }
}
