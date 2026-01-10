use crate::maths::primes::crible2;
use crate::register_problem;
use bit_set::BitSet;

register_problem!(204, "Generalised Hamming Numbers", problem204);

fn find_hamming(mut hamming: &mut BitSet<u32>, primes: &Vec<usize>, n: usize, factor: usize, limit: usize) {
    if n >= primes.len() {
        return;
    }
    let p = primes[n];

    let mut f = factor;
    while f <= limit {
        hamming.insert(f);
        find_hamming(&mut hamming, primes, n + 1, f, limit);
        f *= p;
    }
}

pub fn problem204() -> String {
    // A Hamming number is a positive number which has no prime factor larger than 5.
    // So the first few Hamming numbers are 1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15.
    // There are 1105 Hamming numbers not exceeding 10^8.
    //
    // We will call a positive number a generalised Hamming number of type n, if it has no prime
    // factor larger than n. Hence the Hamming numbers are the generalised Hamming numbers of type 5.
    //
    // How many generalised Hamming numbers of type 100 are there which don't exceed 10^9?
    let limit = 1000000000;

    let mut primes:Vec<usize> = Vec::new();
    crible2(100, |p| primes.push(p));

    let mut hamming = BitSet::new();
    find_hamming(&mut hamming, &primes, 0, 1, limit);
    hamming.len().to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem204::problem204;

    #[test]
    fn test_problem204() {
        let result = problem204();
        assert_eq!(result, "2944730");
    }
}
