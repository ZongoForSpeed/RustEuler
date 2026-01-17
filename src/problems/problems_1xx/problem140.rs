use crate::maths::fibonacci::fibonacci;
use crate::register_problem;
use fraction::Fraction;
use num_traits::{ConstOne, ToPrimitive};

register_problem!(140, "Modified Fibonacci golden nuggets", problem140);

fn nugget(mut x: Fraction) -> u64 {
    let three = Fraction::from(3);
    x = (Fraction::ONE + three * x) * x / (Fraction::ONE - x - x * x);
    x.to_u64().unwrap()
}

pub fn problem140() -> String {
    // Consider the infinite polynomial series AG(x) = xG1 + x2G2 + x3G3 + ..., where Gk is the kth term of the second
    // order recurrence relation Gk = Gk−1 + Gk−2, G1 = 1 and G2 = 4; that is, 1, 4, 5, 9, 14, 23, ... .
    //
    // For this problem we shall be concerned with values of x for which AG(x) is a positive integer.
    //
    // The corresponding values of x for the first five natural numbers are shown below.
    //
    //                                          x           AG(x)
    //                                          (√5−1)/4	1
    //                                          2/5         2
    //                                          (√22−2)/6	3
    //                                          (√137−5)/14	4
    //                                          1/2	        5
    //
    // We shall call AG(x) a golden nugget if x is rational, because they become increasingly rarer; for example, the
    // 20th golden nugget is 211345365.
    //
    // Find the sum of the first thirty golden nuggets.
    let mut fibo: Vec<u64> = Vec::new();
    for f in fibonacci() {
        fibo.push(f);
        if fibo.len() > 80 {
            break;
        }
    }

    let objective = 30;
    let mut result = 0;

    for (n, i) in (3..objective + 2).step_by(2).zip((4..).step_by(4)) {
        let v = nugget(Fraction::new(fibo[n - 1], fibo[n]));
        result += 2 * v - fibo[i];
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem140() {
        let result = problem140();
        assert_eq!(result, "5673835352990");
    }
}
