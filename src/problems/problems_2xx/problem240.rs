use crate::maths::factorial::Factorial;
use crate::register_problem;

register_problem!(240, "Top Dice", problem240);

fn sum(arrangement: &Vec<u32>) -> u32 {
    let mut result = 0;
    for (i, &value) in arrangement.iter().enumerate() {
        result += i as u32 * value;
    }
    result
}

fn combination(arrangement: &Vec<u32>) -> u128 {
    let mut result = 1;
    for &value in arrangement {
        result *= u128::factorial(value as u128);
    }
    result
}

fn w(n: usize, mut arrangement: &mut Vec<u32>, m: usize) -> u128 {
    if n == 0 {
        return u128::factorial(20);
    }
    let mut result = 0;
    for k in 1..=m {
        arrangement[k] += 1;
        result += w(n - 1, &mut arrangement, k) / (arrangement[k] as u128);
        arrangement[k] -= 1;
    }
    result
}

fn a(n: usize, mut arrangement: &mut Vec<u32>, m: usize) -> u128 {
    if n == 0 {
        if sum(&arrangement) == 70 {
            return w(10, arrangement, m) / combination(&arrangement);
        }

        return 0;
    }

    let mut result = 0;
    for k in 1..=m {
        arrangement[k] += 1;
        result += a(n - 1, &mut arrangement, k);
        arrangement[k] -= 1;
    }
    result
}

pub fn problem240() -> String {
    // There are 1111 ways in which five 6-sided dice (sides numbered 1 to 6) can be rolled so that
    // the top three sum to 15. Some examples are:
    //
    //  D1,D2,D3,D4,D5 = 4,3,6,3,5
    //  D1,D2,D3,D4,D5 = 4,3,3,5,6
    //  D1,D2,D3,D4,D5 = 3,3,3,6,6
    //  D1,D2,D3,D4,D5 = 6,6,3,3,3
    //
    // In how many ways can twenty 12-sided dice (sides numbered 1 to 12) be rolled so that the top
    // ten sum to 70?
    let mut arrangement: Vec<u32> = vec![0; 13];

    let result = a(10, &mut arrangement, 12);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem240::problem240;

    #[test]
    fn test_problem240() {
        let result = problem240();
        assert_eq!(result, "7448717393364181966");
    }
}
