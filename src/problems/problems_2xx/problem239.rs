use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;

register_problem!(239, "Infinite string tour", problem239);

fn parity(n: u64) -> i8 {
    if n % 2 == 0 { 1 } else { -1 }
}

pub fn problem239() -> String {
    // A set of disks numbered 1 through 100 are placed in a line in random order.
    //
    // What is the probability that we have a partial derangement such that exactly 22 prime number
    // discs are found away from their natural positions? (Any number of non-prime disks may also be
    // found in or out of their natural positions.)
    //
    // Give your answer rounded to 12 places behind the decimal point in the form 0.abcdefghijkl.
    let denominator = MpzNumber::factorial(100);

    let mut numerator = MpzNumber::new();
    for n in 0..=22 {
        numerator += parity(n) * MpzNumber::binomial_ui(22, n) * MpzNumber::factorial(97 - n);
    }

    numerator *= MpzNumber::binomial_ui(25, 22);

    let mask = u64::pow(10, 12);
    numerator *= mask;
    numerator /= denominator;

    let result = numerator.get_f64() / mask as f64;
    format!("{:.12}", result)
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem239::problem239;

    #[test]
    fn test_problem239() {
        let result = problem239();
        assert_eq!(result, "0.001887854841");
    }
}
