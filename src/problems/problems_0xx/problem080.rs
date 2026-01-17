use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;
use string_builder::Builder;
use crate::maths::polygonal::Polygonal;

register_problem!(80, "Square root digital expansion", problem080);

pub fn problem080() -> String {
    // It is well known that if the square root of a natural number is not an integer, then it is
    // irrational. The decimal expansion of such square roots is infinite without any repeating
    // pattern at all.
    //
    // The square root of two is 1.41421356237309504880..., and the digital sum of the first one
    // hundred decimal digits is 475.
    //
    // For the first one hundred natural numbers, find the total of the digital sums of the first
    // one hundred decimal digits for all the irrational square roots.
    let gogol = MpzNumber::power_ui(10, 100);

    let mut builder = Builder::default();
    for n in 1..100 {
        if !n.is_square() {
            let square = &gogol * &gogol * n;
            let sqrt = square.sqrt();
            builder.append(&sqrt.to_string()[0..100]);
        }
    }

    builder.string()
        .unwrap()
        .bytes()
        .map(|b| b - b'0')
        .map(|b| b as usize)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem080() {
        let result = problem080();
        assert_eq!(result, "40886");
    }
}
