use crate::register_problem;
use std::collections::BTreeMap;
use std::ops::AddAssign;

register_problem!(167, "Investigating Ulam sequences", problem167);

pub fn problem167() -> String {
    // For two positive integers a and b, the Ulam sequence U(a,b) is defined by U(a,b)1 = a, U(a,b)2 = b and for k > 2,
    // U(a,b)k is the smallest integer greater than U(a,b)(k-1) which can be written in exactly one way as the sum of
    // two distinct previous members of U(a,b).
    //
    // For example, the sequence U(1,2) begins with 1, 2, 3 = 1 + 2, 4 = 1 + 3, 6 = 2 + 4, 8 = 2 + 6, 11 = 3 + 8;
    // 5 does not belong to it because 5 = 1 + 4 = 2 + 3 has two representations as the sum of two previous members,
    // likewise 7 = 1 + 6 = 3 + 4.
    //
    // Find ∑U(2,2n+1)k for 2 ≤ n ≤10, where k = 1011.
    let i = 100000000000;
    let mut result = 0;
    for n in 2..=10 {
        let mut x = 0;
        let mut y = 0;

        let mut m: BTreeMap<u64, u64> = BTreeMap::new();
        let mut u_lam = vec![2 * n + 1];

        loop {
            let last_u = u_lam[u_lam.len() - 1];
            m.entry(last_u + 2).or_insert(0).add_assign(1);
            if x != 0 {
                m.entry(last_u + x).or_insert(0).add_assign(1);
            } else {
                for j in 0..u_lam.len() - 1 {
                    m.entry(last_u + u_lam[j]).or_insert(0).add_assign(1);
                }
            }

            if last_u % 2 == 0 {
                x = last_u;
                y = u_lam.len() - 1;
                for (p, e) in m.iter_mut() {
                    if p % 2 == 0 {
                        e.add_assign(1);
                    }
                }
            }

            let (key, _) = m.iter().find(|(_, v)| **v <= 1).unwrap();
            u_lam.push(*key);

            let to_delete: Vec<_> = m.range(..=*key).map(|(&k, _)| k).collect();
            for k in to_delete {
                m.remove(&k);
            }

            if u_lam[u_lam.len() - 1] - u_lam[u_lam.len() - 2] == x {
                break;
            }
        }

        let period = u_lam.len() - 2;
        let diff = u_lam[u_lam.len() - 1] - u_lam[0];

        let mut s = (i - 3) % period;
        if s >= y {
            s += 1;
        }

        let u = ((i - 3) / period) as u64 * diff + u_lam[s];
        // std::cout << "  U(2, " << U[0] << ")_" << i
        //     << " = U(2, " << U[0] << ")_" << s + 2 << " + "
        //     << diff << " * " << (i - 3) / periode << " = " << u << std::endl;

        result += u;
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem167() {
        let result = problem167();
        assert_eq!(result, "3916160068885");
    }
}
