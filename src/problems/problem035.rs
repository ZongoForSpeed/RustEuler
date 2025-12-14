use std::collections::HashSet;
use crate::maths::chiffres::{conversion, extraire_chiffres};
use crate::maths::premiers::crible2;
use crate::maths::timer::ScopeTimer;

fn rotation(n: u64) -> u64 {
    let mut chiffres = extraire_chiffres(n, 10);

    let front = chiffres.pop_front().unwrap();
    chiffres.push_back(front);

    conversion(&chiffres, 10)
}

fn valide(premiers: &HashSet<u64>, p: u64) -> bool {
    let chiffres = extraire_chiffres(p, 10);
    if chiffres.contains(&0) {
        return false;
    }

    let mut tmp = p;
    loop {
        tmp = rotation(tmp);
        if tmp == p {
            return true;
        }
        if !premiers.contains(&tmp) {
            return false;
        }
    }
}

pub fn problem035() -> u64 {
    let _timer = ScopeTimer::new("Problem 35 Circular primes", false);
    // The number, 197, is called a circular prime because all rotations of the digits:
    // 197, 971, and 719, are themselves prime.
    //
    // There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
    //
    // How many circular primes are there below one million?
    let mut premiers: HashSet<u64> = HashSet::new();
    crible2(1000000, |p| {
        premiers.insert(p);
    });

    let mut result = 0;
    for &p in &premiers {
        if valide(&premiers, p) {
            result += 1;
        }
    }
    result
}
