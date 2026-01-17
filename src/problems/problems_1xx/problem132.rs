use crate::maths::power::Power;
use crate::maths::primes::crible23;
use crate::register_problem;

register_problem!(132, "Large repunit factors", problem132);

pub fn problem132() -> String {
    // A number consisting entirely of ones is called a repunit. We shall define R(k) to be a repunit of length k.
    //
    // For example, R(10) = 1111111111 = 11×41×271×9091, and the sum of these prime factors is 9414.
    //
    // Find the sum of the first forty prime factors of R(109).
    let limit = 1000000;
    let mut primes:Vec<u64> = Vec::new();
    crible23(limit, |p| primes.push(p));

    let mut count = 0;
    let mut result = 0;

    for p in primes {
        if u64::power_mod(10, 1000000000, 9*p) == 1 {
            count += 1;
            result += p;
            if count == 40 {
                break;
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem132() {
        let result = problem132();
        assert_eq!(result, "843296");
    }
}
