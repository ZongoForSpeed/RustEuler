use crate::maths::digits::{Digits, conversion};
use crate::maths::primes::crible2;
use crate::register_problem;
use std::collections::BTreeSet;

register_problem!(51, "Prime digit replacements", problem051);

pub fn problem051() -> String {
    // By replacing the 1st digit of the 2-digit number *3, it turns out that six of the nine
    // possible values: 13, 23, 43, 53, 73, and 83, are all prime.
    //
    // By replacing the 3rd and 4th digits of 56**3 with the same digit, this 5-digit number
    // is the first example having seven primes among the ten generated numbers, yielding the
    // family: 56003, 56113, 56333, 56443, 56663, 56773, and 56993. Consequently 56003, being
    // the first member of this family, is the smallest prime with this property.
    //
    // Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits)
    // with the same digit, is part of an eight prime value family.
    let limite = 1000000;

    let mut primes: BTreeSet<u64> = BTreeSet::new();
    crible2(limite, |p| {
        primes.insert(p);
    });

    let mut result = 0;
    let mut found = false;

    for p in &primes {
        let chiffres = p.extract_digits(10);
        let unique = chiffres.iter().cloned().collect::<BTreeSet<_>>();
        for c in unique {
            if c != 0 {
                let chiffres2 = chiffres
                    .iter()
                    .cloned()
                    .map(|d| return if d == c { 0 } else { d })
                    .collect::<Vec<_>>();
                let q = conversion(&chiffres2, 10);
                let diff = (p - q) / c;
                let mut count = 0;
                for n in c..10 {
                    let i = q + n * diff;
                    if primes.contains(&i) {
                        count += 1;
                    }
                }

                if count == 8 {
                    println!("p={}, q={}, count={}", p, q, count);
                    result = *p;
                    found = true;
                    break;
                }
            }
        }

        if found {
            break;
        }
    }

    result.to_string()
}
