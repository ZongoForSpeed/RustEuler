use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;

register_problem!(53, "Combinatoric selections", problem053);

pub fn problem053() -> String {
    // There are exactly ten ways of selecting three from five, 12345:
    //
    // 123, 124, 125, 134, 135, 145, 234, 235, 245, and 345
    //
    // In combinatorics, we use the notation, 5C3 = 10.
    //
    // In general,
    //
    // nCr = n! / [r!(n−r)!] , where r ≤ n, n! = n×(n−1)×...×3×2×1, and 0! = 1.
    // It is not until n = 23, that a value exceeds one-million: 23C10 = 1144066.
    //
    // How many, not necessarily distinct, values of  nCr, for 1 ≤ n ≤ 100, are greater than one-million?
    let mut count = 0;
    let limit = MpzNumber::from_u64(1000000);
    for n in 1..=100 {
        for p in 0..=n {
            if MpzNumber::binomial_ui(n, p) > limit {
                count += 1;
            }
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem053() {
        let result = problem053();
        assert_eq!(result, "4075");
    }
}
