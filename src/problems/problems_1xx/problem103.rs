use crate::maths::digits::Digits;
use crate::register_problem;
use itertools::Itertools;

register_problem!(103, "Special subset sums: optimum", problem103);

pub fn verify(set: &Vec<u64>) -> bool {
    let len = set.len();
    if set[0] + set[1] <= set[len - 1] {
        return false;
    }

    if len > 4 && set[0] + set[1] + set[2] <= set[len - 1] + set[len - 2] {
        return false;
    }

    if len > 6 && set[0] + set[1] + set[2] + set[3] <= set[len - 1] + set[len - 2] + set[len - 3] {
        return false;
    }

    if len > 8
        && set[0] + set[1] + set[2] + set[3] + set[4]
            <= set[len - 1] + set[len - 2] + set[len - 3] + set[len - 4]
    {
        return false;
    }

    if len > 10
        && set[0] + set[1] + set[2] + set[3] + set[4] + set[5]
            <= set[len - 1] + set[len - 2] + set[len - 3] + set[len - 4] + set[len - 5]
    {
        return false;
    }

    let max_mask: usize = 1 << len;
    for mask_a in 1..max_mask {
        if mask_a.count_ones() == 2 {
            let (a1, a2) = mask_a.ones().into_iter().next_tuple().unwrap();
            let step = 2 << a1;
            for mask_b in (step..max_mask).step_by(step) {
                if mask_a & mask_b == 0 && mask_b.count_ones() == 2 {
                    let (b1, b2) = mask_b.ones().into_iter().next_tuple().unwrap();
                    if b1 < a2 && set[a1] + set[a2] == set[b1] + set[b2] {
                        return false;
                    }
                }
            }
        }
        if len > 5 && mask_a.count_ones() == 3 {
            let (a1, a2, a3) = mask_a.ones().into_iter().next_tuple().unwrap();
            let step = 2 << a1;
            for mask_b in (step..max_mask).step_by(step) {
                if mask_a & mask_b == 0 && mask_b.count_ones() == 3 {
                    let (b1, b2, b3) = mask_b.ones().into_iter().next_tuple().unwrap();
                    if b1 < a3 && set[a1] + set[a2] + set[a3] == set[b1] + set[b2] + set[b3] {
                        return false;
                    }
                }
            }
        }
        if len > 7 && mask_a.count_ones() == 4 {
            let (a1, a2, a3, a4) = mask_a.ones().into_iter().next_tuple().unwrap();
            let step = 2 << a1;
            for mask_b in (step..max_mask).step_by(step) {
                if mask_a & mask_b == 0 && mask_b.count_ones() == 4 {
                    let (b1, b2, b3, b4) = mask_b.ones().into_iter().next_tuple().unwrap();
                    if b1 < a4
                        && set[a1] + set[a2] + set[a3] + set[a4]
                            == set[b1] + set[b2] + set[b3] + set[b4]
                    {
                        return false;
                    }
                }
            }
        }
        if len > 9 && mask_a.count_ones() == 5 {
            let (a1, a2, a3, a4, a5) = mask_a.ones().into_iter().next_tuple().unwrap();
            let step = 2 << a1;
            for mask_b in (step..max_mask).step_by(step) {
                if mask_a & mask_b == 0 && mask_b.count_ones() == 5 {
                    let (b1, b2, b3, b4, b5) = mask_b.ones().into_iter().next_tuple().unwrap();
                    if b1 < a5
                        && set[a1] + set[a2] + set[a3] + set[a4] + set[a5]
                            == set[b1] + set[b2] + set[b3] + set[b4] + set[b5]
                    {
                        return false;
                    }
                }
            }
        }
        if len > 11 && mask_a.count_ones() == 6 {
            let (a1, a2, a3, a4, a5, a6) = mask_a.ones().into_iter().next_tuple().unwrap();
            let step = 2 << a1;
            for mask_b in (step..max_mask).step_by(step) {
                if mask_a & mask_b == 0 && mask_b.count_ones() == 6 {
                    let (b1, b2, b3, b4, b5, b6) = mask_b.ones().into_iter().next_tuple().unwrap();
                    if b1 < a6
                        && set[a1] + set[a2] + set[a3] + set[a4] + set[a5] + set[a6]
                            == set[b1] + set[b2] + set[b3] + set[b4] + set[b5] + set[b6]
                    {
                        return false;
                    }
                }
            }
        }
    }

    true
}

pub fn problem103() -> String {
    // Let S(A) represent the sum of elements in set A of size n. We shall call it a special sum set if for any two
    // non-empty disjoint subsets, B and C, the following properties are true:
    //
    //      S(B) ≠ S(C); that is, sums of subsets cannot be equal.
    //      If B contains more elements than C then S(B) > S(C).
    //
    // If S(A) is minimised for a given n, we shall call it an optimum special sum set. The first five optimum special
    // sum sets are given below.
    //
    // n = 1: {1}
    // n = 2: {1, 2}
    // n = 3: {2, 3, 4}
    // n = 4: {3, 5, 6, 7}
    // n = 5: {6, 9, 11, 12, 13}
    //
    // It seems that for a given optimum set, A = {a1, a2, ... , an}, the next optimum set is of the form
    // B = {b, a1+b, a2+b, ... ,an+b}, where b is the "middle" element on the previous row.
    //
    // By applying this "rule" we would expect the optimum set for n = 6 to be A = {11, 17, 20, 22, 23, 24}, with
    // S(A) = 117. However, this is not the optimum set, as we have merely applied an algorithm to provide a near
    // optimum set. The optimum set for n = 6 is A = {11, 18, 19, 20, 22, 25}, with S(A) = 115 and corresponding set
    // string: 111819202225.
    //
    // Given that A is an optimum special sum set for n = 7, find its set string.
    //
    // NOTE: This problem is related to Problem 105 and Problem 106.
    let mut minimum = u64::MAX;
    let mut solution: Vec<u64> = Vec::new();

    for a1 in 20..50 {
        for a2 in a1 + 1..50 {
            for a3 in a2 + 1..50 {
                for a4 in a3 + 1..50 {
                    for a5 in a4 + 1..50 {
                        for a6 in a5 + 1..50 {
                            for a7 in a6 + 1..50 {
                                let v = vec![a1, a2, a3, a4, a5, a6, a7];
                                let sum = v.iter().sum();
                                if sum < minimum && verify(&v) {
                                    println!("{v:?} => {sum}");
                                    minimum = sum;
                                    solution = v;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    solution.iter().map(|x| x.to_string()).join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem103() {
        let result = problem103();
        assert_eq!(result, "20313839404245");
    }
}
