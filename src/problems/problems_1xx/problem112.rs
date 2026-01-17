use crate::maths::digits::Digits;
use crate::register_problem;
use fraction::Fraction;

register_problem!(112, "Bouncy numbers", problem112);

fn is_sorted(d: &Vec<u64>) -> bool {
    d.windows(2).all(|w| w[0] <= w[1])
        || d.windows(2).all(|w| w[0] >= w[1])
}

pub fn problem112() -> String {
    // Working from left-to-right if no digit is exceeded by the digit to its left it is called an
    // increasing number; for example, 134468.
    //
    // Similarly if no digit is exceeded by the digit to its right it is called a decreasing number;
    // for example, 66420.
    //
    // We shall call a positive integer that is neither increasing nor decreasing a "bouncy" number;
    // for example, 155349.
    //
    // Clearly there cannot be any bouncy numbers below one-hundred, but just over half of the numbers
    // below one-thousand (525) are bouncy. In fact, the least number for which the proportion of
    // bouncy numbers first reaches 50% is 538.
    //
    // Surprisingly, bouncy numbers become more and more common and by the time we reach 21780 the
    // proportion of bouncy numbers is equal to 90%.
    //
    // Find the least number for which the proportion of bouncy numbers is exactly 99%.
    let limit = Fraction::new(99u64, 100u64);

    let mut ratio_numer: u64 = 0;
    for n in 1.. {
        let digits = Vec::from_iter(n.extract_digits(10));
        if !is_sorted(&digits) {
            ratio_numer += 1;
        }
        if Fraction::new(ratio_numer, n) >= limit {
            return n.to_string();
        }
    }

    panic!("did not find the correct number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem112() {
        let result = problem112();
        assert_eq!(result, "1587000");
    }
}
