use crate::maths::pythagorean::pythagorean_limit;
use crate::register_problem;
use std::collections::BTreeMap;

register_problem!(309, "Integer Ladders", problem309);

/// In the classic "Crossing Ladders" problem, we are given the lengths x and y of two ladders resting on the
/// opposite walls of a narrow, level street. We are also given the height h above the street where the two ladders
/// cross and we are asked to find the width of the street (w).
///
/// Here, we are only concerned with instances where all four variables are positive integers.
/// For example, if x = 70, y = 119 and h = 30, we can calculate that w = 56.
///
/// In fact, for integer values x, y, h and 0 < x < y < 200, there are only five triplets (x,y,h) producing integer
/// solutions for w: (70, 119, 30), (74, 182, 21), (87, 105, 35), (100, 116, 35) and (119, 175, 40).
///
/// For integer values x, y, h and 0 < x < y < 1 000 000, how many triplets (x,y,h) produce integer solutions for w?
pub fn problem309() -> String {
    let limite: u64 = 1_000_000;

    // https://en.wikipedia.org/wiki/Crossed_ladders_problem
    let mut dictionnaire: BTreeMap<u64, Vec<(u64, u64)>> = BTreeMap::new();
    for (x, y, z) in pythagorean_limit(limite) {
        for k in 1.. {
            if z * k >= limite {
                break;
            }
            dictionnaire.entry(k * x).or_default().push((k * y, k * z));
            dictionnaire.entry(k * y).or_default().push((k * x, k * z));
        }
    }

    let mut resultat: Vec<(u64, u64, u64)> = Vec::new();
    for v in dictionnaire.values() {
        for (i, &(a1, b1)) in v.iter().enumerate() {
            for &(a2, b2) in &v[i + 1..] {
                // 1 / h = 1 / a1 + 1 / a2
                let n1 = a1 * a2;
                let n2 = a1 + a2;
                if n1 % n2 == 0 {
                    let h = n1 / n2;
                    resultat.push((b1.min(b2), b1.max(b2), h));
                }
            }
        }
    }

    resultat.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem309() {
        assert_eq!(problem309(), "210139");
    }
}
