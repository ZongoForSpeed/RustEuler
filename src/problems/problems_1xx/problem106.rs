use crate::register_problem;
use itertools::Itertools;
use crate::maths::digits::Digits;

register_problem!(106, "Special subset sums: meta-testing", problem106);

pub fn problem106() -> String {
    // Let S(A) represent the sum of elements in set A of size n. We shall call it a special sum set if for any two
    // non-empty disjoint subsets, B and C, the following properties are true:
    //
    //      S(B) ≠ S(C); that is, sums of subsets cannot be equal.
    //      If B contains more elements than C then S(B) > S(C).
    //
    // For this problem we shall assume that a given set contains n strictly increasing elements and it already
    // satisfies the second rule.
    //
    // Surprisingly, out of the 25 possible subset pairs that can be obtained from a set for which n = 4, only 1 of
    // these pairs need to be tested for equality (first rule). Similarly, when n = 7, only 70 out of the 966 subset
    // pairs need to be tested.
    //
    // For n = 12, how many of the 261625 subset pairs that can be obtained need to be tested for equality?
    //
    // NOTE: This problem is related to Problem 103 and Problem 105.
    let size: usize = 12;
    let max_mask: usize = 1 << size;
    let mut result = 0;

    for mask_a in 1..max_mask {
        if mask_a.count_ones() == 3 {
            let (a1, a2, a3) = mask_a.ones().into_iter().next_tuple().unwrap();
            let step = 1 << a1;
            for mask_b in (step..max_mask).step_by(step) {
                if mask_a & mask_b == 0 && mask_b.count_ones() == 3 {
                    let (b1, b2, b3) = mask_b.ones().into_iter().next_tuple().unwrap();
                    if b1 < a3 && (b2 < a2 || b3 < a3) {
                        result += 1;
                    }
                }
            }
        }
        if mask_a.count_ones() == 4 {
            result += 1;
            let (a1, a2, a3, a4) = mask_a.ones().into_iter().next_tuple().unwrap();
            let step = 1 << a1;
            for mask_b in (step..max_mask).step_by(step) {
                if mask_a & mask_b == 0 && mask_b.count_ones() == 4 {
                    let (b1, b2, b3, b4) = mask_b.ones().into_iter().next_tuple().unwrap();
                    if (b1 < a4) && (a2 > b2 || a3 > b3 || a4 > b4) {
                        result += 1;
                    }
                }
            }
        }
        if mask_a.count_ones() == 5 {
            let (a1, a2, a3, a4, a5) = mask_a.ones().into_iter().next_tuple().unwrap();
            let step = 1 << a1;
            for mask_b in (step..max_mask).step_by(step) {
                if mask_a & mask_b == 0 && mask_b.count_ones() == 5 {
                    let (b1, b2, b3, b4, b5) = mask_b.ones().into_iter().next_tuple().unwrap();
                    if (b1 < a5) && (a2 > b2 || a3 > b3 || a4 > b4 || a5 > b5) {
                        result += 1;
                    }
                }
            }
        }
        if mask_a.count_ones() == 6 {
            let (a1, a2, a3, a4, a5, a6) = mask_a.ones().into_iter().next_tuple().unwrap();
            let step = 1 << a1;
            for mask_b in (step..max_mask).step_by(step) {
                if mask_a & mask_b == 0 && mask_b.count_ones() == 6 {
                    let (b1, b2, b3, b4, b5, b6) = mask_b.ones().into_iter().next_tuple().unwrap();
                    if (b1 < a6) && (a2 > b2 || a3 > b3 || a4 > b4 || a5 > b5 || a6 > b6) {
                        result += 1;
                    }
                }
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem106() {
        let result = problem106();
        assert_eq!(result, "21384");
    }
}
