use crate::maths::premiers::crible235;
use crate::maths::timer::ScopeTimer;
use crate::maths::chiffres;
use std::collections::VecDeque;
use chiffres::{conversion, extraire_chiffres};

fn is_permutation<T: Ord + Clone>(a: &VecDeque<T>, b: &VecDeque<T>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut a_sorted = Vec::from_iter(a);
    let mut b_sorted = Vec::from_iter(b);

    a_sorted.sort();
    b_sorted.sort();

    a_sorted == b_sorted
}


pub fn problem049() -> u64 {
    let _timer = ScopeTimer::new("Problem 49 Prime permutations", false);
    // The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330,
    // is unusual in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit
    // numbers are permutations of one another.
    //
    // There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting
    // this property, but there is one other 4-digit increasing sequence.
    //
    // What 12-digit number do you form by concatenating the three terms in this sequence?
    let limite = 10000;

    let mut premiers: Vec<u64> = Vec::new();
    crible235(limite, |p| premiers.push(p));

    let mut suites_premiers : Vec<Vec<u64>> = Vec::new();
    let size = premiers.len();
    for i in 0..size {
        let p = premiers[i];
        if p > 1000 {
            let mut suite: Vec<u64> = vec![p];
            let chiffres = extraire_chiffres(p, 10);
            for j in i+1..size {
                let q = premiers[j];
                if is_permutation(&chiffres, &extraire_chiffres(q, 10)) {
                    suite.push(q);
                }
            }

            if suite.len() > 2 {
                println!("Suites: {:?}", suite);
                suites_premiers.push(suite);
            }
        }
    }

    let mut result = Vec::<u64>::new();
    for s in suites_premiers {
        if s.len() == 3 && (s[2] - s[1]) == (s[1] - s[0]) {
            result = s;
        }
    }

    println!("result: {:?}", result);
    conversion(&result, 10000)
}
