use crate::maths::graph::a_star;
use crate::register_problem;
use std::collections::HashMap;
use std::path::Path;

register_problem!(83, "Path sum: four ways", problem083);

fn distance(i1: &usize, j1: &usize, m: &Vec<Vec<u32>>) -> u32 {
    m[*i1][*j1]
}

type Node = (usize, usize);

pub fn problem083() -> String {
    // The minimal path sum in the 5 by 5 matrix below, by starting in any cell in the left column and finishing in any
    // cell in the right column, and only moving up, down, and right, is indicated in red and bold; the sum is equal to
    // 994.
    //
    // Find the minimal path sum, in matrix.txt (right click and "Save Link/Target As..."), a 31K text file containing
    // a 80 by 80 matrix, from the left column to the right column.
    let path = Path::new("data/p083_matrix.txt");
    let m: Vec<Vec<u32>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.split(',').map(str::parse).map(Result::unwrap).collect())
        .collect();

    // let m = vec![
    //    vec![131, 673, 234, 103, 18],
    //    vec![201, 96, 342, 965, 150],
    //    vec![630, 803, 746, 422, 111],
    //    vec![537, 699, 497, 121, 956],
    //    vec![805, 732, 524, 37, 331],
    // ];

    let mut graph: HashMap<Node, Vec<Node>> = HashMap::new();
    let length = m.len();
    for i in 0..length {
        for j in 0..length {
            let mut v: Vec<Node> = Vec::new();
            if i > 0 {
                v.push((i - 1, j));
            }
            if j > 0 {
                v.push((i, j - 1));
            }
            if i < length - 1 {
                v.push(((i + 1), j));
            }
            if j < length - 1 {
                v.push((i, j + 1));
            }
            graph.insert((i, j), v);
        }
    }

    let g = |v: &Node| match graph.get(v) {
        None => vec![],
        Some(v) => v.clone(),
    };

    let d = |(i, j): &Node, _: &Node| distance(i, j, &m);

    let h = |_: &Node, _: &Node| 0;


    let result: u32 = a_star((0, 0), (length - 1, length - 1), g, d, h) + m[length - 1][length - 1];
    result.to_string()
}
