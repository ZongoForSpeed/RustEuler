use crate::maths::arithmetic::Arithmetic;
use crate::maths::primes;
use crate::register_problem;

register_problem!(21, "Amicable numbers", problem021);

pub fn problem021() -> String {
    // Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
    // If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.
    //
    // For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
    //
    // Evaluate the sum of all the amicable numbers under 10000.
    let mut primes: Vec<u64> = Vec::new();
    primes::crible2(10000, |p| primes.push(p));
    let mut diviseurs: Vec<u64> = Vec::new();
    diviseurs.reserve(10000);
    diviseurs.push(0);
    let mut resultat:u64 = 0;
    for n in 1..10000 {
        let d = n.sum_of_divisors(&primes) - n;
        diviseurs.push(d);
        if d < n && diviseurs[d as usize] == n {
            resultat += d + n;
        }
    }
    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem021() {
        let result = problem021();
        assert_eq!(result, "31626");
    }
}
