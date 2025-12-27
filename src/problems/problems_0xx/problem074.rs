use crate::maths::digits::Digits;
use crate::register_problem;
use std::collections::HashSet;

register_problem!(74, "Digit factorial chains", problem074);

pub fn problem074() -> String {
    // The number 145 is well known for the property that the sum of the factorial of its digits is
    // equal to 145:
    //
    //                                  1! + 4! + 5! = 1 + 24 + 120 = 145
    //
    // Perhaps less well known is 169, in that it produces the longest chain of numbers that link
    // back to 169; it turns out that there are only three such loops that exist:
    //
    //      169 → 363601 → 1454 → 169
    //      871 → 45361 → 871
    //      872 → 45362 → 872
    //
    // It is not difficult to prove that EVERY starting number will eventually get stuck in a loop.
    // For example,
    //
    //      69 → 363600 → 1454 → 169 → 363601 (→ 1454)
    //      78 → 45360 → 871 → 45361 (→ 871)
    //      540 → 145 (→ 145)
    //
    // Starting with 69 produces a chain of five non-repeating terms, but the longest non-repeating
    // chain with a starting number below one million is sixty terms.
    //
    // How many chains, with a starting number below one million, contain exactly sixty
    // non-repeating terms?
    let factorial = vec![1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

    let mut result = 0;

    for n in 1..1000000 {
        let mut chain: HashSet<u32> = HashSet::new();
        let mut m = n;
        while chain.insert(m) {
            let mut s = 0;
            m.loop_digits(10, |d| {
                s += factorial[d as usize];
            });
            m = s;
        }

        if chain.len() == 60 {
            result += 1;
        }
    }
    result.to_string()
}
