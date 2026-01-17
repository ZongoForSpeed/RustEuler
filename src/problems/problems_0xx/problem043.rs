use crate::maths::digits::conversion;
use crate::register_problem;
use crate::utils::permutations::permutations;

register_problem!(43, "Sub-string divisibility", problem043);

pub fn problem043() -> String {
    // The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the
    // digits 0 to 9 in some order,
    // but it also has a rather interesting sub-string divisibility property.
    //
    // Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way, we note the following:
    //
    //    d2d3d4=406 is divisible by 2
    //    d3d4d5=063 is divisible by 3
    //    d4d5d6=635 is divisible by 5
    //    d5d6d7=357 is divisible by 7
    //    d6d7d8=572 is divisible by 11
    //    d7d8d9=728 is divisible by 13
    //    d8d9d10=289 is divisible by 17
    //
    // Find the sum of all 0 to 9 pandigital numbers with this property.
    let pandigital = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut result: usize = 0;
    for permute in permutations(pandigital) {
        let conversion = conversion(&permute, 10);
        if permute[3] % 2 == 0
            && (permute[2] + permute[3] + permute[4]) % 3 == 0
            && permute[5] % 5 == 0
            && ((conversion % 1000000) / 1000) % 7 == 0
            && ((conversion % 100000) / 100) % 11 == 0
            && ((conversion % 10000) / 10) % 13 == 0
            && (conversion % 1000) % 17 == 0
        {
            result += conversion;
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem043() {
        let result = problem043();
        assert_eq!(result, "16695334890");
    }
}
