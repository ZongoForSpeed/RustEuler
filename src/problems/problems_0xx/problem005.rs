use crate::maths::arithmetic::Arithmetic;
use crate::register_problem;

register_problem!(5, "Smallest multiple", problem005);

pub fn problem005() -> String {
    // 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without
    // any remainder.
    //
    // What is the smallest positive number that is evenly divisible by all of the numbers from 1
    // to 20?
    let mut solution:u64 = 2;
    for d in 3..=20 {
        solution = u64::lcm(solution, d);
    }
    solution.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem005() {
        let result = problem005();
        assert_eq!(result, "232792560");
    }
}