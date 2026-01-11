use crate::register_problem;
use crate::utils::permutations::permutations;
use std::collections::HashMap;
use std::ops::AddAssign;

register_problem!(215, "Crack-free Walls", problem215);

pub fn problem215() -> String {
    // Consider the problem of building a wall out of 2×1 and 3×1 bricks (horizontal×vertical dimensions) such that, for
    // extra strength, the gaps between horizontally-adjacent bricks never line up in consecutive layers, i.e. never
    // form a "running crack".
    //
    // For example, the following 9×3 wall is not acceptable due to the running crack shown in red:
    //
    // There are eight ways of forming a crack-free 9×3 wall, written W(9,3) = 8.
    //
    // Calculate W(32,10).
    let size = 32;
    let height = 10;

    let mut walls = Vec::new();
    for n in 0..=size / 3 {
        if (size - 3 * n) % 2 == 0 {
            let mut wall = vec![2; (size - 3 * n) / 2];
            wall.resize(wall.len() + n, 3);
            for permute in permutations(wall) {
                let mut bit_set: u64 = 0;
                let mut w = 0;
                for m in permute.iter() {
                    // add l to bit_set
                    bit_set |= 1 << w;
                    w += m;
                }
                // remove 0 from bit_set
                bit_set = (bit_set | 1) - 1;
                walls.push(bit_set);
            }
        }
    }

    let mut disjoints = HashMap::new();
    for m1 in &walls {
        for m2 in &walls {
            if m1 & m2 == 0 {
                // m1.is_disjoint(m2)
                disjoints
                    .entry(m1.clone())
                    .or_insert_with(Vec::new)
                    .push(m2.clone());
            }
        }
    }

    let mut dp = HashMap::new();
    for (&k, v) in &disjoints {
        dp.insert(k, v.len());
    }

    for _ in 2..height {
        let mut next_dp = HashMap::new();
        for (k, &v) in &dp {
            if let Some(values) = disjoints.get(k) {
                for &i in values {
                    next_dp.entry(i).or_insert(0).add_assign(v);
                }
            }
        }

        dp = next_dp;
    }

    let result = dp.values().into_iter().sum::<usize>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem215::problem215;

    #[test]
    fn test_problem215() {
        let result = problem215();
        assert_eq!(result, "806844323190414");
    }
}
