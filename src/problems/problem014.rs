use crate::maths::timer::ScopeTimer;
use std::collections::HashMap;

pub fn problem014() -> u64 {
    let _timer = ScopeTimer::new("Problem 14 Longest Collatz sequence", false);
    // The following iterative sequence is defined for the set of positive integers:
    //
    // n -> n/2 (n is even)
    // n -> 3n + 1 (n is odd)
    //
    // Using the rule above and starting with 13, we generate the following sequence:
    //
    // 13 -> 40 -> 20 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
    // It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
    // Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
    //
    // Which starting number, under one million, produces the longest chain?
    //
    // NOTE: Once the chain starts the terms are allowed to go above one million.
    let mut cache: HashMap<u64, u64> = HashMap::new();
    cache.insert(1, 1);

    let mut max_longueur: u64 = 1;
    let mut max_nombre: u64 = 1;
    for n in 2..1000000 {
        let mut chaine: Vec<u64> = Vec::new();
        chaine.push(n);
        let mut p = n;
        while !cache.contains_key(&p) {
            if p % 2 == 0 {
                p /= 2;
            } else {
                p = 3 * p + 1;
            }
            chaine.push(p);
        }
        
        if let Some(length) = cache.get_mut(&p) {
            let mut mut_longueur = length.clone();
            for c in chaine.iter().rev() {
                mut_longueur += 1;
                cache.insert(*c, mut_longueur);
            }
        }

        if let Some(length) = cache.get(&n) {
            if *length > max_longueur {
                max_longueur = *length;
                max_nombre = n;
            }
        }
    }

    max_nombre
}
