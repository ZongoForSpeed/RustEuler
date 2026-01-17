use crate::register_problem;
use std::path::Path;

register_problem!(82, "Path sum: three ways", problem082);

pub fn problem082() -> String {
    // The minimal path sum in the 5 by 5 matrix below, by starting in any cell in the left column and finishing in any
    // cell in the right column, and only moving up, down, and right, is indicated in red and bold; the sum is equal to
    // 994.
    //
    // Find the minimal path sum, in matrix.txt (right click and "Save Link/Target As..."), a 31K text file containing
    // a 80 by 80 matrix, from the left column to the right column.
    let path = Path::new("data/p082_matrix.txt");
    let graph: Vec<Vec<u32>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.split(',').map(str::parse).map(Result::unwrap).collect())
        .collect();

    let length = graph.len();
    let mut paths = vec![vec![0; length]; length];
    for i in 0..length {
        let mut lower_path = vec![0; length];
        for j in 0..length {
            if i == 0 {
                lower_path[j] = graph[j][i];
            } else if j == 0 {
                lower_path[j] = paths[j][i - 1] + graph[j][i];
            } else {
                lower_path[j] = std::cmp::min(paths[j][i - 1], lower_path[j - 1]) + graph[j][i];
            }
        }

        let mut high_path = vec![0; length];
        for jj in 0..length {
            let j = length - jj - 1;
            if i == 0 {
                high_path[j] = graph[j][i];
            } else if j == length - 1 {
                high_path[j] = paths[j][i - 1] + graph[j][i];
            } else {
                high_path[j] = std::cmp::min(paths[j][i - 1], high_path[j + 1]) + graph[j][i];
            }
        }

        for j in 0..length {
            paths[j][i] = std::cmp::min(high_path[j], lower_path[j]);
        }
    }

    paths
        .into_iter()
        .map(|l| l.last().unwrap().clone())
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem082() {
        let result = problem082();
        assert_eq!(result, "260324");
    }
}
