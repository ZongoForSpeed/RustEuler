use crate::maths::pythagorean::pythagorean_limit;
use crate::register_problem;

register_problem!(39, "Integer right triangles", problem039);

pub fn problem039() -> String {
    // If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there
    // are exactly three solutions for p = 120.
    //
    // {20,48,52}, {24,45,51}, {30,40,50}
    //
    // For which value of p ≤ 1000, is the number of solutions maximised?
    let limit = 1000;
    let mut solutions = vec![0; limit + 1];

    for (a,b,c) in pythagorean_limit(limit) {
        let s = a +b + c;
        for k in (s..=limit).step_by(s) {
            solutions[k] += 1;
        }
    }

    let index_of_max: Option<usize> = solutions
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(index, _)| index);
    index_of_max.unwrap().to_string()
}
