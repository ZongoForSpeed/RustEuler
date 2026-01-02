use crate::maths::graph::{Edge, kruskal};
use crate::register_problem;
use std::path::Path;

fn minimal_network(file_path: &str) -> u64 {
    let path = Path::new(file_path);
    let mut graph: Vec<Edge> = Vec::new();
    for (i, line) in std::fs::read_to_string(path).unwrap().lines().enumerate() {
        for (j, s) in line.split(',').enumerate() {
            if let Ok(v) = s.parse()
                && i < j
            {
                graph.push((i, j, v));
            }
        }
    }

    // println!("{:?}", graph);
    let cost_graph = graph.iter().map(|e| e.2).sum::<u64>();
    let min_tree = kruskal(&graph);

    let min_cost = min_tree.iter().map(|g| g.2).sum::<u64>();
    println!("File: {file_path} ==> Current Cost {cost_graph}, Minimal Cost {min_cost}");
    let result = cost_graph - min_cost;
    result
}

register_problem!(107, "Triangle containment", problem107);

pub fn problem107() -> String {
    // Three distinct points are plotted at random on a Cartesian plane, for which -1000 ≤ x, y ≤ 1000, such that a
    // triangle is formed.
    //
    // Consider the following two triangles:
    //
    //                                  A(-340,495), B(-153,-910), C(835,-947)
    //
    //                                  X(-175,41), Y(-421,-714), Z(574,-645)
    //
    // It can be verified that triangle ABC contains the origin, whereas triangle XYZ does not.
    //
    // Using triangles.txt (right click and 'Save Link/Target As...'), a 27K text file containing the co-ordinates of
    // one thousand "random" triangles, find the number of triangles for which the interior contains the origin.
    //
    // NOTE: The first two examples in the file represent the triangles in the example given above.
    minimal_network("data/p107_example.txt");

    minimal_network("data/p107_network.txt").to_string()
}
