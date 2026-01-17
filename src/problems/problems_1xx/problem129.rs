use crate::maths::arithmetic::Arithmetic;
use crate::register_problem;

register_problem!(129, "Repunit divisibility", problem129);

pub fn problem129() -> String {
    // A number consisting entirely of ones is called a repunit. We shall define R(k) to be a repunit of length k;
    // for example, R(6) = 111111.
    //
    // Given that n is a positive integer and GCD(n, 10) = 1, it can be shown that there always exists a value, k, for
    // which R(k) is divisible by n, and let A(n) be the least such value of k; for example, A(7) = 6 and A(41) = 5.
    //
    // The least value of n for which A(n) first exceeds ten is 17.
    //
    // Find the least value of n for which A(n) first exceeds one-million.
    let limit = 1000000;
    for n in (999999..).step_by(2) {
        if n % 5 != 0 {
            let k = n.repunit_a(10);
            if k > limit {
                println!("A({}) = {}", n, k);
                return n.to_string();
            }
        }
    }

    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem129() {
        let result = problem129();
        assert_eq!(result, "1000023");
    }
}
