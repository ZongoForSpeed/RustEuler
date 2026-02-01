use crate::maths::arithmetic::Arithmetic;
use crate::register_problem;
use fraction::Fraction;

register_problem!(243, "Resilience", problem243);

pub fn problem243() -> String {
    // A positive fraction whose numerator is less than its denominator is called a proper fraction.
    // For any denominator, d, there will be d−1 proper fractions; for example, with d = 12:
    // 1/12 , 2/12 , 3/12 , 4/12 , 5/12 , 6/12 , 7/12 , 8/12 , 9/12 , 10/12 , 11/12 .
    //
    // We shall call a fraction that cannot be cancelled down a resilient fraction.
    //
    // Furthermore we shall define the resilience of a denominator, R(d), to be the ratio of its
    // proper fractions that are resilient; for example, R(12) = 4/11 .
    //
    // In fact, d = 12 is the smallest denominator having a resilience R(d) < 4/10 .
    //
    // Find the smallest denominator d, having a resilience R(d) < 15499/94744 .
    let objectif = Fraction::new(15499u64, 94744u64);

    let primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199,
    ];

    let mut denominator = 1;
    for &p in &primes {
        denominator *= p;
        let ratio = Fraction::new(u64::phi(denominator, &primes), denominator - 1);
        if ratio < objectif {
            denominator /= p;
            break;
        }
    }

    for i in 1.. {
        let n = i * denominator;
        let ratio = Fraction::new(u64::phi(n, &primes), n - 1);
        if ratio < objectif {
            return n.to_string();
        }
    }

    panic!("No solution found!");
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem243::problem243;

    #[test]
    fn test_problem243() {
        let result = problem243();
        assert_eq!(result, "892371480");
    }
}
