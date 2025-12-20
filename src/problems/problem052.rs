use crate::maths::digits::is_permutation;
use crate::register_problem;

register_problem!(52, "Permuted multiples", problem052);

pub fn problem052() -> String {
    // It can be seen that the number, 125874, and its double, 251748, contain exactly the same
    // digits, but in a different order.
    //
    // Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits.
    let mut result: u64 = 0;
    for n in 1.. {
        let mut found = true;
        for k in 2..7 {
            if !is_permutation(n, n * k, 10) {
                found = false;
                break;
            }
        }
        if found {
            result = n;
            break;
        }
    }

    result.to_string()
}
