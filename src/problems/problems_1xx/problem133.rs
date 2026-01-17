use crate::maths::arithmetic::Arithmetic;
use crate::maths::primes::crible2;
use crate::register_problem;

register_problem!(133, "Repunit nonfactors", problem133);

pub fn problem133() -> String {
    // A number consisting entirely of ones is called a repunit. We shall define R(k) to be a repunit of length k;
    // for example, R(6) = 111111.
    //
    // Let us consider repunits of the form R(10n).
    //
    // Although R(10), R(100), or R(1000) are not divisible by 17, R(10000) is divisible by 17. Yet there
    // is no value of n for which R(10n) will divide by 19. In fact, it is remarkable that 11, 17, 41,
    // and 73 are the only four primes below one-hundred that can be a factor of R(10n).
    //
    // Find the sum of all the primes below one-hundred thousand that will never be a factor of R(10n).
    let limit = 100000;
    let mut primes:Vec<u32> = Vec::new();
    crible2(limit, |p| primes.push(p));

    let mut result = 0;
    for p in primes {
        if p == 2 || p == 5 {
            result += p;
            continue;
        }

        let mut k = p.repunit_a(10);
        while k & 1 == 0 {
            k >>= 1;
        }
        while k % 5 == 0 {
            k /= 5;
        }
        if k != 1 {
            result += p;
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem133() {
        let result = problem133();
        assert_eq!(result, "453647705");
    }
}
