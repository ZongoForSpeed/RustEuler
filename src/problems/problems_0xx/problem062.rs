use crate::maths::digits::{is_permutation, Digits};
use crate::register_problem;

pub fn cubic_permutations() -> Vec<u64> {
    let mut cubes: Vec<u64> = Vec::new();

    let mut taille = 1;

    for n in 1.. {
        let n3 = n * n * n;
        let nb = n3.count_digits(10);
        if nb != taille {
            let len = cubes.len();
            for i in 0..len {
                let mut result: Vec<u64> = Vec::new();
                let c1 = cubes[i];
                result.push(c1);
                for j in i + 1..len {
                    let c2 = cubes[j];
                    if is_permutation(c1, c2, 10) {
                        result.push(c2);
                    }
                }
                if result.len() > 4 {
                    return result;
                }
            }
            cubes.clear();
            taille = nb;
        }
        cubes.push(n3);
    }

    vec![]
}

register_problem!(62, "Cubic permutations", problem062);

pub fn problem062() -> String {
    // The cube, 41063625 (345^3), can be permuted to produce two other cubes: 56623104 (384^3) and
    // 66430125 (405^3). In fact, 41063625 is the smallest cube which has exactly three permutations
    // of its digits which are also cube.
    //
    // Find the smallest cube for which exactly five permutations of its digits are cube.
    let result = cubic_permutations();
    result.first().unwrap().to_string()
}
