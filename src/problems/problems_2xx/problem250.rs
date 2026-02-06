use crate::maths::power::Power;
use crate::register_problem;

register_problem!(250, "250250", problem250);

/// Find the number of non-empty subsets of {1^1, 2^2, 3^3,..., 250250^250250}, the sum of whose elements is
/// divisible by 250. Enter the rightmost 16 digits as your answer.
pub fn problem250() -> String {
    let mask = usize::pow(10, 16);

    let m = 250;
    let mut frequencies = vec![0; m];
    for i in 1..=250250 {
        frequencies[usize::power_mod(i, i, m)] += 1;
    }

    let mut dp = vec![0; m];
    dp[0] = 1;
    for i in 0..m {
        for _ in 0..frequencies[i] {
            let mut temp = vec![0; m];
            for k in 0..m {
                temp[k] = (dp[k] + dp[(m + k - i) % m]) % mask;
            }
            dp = temp;
        }
    }

    let result = dp[0] - 1;
    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem250::problem250;

    #[test]
    fn test_problem250() {
        let result = problem250();
        assert_eq!(result, "1425480602091519");
    }
}
