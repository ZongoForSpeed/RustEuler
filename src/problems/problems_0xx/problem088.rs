use bit_set::BitSet;
use crate::register_problem;

register_problem!(88, "Product-sum numbers", problem088);

fn generate(best: &mut Vec<usize>, n: usize, length: usize, sum: usize, ratio: usize) {
    let new_length = length + 1 + n - sum - ratio;
    if new_length <= best.len() && n < best[new_length] {
        best[new_length] = n;
    }
    for i in 2..=ratio.isqrt() {
        if ratio % i == 0 {
            generate(best, n, length + 1, sum + i, ratio / i);
        }
    }
}

pub fn problem088() -> String {
    // A natural number, N, that can be written as the sum and product of a given set of at least
    // two natural numbers, {a1, a2, ... , ak} is called a product-sum number:
    // N = a1 + a2 + ... + ak = a1 × a2 × ... × ak.
    //
    // For example, 6 = 1 + 2 + 3 = 1 × 2 × 3.
    //
    // For a given set of size, k, we shall call the smallest N with this property a minimal
    // product-sum number. The minimal product-sum numbers for sets of size, k = 2, 3, 4, 5,
    // and 6 are as follows.
    //
    //      k=2: 4 = 2 × 2 = 2 + 2
    //      k=3: 6 = 1 × 2 × 3 = 1 + 2 + 3
    //      k=4: 8 = 1 × 1 × 2 × 4 = 1 + 1 + 2 + 4
    //      k=5: 8 = 1 × 1 × 2 × 2 × 2 = 1 + 1 + 2 + 2 + 2
    //      k=6: 12 = 1 × 1 × 1 × 1 × 2 × 6 = 1 + 1 + 1 + 1 + 2 + 6
    //
    // Hence for 2≤k≤6, the sum of all the minimal product-sum numbers is 4+6+8+12 = 30; note that
    // 8 is only counted once in the sum.
    //
    // In fact, as the complete set of minimal product-sum numbers for 2≤k≤12 is
    // {4, 6, 8, 12, 15, 16}, the sum is 61.
    //
    // What is the sum of all the minimal product-sum numbers for 2≤k≤12000?
    let limit:usize = 12000;
    let mut best: Vec<usize> = vec![usize::MAX; limit * 2];
    best[1] = 1;
    for i in 2..limit * 2 {
        for j in 2..=i.isqrt()+1 {
            if i % j == 0 {
                generate(&mut best, i, 1, j, i/j);
            }
        }
    }

    let mut used2:BitSet<usize> = BitSet::default();
    let mut result = 0;
    for i in 2..=limit {
        let b = best[i];
        if used2.insert(b) {
            result += b;
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem088() {
        let result = problem088();
        assert_eq!(result, "7587457");
    }
}
