use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;

register_problem!(113, "Non-bouncy numbers", problem113);

pub fn problem113() -> String {
    // Working from left-to-right if no digit is exceeded by the digit to its left it is called an increasing number;
    // for example, 134468.
    //
    // Similarly if no digit is exceeded by the digit to its right it is called a decreasing number; for example, 66420.
    //
    // We shall call a positive integer that is neither increasing nor decreasing a "bouncy" number; for example, 155349.
    //
    // As n increases, the proportion of bouncy numbers below n increases such that there are only 12951 numbers below
    // one-million that are not bouncy and only 277032 non-bouncy numbers below 10^10.
    //
    // How many numbers below a googol (10^100) are not bouncy?
    let length = 100;
    let mut result = MpzNumber::new();

    for n in 0..length {
        for d in 0..10 {
            result += MpzNumber::binomial_ui(n + d, d);
        }
    }

    for n in 0..length {
        for d in 0..9 {
            result += MpzNumber::binomial_ui(n + d, d);
        }
    }

    result -= 10 * length;
    result.to_string()
}
