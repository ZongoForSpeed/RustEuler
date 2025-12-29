use crate::maths::digits::conversion;
use crate::maths::primes::Primes;
use crate::register_problem;
use permutohedron::LexicalPermutation;
use std::collections::HashSet;

register_problem!(118, "Pandigital prime sets", problem118);

fn test(digits: &[u64]) -> HashSet<Vec<u64>> {
    let mut result = HashSet::new();
    let p = conversion(digits, 10);
    if p.miller_rabin(15) {
        result.insert(vec![p]);
    }

    for j in 1..digits.len() {
        let p = conversion(&digits[0..j], 10);
        if p.miller_rabin(15) {
            let s = test(&digits[j..]);
            if !s.is_empty() {
                for mut v in s {
                    v.push(p);
                    v.sort();
                    result.insert(v);
                }
            }
        }
    }

    result
}

pub fn problem118() -> String {
    // Using all of the digits 1 through 9 and concatenating them freely to form decimal integers,
    // different sets can be formed. Interestingly with the set {2,5,47,89,631}, all of the elements
    // belonging to it are prime.
    //
    // How many distinct sets containing each of the digits one through nine exactly once contain
    // only prime elements?
    let mut result = HashSet::new();

    let mut digits = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    loop {
        let s = test(&digits);
        result.extend(s);
        if !digits.next_permutation() {
            break;
        }
    }

    result.len().to_string()
}
