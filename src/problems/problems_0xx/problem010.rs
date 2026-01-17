use crate::maths::primes;
use crate::register_problem;

register_problem!(10, "Summation of primes", problem010);

pub fn problem010() -> String {
    // The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
    //
    // Find the sum of all the primes below two million.
    let mut solution: u64 = 0;
    primes::crible2(2000000, |p: u64| solution += p);
    solution.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem010() {
        let result = problem010();
        assert_eq!(result, "142913828922");
    }
}