use crate::register_problem;

register_problem!(137, "Fibonacci golden nuggets", problem137);

pub fn problem137() -> String {
    // Consider the infinite polynomial series AF(x) = xF1 + x2F2 + x3F3 + ..., where Fk is the kth term in the
    // Fibonacci sequence: 1, 1, 2, 3, 5, 8, ... ; that is, Fk = Fk−1 + Fk−2, F1 = 1 and F2 = 1.
    //
    // For this problem we shall be interested in values of x for which AF(x) is a positive integer.
    //
    // Surprisingly AF(1/2) = (1/2).1 + (1/2)2.1 + (1/2)3.2 + (1/2)4.3 + (1/2)5.5 + ...
    //                      = 1/2 + 1/4 + 2/8 + 3/16 + 5/32 + ...
    //                      = 2
    //
    // The corresponding values of x for the first five natural numbers are shown below.
    //
    //                                      x       	AF(x)
    //                                      √2−1	    1
    //                                      1/2	        2
    //                                      (√13−2)/3	3
    //                                      (√89−5)/8	4
    //                                      (√34−3)/5	5
    //
    // We shall call AF(x) a golden nugget if x is rational, because they become increasingly rarer; for example, the
    // 10th golden nugget is 74049690.
    //
    // Find the 15th golden nugget.
    let mut f: (u64, u64) = (1, 2);
    for _ in 1..15 {
        f = (f.0 + f.1, f.0 + 2 * f.1);
    }

    (f.0 * f.1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem137() {
        let result = problem137();
        assert_eq!(result, "1120149658760");
    }
}
