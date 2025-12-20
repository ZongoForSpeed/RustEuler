use crate::{maths, register_problem};
use maths::arithmetique;

register_problem!(5, "Smallest multiple", problem005);

pub fn problem005() -> String {
    // 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without
    // any remainder.
    //
    // What is the smallest positive number that is evenly divisible by all of the numbers from 1
    // to 20?
    let mut solution:u64 = 2;
    for d in 3..=20 {
        solution = arithmetique::ppcm(solution, d);
    }
    solution.to_string()
}
