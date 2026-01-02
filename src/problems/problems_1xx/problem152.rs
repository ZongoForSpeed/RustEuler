use crate::register_problem;
use fraction::Fraction;

register_problem!(152, "Writing 1/2 as a sum of inverse squares", problem152);

fn generate(primes: &Vec<u64>, p: u64, f: Fraction, a: u64) -> u64 {
    let objective = Fraction::new(1u64, 2u64);

    let mut result = 0;
    for m in (a + 1..).take_while(|m| m * p < 80 + 1) {
        let mut q = m;
        for pp in primes {
            if *pp > p {
                break;
            }
            while q % (*pp) == 0 {
                q /= pp;
            }
        }

        if q == 1 {
            let f1 = f + Fraction::new(1u64, m * m * p * p);
            if f1 == objective {
                result += 1;
            } else {
                if f1.denom().unwrap() % p != 0 {
                    for pp in primes {
                        if *pp < p {
                            result += generate(primes, *pp, f1, 0);
                        }
                    }
                }
                result += generate(primes, p, f1, m);
            }
        }
    }
    result
}

pub fn problem152() -> String {
    // There are several ways to write the number 1/2 as a sum of inverse squares using distinct integers.
    //
    // For instance, the numbers {2,3,4,5,7,12,15,20,28,35} can be used:
    //
    //      1/2 = 1/2² + 1/3² + 1/4² + 1/5² + 1/7² + 1/12² + 1/15² + 1/20² + 1/28² + 1/35²
    //
    // In fact, only using integers between 2 and 45 inclusive, there are exactly three ways to do it, the remaining two
    // being: {2,3,4,6,7,9,10,20,28,35,36,45} and {2,3,4,6,7,9,12,15,28,30,35,36,45}.
    //
    // How many ways are there to write the number 1/2 as a sum of inverse squares using distinct integers between 2 and
    // 80 inclusive?
    let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    let mut result = 0;
    for p in &primes {
        result += generate(&primes, *p, Fraction::from(0), 0);
    }

    result.to_string()
}
