use crate::maths::digits::Digits;
use crate::register_problem;
use num_integer::Roots;

register_problem!(206, "Concealed Square", problem206);

fn test(n: u128) -> bool {
    let digits = n.extract_digits(10);
    digits[0] == 1
        && digits[2] == 2
        && digits[4] == 3
        && digits[6] == 4
        && digits[8] == 5
        && digits[10] == 6
        && digits[12] == 7
        && digits[14] == 8
        && digits[16] == 9
        && digits[18] == 0
}

pub fn problem206() -> String {
    // Find the unique positive integer whose square has the form 1_2_3_4_5_6_7_8_9_0,
    // where each “_” is a single digit.
    let min = 102030405060708.sqrt();
    let max = 192939495969798.sqrt();

    for n in min..max {
        let square3 = (n * 100 + 30) * (n * 100 + 30);
        if test(square3) {
            let result = n * 100 + 30;
            return result.to_string();
        }

        let square7 = (n * 100 + 70) * (n * 100 + 70);
        if test(square7) {
            let result = n * 100 + 70;
            return result.to_string();
        }
    }

    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem206::problem206;

    #[test]
    fn test_problem206() {
        let result = problem206();
        assert_eq!(result, "1389019170");
    }

}
