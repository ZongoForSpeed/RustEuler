use crate::maths::digits;
use crate::maths::primes::crible235;
use crate::register_problem;
use digits::{conversion, extract_digits};
use std::collections::VecDeque;

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


register_problem!(49, "Prime permutations", problem049);

pub fn problem049() -> String {
    // The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330,
    // is unusual in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit
    // numbers are permutations of one another.
    //
    // There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting
    // this property, but there is one other 4-digit increasing sequence.
    //
    // What 12-digit number do you form by concatenating the three terms in this sequence?
    let limite = 10000;

    let mut primes: Vec<u64> = Vec::new();
    crible235(limite, |p| primes.push(p));

    let mut prime_permutations: Vec<Vec<u64>> = Vec::new();
    let size = primes.len();
    for i in 0..size {
        let p = primes[i];
        if p > 1000 {
            let mut suite: Vec<u64> = vec![p];
            let chiffres = extract_digits(p, 10);
            for j in i+1..size {
                let q = primes[j];
                if is_permutation(&chiffres, &extract_digits(q, 10)) {
                    suite.push(q);
                }
            }

            if suite.len() > 2 {
                println!("Suites: {:?}", suite);
                prime_permutations.push(suite);
            }
        }
    }

    let mut result = Vec::<u64>::new();
    for s in prime_permutations {
        if s.len() == 3 && (s[2] - s[1]) == (s[1] - s[0]) {
            result = s;
        }
    }

    println!("result: {:?}", result);
    conversion(&result, 10000).to_string()
}
