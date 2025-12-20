use crate::problems::problem018::maximum_path_sum;
use crate::register_problem;
use std::path::Path;

register_problem!(67, "Maximum path sum II", problem067);

pub fn problem067() -> String {
    // By starting at the top of the triangle below and moving to adjacent numbers on the row below,
    // the maximum total from top to bottom is 23.
    //
    //                                             3
    //                                            7 4
    //                                           2 4 6
    //                                          8 5 9 3
    //
    // That is, 3 + 7 + 4 + 9 = 23.
    //
    // Find the maximum total from top to bottom in triangle.txt (right click and 'Save Link/Target As...'),
    // a 15K text file containing a triangle with one-hundred rows.
    //
    // NOTE: This is a much more difficult version of Problem 18. It is not possible to try every route
    // to solve this problem, as there are 299 altogether! If you could check one trillion (1012) routes
    // every second it would take over twenty billion years to check them all. There is an efficient
    // algorithm to solve it. ;o)
    let path = Path::new("data/p067_triangle.txt");
    let triangle:Vec<Vec<u16>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .map(|s| s.split_whitespace().map(str::parse).map(Result::unwrap).collect()).collect();

    println!("{:?}", triangle);
    maximum_path_sum(triangle).to_string()
}
