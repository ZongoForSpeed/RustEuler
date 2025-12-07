use crate::maths::chiffres::conversion;
use crate::maths::timer::ScopeTimer;
use permutohedron::LexicalPermutation;
use std::collections::HashSet;

pub fn problem032() -> u64 {
    let _timer = ScopeTimer::new("Problem 32 Coin sums", false);
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
    let mut v: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut result:HashSet<u64> = HashSet::new();
    loop {
        for i in 1..8 {
            let a = conversion(&v[0..i], 10);
            for j in i+1..9 {
                let b = conversion(&v[i..j], 10);
                let c = conversion(&v[j..], 10);
                if a * b == c {
                    println!("{} * {} = {}", a, b, c);
                    result.insert(c);
                }
            }
        }
        if !v.next_permutation() {
            break;
        }
    }
    result.iter().sum()
}
