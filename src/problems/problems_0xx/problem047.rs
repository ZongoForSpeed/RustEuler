use crate::maths::primes::crible235;
use crate::register_problem;
use std::collections::{BTreeSet, HashSet, VecDeque};
use crate::maths::arithmetic::Arithmetic;

register_problem!(47, "Distinct primes factors", problem047);

pub fn problem047() -> String {
    // The first two consecutive numbers to have two distinct prime factors are:
    //
    //              14 = 2 × 7
    //              15 = 3 × 5
    //
    // The first three consecutive numbers to have three distinct prime factors are:
    //
    //              644 = 2² × 7 × 23
    //              645 = 3 × 5 × 43
    //              646 = 2 × 17 × 19.
    //
    // Find the first four consecutive integers to have four distinct prime factors.
    // What is the first of these numbers?
    let mut primes: BTreeSet<u64> = BTreeSet::new();
    crible235(1000000, |p| {
        primes.insert(p);
    });

    let mut result = 0;

    let mut decomposition: VecDeque<Vec<u64>> = VecDeque::new();

    for n in 2.. {
        {
            let mut f: Vec<u64> = Vec::new();
            n.factorization(&primes, |p, c| {
                f.push(p.pow(c as u32));
            });
            decomposition.push_back(f);
        }

        if decomposition.len() > 4 {
            decomposition.pop_front();
        }

        let mut facteur: HashSet<u64> = HashSet::new();
        for i in &decomposition {
            if i.len() != 4 {
                break;
            }
            for f in i {
                facteur.insert(*f);
            }
        }
        if facteur.len() == 16 {
            result = n - 3;
            break;
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem047() {
        let result = problem047();
        assert_eq!(result, "134043");
    }
}
// ... existing code ...