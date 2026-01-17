use crate::register_problem;
use itertools::iproduct;

register_problem!(178, "Step Numbers", problem178);

pub fn problem178() -> String {
    // Consider the number 45656.
    // It can be seen that each pair of consecutive digits of 45656 has a difference of one.
    // A number for which every pair of consecutive digits has a difference of one is called a step number.
    // A pandigital number contains every decimal digit from 0 to 9 at least once.
    // How many pandigital step numbers less than 1040 are there?
    let mut f = vec![vec![vec![vec![0u64; 10]; 10]; 10]; 41];
    for i in 0..10 {
        f[1][i][i][i] = 1;
    }

    for (m, x) in iproduct!(2..=40, 0..10) {
        for y in x + 1..10 {
            f[m][x][y][x] = f[m - 1][x][y][x + 1] + f[m - 1][x + 1][y][x + 1];
            for z in x + 1..y {
                f[m][x][y][z] = f[m - 1][x][y][z - 1] + f[m - 1][x][y][z + 1];
            }
            f[m][x][y][y] = f[m - 1][x][y][y - 1] + f[m - 1][x][y - 1][y - 1];
        }
    }

    let mut result = 0;
    for (m, z) in iproduct!(1..=40, 1..10) {
        result += f[m][0][9][z];
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem178() {
        let result = problem178();
        assert_eq!(result, "126461847755");
    }
}
