use crate::maths::primes::{crible2};
use crate::register_problem;

register_problem!(249, "Prime Subset Sums", problem249);

/// Let S = {2, 3, 5, ..., 4999} be the set of prime numbers less than 5000.
///
/// Find the number of subsets of S, the sum of whose elements is a prime number.
/// Enter the rightmost 16 digits as your answer.
pub fn problem249() -> String {
    let mask = usize::pow(10, 16);

    let mut primes: Vec<usize> = Vec::new();
    crible2(5000, |p| primes.push(p));

    let sum = primes.iter().sum::<usize>();

    println!("Sum: {}", sum);

    let mut t = vec![0; sum + 1];
    t[0] = 1;

    let mut sp = 0;
    for &p in &primes {
        sp += p;
        let mut j = sp;
        while j > p - 1 {
            t[j] = (t[j] + t[j - p]) % mask;
            j = j - 1;
        }
    }
    
    primes.clear();
    crible2(sum, |p| primes.push(p));

    let mut result = 0;
    for p in primes {
        result = (result + t[p]) % mask;
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem249::problem249;

    #[test]
    fn test_problem249() {
        let result = problem249();
        assert_eq!(result, "9275262564250418");
    }
}
