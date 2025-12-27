use std::path::Path;
use crate::register_problem;
use itertools::Itertools;
use crate::problems::problem103::verify;

register_problem!(105, "Special subset sums: testing", problem105);

pub fn problem105() -> String {
    // Let S(A) represent the sum of elements in set A of size n. We shall call it a special sum set if for any two
    // non-empty disjoint subsets, B and C, the following properties are true:
    //
    //      S(B) ≠ S(C); that is, sums of subsets cannot be equal.
    //      If B contains more elements than C then S(B) > S(C).
    // For example, {81, 88, 75, 42, 87, 84, 86, 65} is not a special sum set because 65 + 87 + 88 = 75 + 81 + 84,
    // whereas {157, 150, 164, 119, 79, 159, 161, 139, 158} satisfies both rules for all possible subset pair
    // combinations and S(A) = 1286.
    //
    // Using sets.txt (right click and "Save Link/Target As..."), a 4K text file with one-hundred sets containing seven
    // to twelve elements (the two examples given above are the first two sets in the file), identify all the special
    // sum sets, A1, A2, ..., Ak, and find the value of S(A1) + S(A2) + ... + S(Ak).
    //
    // NOTE: This problem is related to Problem 103 and Problem 106.
    let path = Path::new("data/p105_sets.txt");
    let mut result: u64 = 0;
    for line in std::fs::read_to_string(path).unwrap().lines() {
        let set = line.split(',').map(|s| s.parse::<u64>().unwrap()).sorted().collect::<Vec<u64>>();
        if verify(&set) {
            result += set.iter().sum::<u64>()
        }

    }

    result.to_string()

}
