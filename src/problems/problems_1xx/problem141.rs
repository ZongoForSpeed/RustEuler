use crate::maths::arithmetic::Arithmetic;
use crate::register_problem;
use std::collections::HashSet;
use crate::maths::polygonal::Polygonal;

register_problem!(141, "Investigating progressive numbers, n, which are also square", problem141);

pub fn problem141() -> String {
    // A positive integer, n, is divided by d and the quotient and remainder are q and r respectively. 
    // In addition d, q, and r are consecutive positive integer terms in a geometric sequence, but not necessarily in
    // that order.
    //
    // For example, 58 divided by 6 has quotient 9 and remainder 4. It can also be seen that 4, 6, 9 are consecutive
    // terms in a geometric sequence (common ratio 3/2).
    // We will call such numbers, n, progressive.
    //
    // Some progressive numbers, such as 9 and 10404 = 102², happen to also be perfect squares.
    // The sum of all progressive perfect squares below one hundred thousand is 124657.
    //
    // Find the sum of all progressive perfect squares below one trillion (1012).
    let limit:u64 = 1000000000000;
    
    let mut solutions = HashSet::<u64>::new();
    for t in (1..).take_while(|t| t * t * (1 + t * t) < limit) {
        for s in (t+1..).take_while(|s| t * (t + s * s * s) < limit) {
            if u64::gcd(t, s) == 1 {
                let s_cube = s*s*s;
                for v in (1..).take_while(|v| v * t * (t + s_cube * v) < limit) {
                    let n = v * t * (t + s_cube * v);
                    if n.is_square() {
                        solutions.insert(n);
                    }
                }
            }
        }
    }
    
    solutions.iter().sum::<u64>().to_string()
}
