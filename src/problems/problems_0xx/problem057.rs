use crate::register_problem;
use crate::utils::mpq_fraction::MpqFraction;

register_problem!(57, "Powerful digit sum", problem057);

pub fn problem057() -> String {
    // It is possible to show that the square root of two can be expressed as an infinite continued fraction.
    //
    //                              √ 2 = 1 + 1/(2 + 1/(2 + 1/(2 + ... ))) = 1.414213...
    //
    // By expanding this for the first four iterations, we get:
    //
    //      1 + 1/2 = 3/2 = 1.5
    //      1 + 1/(2 + 1/2) = 7/5 = 1.4
    //      1 + 1/(2 + 1/(2 + 1/2)) = 17/12 = 1.41666...
    //      1 + 1/(2 + 1/(2 + 1/(2 + 1/2))) = 41/29 = 1.41379...
    //
    // The next three expansions are 99/70, 239/169, and 577/408, but the eighth expansion, 1393/985, is the
    // first example where the number of digits in the numerator exceeds the number of digits in the denominator.
    //
    // In the first one-thousand expansions, how many fractions contain a numerator with more digits than denominator?
    let mut f = MpqFraction::from_i64(3, 2);
    let mut count = 0;
    for _ in 1..1000 {
        f = 1 + 1 / (f + 1);
        if f.numerator().number_digits(10) > f.denominator().number_digits(10) {
            count += 1;
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem057() {
        let result = problem057();
        assert_eq!(result, "153");
    }
}
