use crate::maths::digits::Digits;
use crate::maths::power::Power;
use crate::register_problem;
use num_integer::Roots;
use std::collections::HashSet;

register_problem!(125, "Palindromic sums", problem125);

pub fn problem125() -> String {
    // The palindromic number 595 is interesting because it can be written as the sum of
    // consecutive squares: 6² + 7² + 8² + 9² + 10² + 11² + 12².
    //
    // There are exactly eleven palindromes below one-thousand that can be written as
    // consecutive square sums, and the sum of these palindromes is 4164. Note that 1 = 0² + 1²
    // has not been included as this problem is concerned with the squares of positive integers.
    //
    // Find the sum of all the numbers less than 10^8 that are both palindromic and can be written
    // as the sum of consecutive squares.
    let limit: u64 = u64::power(10, 8);
    let mut squares = Vec::new();
    for n in 1..limit.sqrt() {
        squares.push(n * n);
    }
    let len = squares.len();

    let mut result = HashSet::new();
    for i in 0..len {
        let si = squares[i];
        let mut sum = si;
        for j in i + 1..len {
            let sj = squares[j];
            sum += sj;
            if sum > limit {
                break;
            } else if sum.palindrome(10) {
                result.insert(sum);
            }
        }
    }

    result.into_iter().sum::<u64>().to_string()
}
