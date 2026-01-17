use crate::maths::arithmetic::Arithmetic;
use crate::maths::primes;
use crate::register_problem;
use bit_set::BitSet;

register_problem!(23, "Non-abundant sums", problem023);

pub fn problem023() -> String {
    // A perfect number is a number for which the sum of its proper divisors is exactly equal to the number.
    // For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a
    // perfect number.
    //
    // A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this
    // sum exceeds n.
    //
    // As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum
    // of two abundant numbers is 24. By mathematical analysis, it can be shown that all integers greater than 28123 can
    // be written as the sum of two abundant numbers. However, this upper limit cannot be reduced any further by analysis
    // even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is
    // less than this limit.
    //
    // Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.
    let limite = 28123;
    let mut primes: Vec<usize> = Vec::new();
    primes::crible2(limite, |p| primes.push(p));

    let mut abundant: Vec<usize> = Vec::new();
    for n in 12..limite {
        if n.sum_of_divisors(&primes) > 2 * n {
            abundant.push(n);
        }
    }

    let mut test: BitSet<usize> = BitSet::from_iter(1..limite);

    let len = abundant.len();
    for i in 0..len {
        let n = abundant[i];
        for j in 0..=i {
            let m = abundant[j];
            if n + m < limite {
                test.remove(n + m);
            } else {
                break;
            }
        }
    }

    let result: usize = test.iter().sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem023() {
        let result = problem023();
        assert_eq!(result, "4179871");
    }
}