use crate::maths::arithmetic::Arithmetic;
use crate::maths::primes::crible235;
use crate::register_problem;

register_problem!(110, "Diophantine reciprocals II", problem110);

pub fn problem110() -> String {
    // In the following equation x, y, and n are positive integers.
    //
    //                                          1/x + 1/y = 1/n
    //
    // It can be verified that when n = 1260 there are 113 distinct solutions and this is the least value of n for which
    // the total number of distinct solutions exceeds one hundred.
    //
    // What is the least value of n for which the number of distinct solutions exceeds four million?
    //
    // NOTE: This problem is a much more difficult version of Problem 108 and as it is well beyond the limitations of a
    // brute force approach it requires a clever implementation.
    let limit = 4000000;
    let mut primes:Vec<u64> = Vec::new();
    crible235(limit * 3, |p| primes.push(p));

    let mut solution = 0;
    let mut result = 0;
    while solution < limit {
        result += 2036934900; // 2^2 * 3^2 * 5^2 * 7^2 * 11 * 13 * 17

        solution = 1;
        result.factorization(&primes, |_, e| {
            solution *= 2 * e + 1;
        });
        solution /= 2;
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem110() {
        let result = problem110();
        assert_eq!(result, "9350130049860600");
    }
}
