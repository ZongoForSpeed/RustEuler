use crate::maths::polygonal::Polygonal;
use crate::register_problem;

register_problem!(142, "Perfect Square Collection", problem142);

pub fn problem142() -> String {
    // Find the smallest x + y + z with integers x > y > z > 0 such that x + y, x − y, x + z, x − z, y + z, y − z
    // are all perfect squares.
    let limit = 7000;
    let mut result = u64::MAX;

    for a in 1..limit {
        let aa = a * a;
        let parity = if a % 2 == 0 { 2 } else { 1 };

        for c in (parity..a).step_by(2) {
            let cc = c * c;
            let n = aa - cc;
            if !n.is_square() {
                continue;
            }

            for b in (parity..c).step_by(2) {
                let bb = b * b;
                if 2 * cc < aa + bb {
                    continue;
                }

                let n = aa + bb - cc;
                let n1 = cc - bb;
                if !n1.is_square() || !n.is_square() {
                    continue;
                }
                let x = (aa + bb) / 2;
                let y = (aa - bb) / 2;
                let z = cc - x;
                result = std::cmp::min(result, x + y + z);
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem142() {
        let result = problem142();
        assert_eq!(result, "1006193");
    }
}
