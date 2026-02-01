use crate::maths::primes::crible235;
use crate::register_problem;
use fraction::GenericFraction;
use num_traits::ConstOne;

register_problem!(241, "Perfection Quotients", problem241);

type UFraction = GenericFraction<u128>;

fn first_factor(mut n: u128, primes: &Vec<u128>) -> (u128, u32) {
    for &p in primes {
        if p * p > n {
            break;
        }
        if n % p == 0 {
            let mut count = 0;
            while n % p == 0 {
                count += 1;
                n /= p;
            }
            return (p, count);
        }
    }

    (n, 1)
}

fn solution(
    primes: &Vec<u128>,
    limit: u128,
    n: u128,
    sigma: UFraction,
    mut results: &mut Vec<u128>,
) {
    let (p, exponent) = first_factor(*sigma.numer().unwrap(), &primes);
    if (n % p) == 0 {
        return;
    }

    for a in exponent.. {
        let m = n * p.pow(a);
        if m > limit {
            break;
        }

        let sigma2 = UFraction::new(p.pow(a + 1) - 1, p.pow(a) * (p - 1)) * sigma;

        if sigma2 == UFraction::ONE {
            results.push(m);
        } else if let Some(&num) = sigma2.numer()
            && let Some(&den) = sigma2.denom()
            && num > 1
            && den > 1
        {
            solution(&primes, limit, m, sigma2, &mut results);
        }
    }
}

pub fn problem241() -> String {
    // For a positive integer n, let σ(n) be the sum of all divisors of n, so e.g.
    // σ(6) = 1 + 2 + 3 + 6 = 12.
    //
    // A perfect number, as you probably know, is a number with σ(n) = 2n.
    //
    // Let us define the perfection quotient of a positive integer as p(n) = σ(n)/n
    //
    // Find the sum of all positive integers n ≤ 1018 for which p(n) has the form k + 1/2, where
    // k is an integer.
    let limit = u128::pow(10, 18);

    let mut primes: Vec<u128> = Vec::new();
    crible235(1000, |p| primes.push(p));

    let mut resultats: Vec<u128> = Vec::new();

    for k in 1..6 {
        solution(
            &primes,
            limit,
            1,
            UFraction::new(2u128, 2 * k + 1u128),
            &mut resultats,
        );
    }

    resultats.sort();
    println!("{:?}", resultats);

    let result: u128 = resultats.iter().sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem241::problem241;

    #[test]
    fn test_problem241() {
        let result = problem241();
        assert_eq!(result, "482316491800641154");
    }
}
