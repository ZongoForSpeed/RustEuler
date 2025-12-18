use crate::maths::digits::extract_digits;
use crate::maths::primes::crible235;
use crate::maths::timer::ScopeTimer;

fn is_permutation<T: Ord + Clone>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut a_sorted = a.to_vec();

    a_sorted.sort();

    a_sorted == b
}

fn pandigital(n: u32, pandigits: &Vec<u32>) -> bool {
    let chiffres = Vec::from_iter(extract_digits(n, 10));
    is_permutation(&chiffres, &pandigits[0..chiffres.len()])
}

pub fn problem041() -> u32 {
    let _timer = ScopeTimer::new("Problem 41 Pandigital prime", false);
    // We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once.
    // For example, 2143 is a 4-digit pandigital and is also prime.
    //
    // What is the largest n-digit pandigital prime that exists?
    let mut primes: Vec<u32> = Vec::new();
    crible235(10000000, |p| primes.push(p));

    let pandigits = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    primes
        .into_iter()
        .rev()
        .filter(|p| pandigital(*p, &pandigits))
        .next()
        .unwrap()
}
