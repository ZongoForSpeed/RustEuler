use crate::maths::digits::Digits;
use crate::register_problem;

register_problem!(92, "Square digit chains", problem092);

fn square_digits(n: u32) -> u32 {
    let mut s = 0;
    n.loop_digits(10, |d| s += d * d);
    s
}

pub fn problem092() -> String {
    // A number chain is created by continuously adding the square of the digits in a number to form a new number until
    // it has been seen before.
    //
    // For example,
    //
    //      44 → 32 → 13 → 10 → 1 → 1
    //      85 → 89 → 145 → 42 → 20 → 4 → 16 → 37 → 58 → 89
    //
    // Therefore any chain that arrives at 1 or 89 will become stuck in an endless loop. What is most amazing is that
    // EVERY starting number will eventually arrive at 1 or 89.
    //
    // How many starting numbers below ten million will arrive at 89?
    let mut result = 0;
    for n in 1..10000000 {
        let mut m = n;
        while m != 89 && m != 1 {
            m = square_digits(m);
        }
        if m == 89 {
            result += 1;
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem092() {
        let result = problem092();
        assert_eq!(result, "8581146");
    }
}
