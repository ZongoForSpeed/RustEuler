use std::collections::{BTreeSet, HashMap, HashSet};
use crate::maths::primes::crible23;
use crate::register_problem;

register_problem!(77, "Prime summations", problem077);

pub fn problem077() -> String {
    // It is possible to write ten as the sum of primes in exactly five different ways:
    //
    //      7 + 3
    //      5 + 5
    //      5 + 3 + 2
    //      3 + 3 + 2 + 2
    //      2 + 2 + 2 + 2 + 2
    //
    // What is the first value which can be written as the sum of primes in over five thousand
    // different ways?
    let mut primes: BTreeSet<u32> = BTreeSet::new();
    crible23(1000000, |p| {primes.insert(p);});

    let mut solutions: HashMap<u32, HashSet<Vec<u32>>> = HashMap::new();
    for n in 4.. {
        let mut sol_n: HashSet<Vec<u32>> = HashSet::new();
        for p in &primes {
            if *p >= n {
                break;
            }

            if primes.contains(&(n - p)) {
                let mut s =vec![n - p, *p];
                s.sort();
                sol_n.insert(s);
            }

            if let Some(v) = solutions.get(&(n - p)) {
                for s in v {
                    let mut ss = s.clone();
                    ss.push(*p);
                    ss.sort();
                    sol_n.insert(ss);
                }
            }
        }

        if sol_n.len() > 5000 {
            // println!("{:?}", sol_n);
            return n.to_string();
        }

        solutions.insert(n, sol_n);
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem077() {
        let result = problem077();
        assert_eq!(result, "71");
    }
}
