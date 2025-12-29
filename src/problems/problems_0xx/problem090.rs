use crate::register_problem;
use crate::utils::permutations::permutations;
use bit_set::BitSet;
use std::collections::HashSet;

register_problem!(90, "Cube digit pairs", problem090);

fn test(de1: &BitSet<u8>, de2: &BitSet<u8>, c1: usize, c2: usize) -> bool {
    (de1.contains(c1) && de2.contains(c2)) || (de1.contains(c2) && de2.contains(c1))
}

pub fn problem090() -> String {
    // Each of the six faces on a cube has a different digit (0 to 9) written on it; the same is
    // done to a second cube. By placing the two cubes side-by-side in different positions we can
    // form a variety of 2-digit numbers.
    //
    // For example, the square number 64 could be formed:
    //
    // In fact, by carefully choosing the digits on both cubes it is possible to display all of the
    // square numbers below one-hundred: 01, 04, 09, 16, 25, 36, 49, 64, and 81.
    //
    // For example, one way this can be achieved is by placing {0, 5, 6, 7, 8, 9} on one cube and
    // {1, 2, 3, 4, 8, 9} on the other cube.
    //
    // However, for this problem we shall allow the 6 or 9 to be turned upside-down so that an
    // arrangement like {0, 5, 6, 7, 8, 9} and {1, 2, 3, 4, 6, 7} allows for all nine square
    // numbers to be displayed; otherwise it would be impossible to obtain 09.
    //
    // In determining a distinct arrangement we are interested in the digits on each cube, not the
    // order.
    //
    //      {1, 2, 3, 4, 5, 6} is equivalent to {3, 6, 4, 1, 2, 5}
    //      {1, 2, 3, 4, 5, 6} is distinct from {1, 2, 3, 4, 5, 9}
    //
    // But because we are allowing 6 and 9 to be reversed, the two distinct sets in the last example
    // both represent the extended set {1, 2, 3, 4, 5, 6, 9} for the purpose of forming 2-digit
    // numbers.
    //
    // How many distinct arrangements of the two cubes allow for all of the square numbers to be
    // displayed?
    let faces = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut cubes: HashSet<BitSet<u8>> = HashSet::new();
    for permute in permutations(faces) {
        cubes.insert(BitSet::from_iter(permute[0..6].iter().cloned()));
    }

    let dices = Vec::from_iter(cubes.iter().cloned());
    let len = dices.len();

    let mut result: usize = 0;
    for i in 0..len {
        for j in i..len {
            let die1 = &dices[i];
            let die2 = &dices[j];
            if !test(die1, die2, 0, 1) {
                continue;
            } else if !test(die1, die2, 0, 4) {
                continue;
            } else if !test(die1, die2, 0, 9) && !test(die1, die2, 0, 6) {
                continue;
            } else if !test(die1, die2, 1, 6) && !test(die1, die2, 1, 9) {
                continue;
            } else if !test(die1, die2, 2, 5) {
                continue;
            } else if !test(die1, die2, 3, 6) && !test(die1, die2, 3, 9) {
                continue;
            } else if !test(die1, die2, 4, 9) && !test(die1, die2, 4, 6) {
                continue;
            } else if !test(die1, die2, 6, 4) && !test(die1, die2, 9, 4) {
                continue;
            } else if !test(die1, die2, 8, 1) {
                continue;
            }

            result += 1;
        }
    }

    result.to_string()
}
