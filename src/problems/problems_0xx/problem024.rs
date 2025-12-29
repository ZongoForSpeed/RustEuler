use crate::maths::digits::conversion;
use crate::register_problem;
use crate::utils::permutations::permutations;

register_problem!(24, "Lexicographic permutations", problem024);

pub fn problem024() -> String {
    // A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits
    // 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically, we call it lexicographic
    // order. The lexicographic permutations of 0, 1 and 2 are:
    //
    //                                    012   021   102   120   201   210
    //
    // What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
    let v: Vec<u64> = Vec::from_iter(0..10);
    let option = permutations(v).nth(999999);

    println!("v = {:?}", option);
    conversion(&option.unwrap(), 10).to_string()
}
