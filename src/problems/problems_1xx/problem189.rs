use crate::register_problem;
use itertools::iproduct;
use std::collections::HashMap;
use std::ops::AddAssign;

fn test(c1: &Vec<u8>, c2: &Vec<u8>) -> bool {
    if c1.len() != c2.len() {
        return false;
    }

    for n in 0..c1.len() {
        if c1[n] == c2[n] {
            return false;
        }
    }
    true
}

register_problem!(189, "Tri-colouring a triangular grid", problem189);

pub fn problem189() -> String {
    // Consider the following configuration of 64 triangles:
    //
    // We wish to colour the interior of each triangle with one of three colours: red, green or blue,
    // so that no two neighbouring triangles have the same colour. Such a colouring shall be called
    // valid. Here, two triangles are said to be neighbouring if they share an edge.
    // Note: if they only share a vertex, then they are not neighbours.
    //
    // For example, here is a valid colouring of the above grid:
    //
    // A colouring C' which is obtained from a colouring C by rotation or reflection is considered
    // distinct from C unless the two are identical.
    //
    // How many distinct valid colourings are there for the above configuration?
    let mut triangles4: Vec<Vec<u8>> = Vec::new();
    let mut cotes: HashMap<Vec<u8>, u64> = HashMap::new();

    for (a1, a2, b1, b2) in iproduct!(0..3, 0..3, 0..3, 0..3) {
        if b1 != a1 && b1 != a2 && b2 != a2 {
            for (a3, a4, b3) in iproduct!(0..3, 0..3, 0..3) {
                if b2 != a3 && b3 != a3 && b3 != a4 {
                    for (c1, c2, c3) in iproduct!(0..3, 0..3, 0..3) {
                        if b1 != c1 && b2 != c2 && b3 != c3 {
                            for (d1, d2, e1, e2) in iproduct!(0..3, 0..3, 0..3, 0..3) {
                                if d1 != c1
                                    && d1 != c2
                                    && d2 != c2
                                    && d2 != c3
                                    && d1 != e1
                                    && d2 != e2
                                {
                                    for (f1, g1) in iproduct!(0..3, 0..3) {
                                        if f1 != e1 && f1 != e2 && g1 != f1 {
                                            let cote = vec![a1, a2, a3, a4];
                                            cotes.entry(cote).or_insert(0).add_assign(1);
                                            let triangle =
                                                vec![a1, a2, a3, a4, c3, e2, g1, e1, c1, a1];
                                            triangles4.push(triangle);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut result = 0;
    for t in triangles4 {
        let c1 = t[0..4].to_vec();
        let c2 = t[3..7].to_vec();
        let c3 = t[6..].to_vec();

        let mut value1 = 0;
        let mut value2 = 0;
        let mut value3 = 0;

        for (key, value) in &cotes {
            if test(&c1, &key) {
                value1 += value;
            }
            if test(&c2, &key) {
                value2 += value;
            }
            if test(&c3, &key) {
                value3 += value;
            }
        }
        result += value1 * value2 * value3;
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem189() {
        let result = problem189();
        assert_eq!(result, "10834893628237824");
    }
}
