use crate::register_problem;
use std::path::Path;

register_problem!(81, "Path sum: two ways", problem081);

pub fn problem081() -> String {
    // In the 5 by 5 matrix below, the minimal path sum from the top left to the bottom right, by
    // only moving to the right and down, is indicated in bold red and is equal to 2427.
    //
    //              131 673 234 103  18
    //              201  96 342 965 150
    //              630 803 746 422 111
    //              537 699 497 121 956
    //              805 732 524  37 331
    //
    // Find the minimal path sum, in matrix.txt (right click and "Save Link/Target As..."), a 31K
    // text file containing a 80 by 80 matrix, from the top left to the bottom right by only moving
    // right and down.
    let path = Path::new("data/p081_matrix.txt");
    let graph: Vec<Vec<u32>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.split(',').map(str::parse).map(Result::unwrap).collect())
        .collect();

    let length = graph.len();
    let mut paths = vec![vec![0; length]; length];

    for i in 0..length {
        for j in 0..length {
            if i == 0 && j == 0 {
                paths[i][j] = graph[i][j];
            } else if i == 0 {
                paths[i][j] = graph[i][j] + paths[i][j - 1];
            } else if j == 0 {
                paths[i][j] = graph[i][j] + paths[i - 1][j];
            } else {
                paths[i][j] = graph[i][j] + std::cmp::min(paths[i][j - 1], paths[i - 1][j]);
            }
        }
    }

    paths[length - 1][length - 1].to_string()
}
