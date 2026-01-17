use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;

register_problem!(55, "Lychrel numbers", problem055);

pub fn problem055() -> String {
    // If we take 47, reverse and add, 47 + 74 = 121, which is palindromic.
    //
    // Not all numbers produce palindromes so quickly. For example,
    //
    //      349 + 943 = 1292,
    //      1292 + 2921 = 4213
    //      4213 + 3124 = 7337
    //
    // That is, 349 took three iterations to arrive at a palindrome.
    //
    // Although no one has proved it yet, it is thought that some numbers, like 196, never produce
    // a palindrome. A number that never forms a palindrome through the reverse and add process
    // is called a Lychrel number. Due to the theoretical nature of these numbers, and for the
    // purpose of this problem, we shall assume that a number is Lychrel until proven otherwise.
    // In addition you are given that for every number below ten-thousand, it will either (i) become
    // a palindrome in less than fifty iterations, or, (ii) no one, with all the computing power that
    // exists, has managed so far to map it to a palindrome. In fact, 10677 is the first number to be
    // shown to require over fifty iterations before producing a palindrome: 4668731596684224866951378664
    // (53 iterations, 28-digits).
    //
    // Surprisingly, there are palindromic numbers that are themselves Lychrel numbers;
    // the first example is 4994.
    //
    // How many Lychrel numbers are there below ten-thousand?
    let mut count = 0;
    for n in 1..10000 {
        let mut lychrel = false;
        let mut m = MpzNumber::from_u64(n);
        for _ in 1..50 {
            m = m.invert(10) + m;
            if m.palindrome(10) {
                lychrel = true;
                break;
            }
        }
        if !lychrel {
            count += 1;
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem055() {
        let result = problem055();
        assert_eq!(result, "249");
    }
}
