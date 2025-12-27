use crate::register_problem;

register_problem!(1, "Multiples of 3 or 5", problem001);

pub fn problem001() -> String {
    // If we list all the natural numbers below 10 that are multiples of 3 or 5,
    // we get 3, 5, 6 and 9. The sum of these multiples is 23.
    //
    // Find the sum of all the multiples of 3 or 5 below 1000.
    let borne = 1000;
    let mut solution: u32 = 0;
    for n in 1..borne {
        if n % 3 == 0 || n % 5 == 0 {
            solution += n;
        }
    }
    solution.to_string()
}
