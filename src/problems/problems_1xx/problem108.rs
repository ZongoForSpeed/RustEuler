use crate::maths::arithmetic::Arithmetic;
use crate::maths::primes::crible23;
use crate::register_problem;

register_problem!(108, "Diophantine reciprocals I", problem108);

pub fn problem108() -> String {
    // In the following equation x, y, and n are positive integers.
    //
    //                                          1/x + 1/y = 1/n
    //
    // For n = 4 there are exactly three distinct solutions:
    //
    //      1/5 + 1/20 = 1/4
    //      1/6 + 1/12 = 1/4
    //      1/8 + 1/8 = 1/4
    //
    // What is the least value of n for which the number of distinct solutions exceeds one-thousand?
    //
    // NOTE: This problem is an easier version of Problem 110; it is strongly advised that you solve this one first.
    let mut primes = Vec::new();
    crible23(1000000, |p| primes.push(p));

    let mut result = 0;
    for n in 2.. {
        let mut solution = 1;
        n.factorization(&primes, |_, e| {
            solution *= 2 * e + 1;
        });

        solution = (solution + 1) / 2;
        if solution > 1000 {
            result = n;
            break;
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem108() {
        let result = problem108();
        assert_eq!(result, "180180");
    }
}
