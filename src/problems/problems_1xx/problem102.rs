use crate::register_problem;
use itertools::Itertools;
use std::path::Path;

type Pair = (i32, i32);
register_problem!(102, "Triangle containment", problem102);

fn area_time_2((xa, ya): &Pair, (xb, yb): &Pair, (xc, yc): &Pair) -> i32 {
    ((xb - xa) * (yc - ya) - (xc - xa) * (yb - ya)).abs()
}

fn contains(a: &Pair, b: &Pair, c: &Pair) -> bool {
    const O: Pair = (0, 0);
    let abc = area_time_2(a, b, c);
    let obc = area_time_2(&O, b, c);
    let aoc = area_time_2(a, &O, c);
    let abo = area_time_2(a, b, &O);
    // println!("abc: {:?}, obc: {:?}, aoc: {:?}, abo: {:?}", abc, obc, aoc, abo);
    abc == obc + aoc + abo
}

pub fn problem102() -> String {
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
    let path = Path::new("data/p102_triangles.txt");
    let mut result: u16 = 0;
    for line in std::fs::read_to_string(path).unwrap().lines() {
        let (xa, ya, xb, yb, xc, yc) = line
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .next_tuple()
            .unwrap();
        // println!("{xa}, {ya}, {xb}, {yb}, {xc}, {yc}");
        let a = (xa, ya);
        let b = (xb, yb);
        let c = (xc, yc);
        if contains(&a, &b, &c) {
            result += 1;
        }
    }

    result.to_string()
}
