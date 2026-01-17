use crate::register_problem;
use bit_set::BitSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

register_problem!(96, "Su Doku", problem096);

type Matrix = [[u16; 9]; 9];

fn options(s: &Matrix, i: usize, j: usize) -> Vec<u16> {
    let mut test: BitSet = BitSet::from_iter(1..10);
    let group_i = i - i % 3;
    let group_j = j - j % 3;
    for k in 0..9 {
        test.remove(s[i][k] as usize);
        test.remove(s[k][j] as usize);
        test.remove(s[group_i + k % 3][group_j + k / 3] as usize);
    }

    test.iter().map(|v| v as u16).collect()
}

fn sodoku(s: &Matrix) -> Option<Matrix> {
    let mut minimum = 10;
    let mut i_min = 10;
    let mut j_min = 10;
    let mut option = Vec::new();

    for i in 0..9 {
        for j in 0..9 {
            if s[i][j] == 0 {
                let o = options(&s, i, j);
                if o.is_empty() {
                    return None;
                }

                if o.len() < minimum {
                    minimum = o.len();
                    i_min = i;
                    j_min = j;
                    option = o;
                }
            }
        }
    }

    let mut t = s.clone();
    if option.is_empty() {
        return Some(t);
    }

    for o in option {
        t[i_min][j_min] = o;
        if let Some(x) = sodoku(&t) {
            return Some(x);
        }
    }

    None
}

pub fn problem096() -> String {
    // Su Doku (Japanese meaning number place) is the name given to a popular puzzle concept. Its origin is unclear, but
    // credit must be attributed to Leonhard Euler who invented a similar, and much more difficult, puzzle idea called
    // Latin Squares. The objective of Su Doku puzzles, however, is to replace the blanks (or zeros) in a 9 by 9 grid in
    // such that each row, column, and 3 by 3 box contains each of the digits 1 to 9. Below is an example of a typical
    // starting puzzle grid and its solution grid.
    //
    // matrice exemple = {
    //     {0, 0, 3, 0, 2, 0, 6, 0, 0},
    //     {9, 0, 0, 3, 0, 5, 0, 0, 1},
    //     {0, 0, 1, 8, 0, 6, 4, 0, 0},
    //     {0, 0, 8, 1, 0, 2, 9, 0, 0},
    //     {7, 0, 0, 0, 0, 0, 0, 0, 8},
    //     {0, 0, 6, 7, 0, 8, 2, 0, 0},
    //     {0, 0, 2, 6, 0, 9, 5, 0, 0},
    //     {8, 0, 0, 2, 0, 3, 0, 0, 9},
    //     {0, 0, 5, 0, 1, 0, 3, 0, 0}
    // };
    //
    // A well constructed Su Doku puzzle has a unique solution and can be solved by logic, although it may be necessary
    // to employ "guess and test" methods in order to eliminate options (there is much contested opinion over this). The
    // complexity of the search determines the difficulty of the puzzle; the example above is considered easy because it
    // can be solved by straight forward direct deduction.
    //
    // The 6K text file, sudoku.txt (right click and 'Save Link/Target As...'), contains fifty different Su Doku puzzles
    // ranging in difficulty, but all with unique solutions (the first puzzle in the file is the example above).
    //
    // By solving all fifty puzzles find the sum of the 3-digit numbers found in the top left corner of each solution
    // grid; for example, 483 is the 3-digit number found in the top left corner of the solution grid above.
    let file = File::open("data/p096_sudoku.txt");
    let reader = BufReader::new(file.ok().unwrap());
    let mut lines = reader.lines().peekable();

    let mut sodokus: Vec<Matrix> = Vec::new();
    while let Some(line) = lines.next() {
        let header = line.unwrap();
        if header.starts_with("Grid") {
            let mut grid = [[0u16; 9]; 9];

            for row in 0..9 {
                let row_line = lines.next().unwrap().ok().unwrap();

                if row_line.len() != 9 {
                    panic!("Unexpected row length {}", row_line.len());
                }

                for (col, ch) in row_line.chars().enumerate() {
                    grid[row][col] = ch.to_digit(10).unwrap() as u16;
                }
            }

            sodokus.push(grid);
        }
    }

    // println!("{:?}", sodokus);
    let mut result = 0;
    for s in sodokus {
        // println!("{:?}", s);
        if let Some(x) = sodoku(&s) {
            println!("{:?}", x);
            result += 100 * x[0][0] + 10 * x[0][1] + x[0][2];
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem096() {
        let result = problem096();
        assert_eq!(result, "24702");
    }
}
