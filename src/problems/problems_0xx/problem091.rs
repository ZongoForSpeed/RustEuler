use crate::register_problem;

register_problem!(91, "Right triangles with integer coordinates", problem091);

type Pair = (i32, i32);

pub fn problem091() -> String {
    // The points P (x1, y1) and Q (x2, y2) are plotted at integer co-ordinates and are joined to the origin, O(0,0), to
    // form ΔOPQ.
    //
    // There are exactly fourteen triangles containing a right angle that can be formed when each co-ordinate lies
    // between 0 and 2 inclusive; that is, 0 ≤ x1, y1, x2, y2 ≤ 2.
    //
    // Given that 0 ≤ x1, y1, x2, y2 ≤ 50, how many right triangles can be formed?
    let limit = 50;
    let mut points: Vec<Pair> = Vec::new();
    for i in 0..=limit {
        for j in 0..=limit {
            if i != 0 || j != 0 {
                points.push((i, j));
            }
        }
    }

    let right_triangle = |p1: &Pair, p2: &Pair| {
        let first = p1.0 - p2.0;
        let second = p1.1 - p2.1;
        return (first * p1.0 + second * p1.1 == 0)
            || (first * p2.0 + second * p2.1 == 0)
            || (p1.0 * p2.0 + p1.1 * p2.1 == 0);
    };

    let mut result:usize = 0;
    let len = points.len();
    for i in 0..len {
        let p1 = points[i];
        for j in i + 1..len {
            let p2 = points[j];
            if right_triangle(&p1, &p2) {
                result += 1;
            }
        }
    }

    result.to_string()
}
