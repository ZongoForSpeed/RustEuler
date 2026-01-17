use crate::maths::digits::conversion;
use crate::register_problem;
use crate::utils::permutations::permutations;
use std::collections::HashSet;

register_problem!(32, "Coin sums", problem032);

pub fn problem032() -> String {
    // We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once; for
    // example, the 5-digit number, 15234, is 1 through 5 pandigital.
    //
    // The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand, multiplier, and product
    // is 1 through 9 pandigital.
    //
    // Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9
    // pandigital.
    //
    // HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.
    let v: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut result: HashSet<u64> = HashSet::new();
    for permute in permutations(v) {
        for i in 1..8 {
            let a = conversion(&permute[0..i], 10);
            for j in i + 1..9 {
                let b = conversion(&permute[i..j], 10);
                let c = conversion(&permute[j..], 10);
                if a * b == c {
                    println!("{} * {} = {}", a, b, c);
                    result.insert(c);
                }
            }
        }
    }
    let result: u64 = result.iter().sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem032() {
        let result = problem032();
        assert_eq!(result, "45228");
    }
}
