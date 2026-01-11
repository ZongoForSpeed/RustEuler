use crate::register_problem;

register_problem!(217, "Balanced Numbers", problem217);

pub fn problem217() -> String {
    // A positive integer with k (decimal) digits is called balanced if its first [k/2] digits sum
    // to the same value as its last [k/2] digits, where ⌈x⌉, pronounced ceiling of x, is the smallest
    // integer ≥ x, thus [π] = 4 and [5] = 5.
    //
    // So, for example, all palindromes are balanced, as is 13722.
    //
    // Let T(n) be the sum of all balanced numbers less than 10^n.
    // Thus: T(1) = 45, T(2) = 540 and T(5) = 334795890.
    //
    // Find T(47) mod 3^15
    let mask = u128::pow(3, 15);
    let limit = 47;

    let mut result = 0;
    let max = limit + 3;
    let mut sum = vec![vec![0; max * 10]; limit + 1];
    let mut count = vec![vec![0; max * 10]; limit + 1];
    let mid = 5 * max;
    for n in 1..=limit {
        for i in 0..=n {
            let w: i8 = if i <= n / 2 {
                -1
            } else if i == n / 2 + (n % 2) {
                0
            } else {
                1
            };
            for j in 10..2 * mid - 10 {
                if i == 0 {
                    sum[i][j] = 0;
                    count[i][j] = if j == mid { 1 } else { 0 };
                } else {
                    sum[i][j] = 0;
                    count[i][j] = 0;
                    for d in 0..10 {
                        if d == 0 && i == 1 {
                            continue;
                        }
                        let old_index = match w {
                            -1 => j + d,
                            1 => j - d,
                            _ => j,
                        };
                        count[i][j] = (count[i][j] + count[i - 1][old_index]) % mask;
                        sum[i][j] = (sum[i][j]
                            + 10 * sum[i - 1][old_index]
                            + count[i - 1][old_index] * d as u128)
                            % mask;
                    }
                }
            }
        }
        result = (result + sum[n][mid]) % mask;
    }
    result %= mask;
    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem217::problem217;

    #[test]
    fn test_problem217() {
        let result = problem217();
        assert_eq!(result, "6273134");
    }
}
