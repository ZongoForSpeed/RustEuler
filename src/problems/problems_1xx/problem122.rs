use crate::register_problem;
use itertools::Itertools;

register_problem!(122, "Efficient exponentiation", problem122);

pub fn problem122() -> String {
    // The most naive way of computing n^15 requires fourteen multiplications:
    //
    //      n × n × ... × n = n^15
    //
    // But using a "binary" method you can compute it in six multiplications:
    //
    //      n × n = n^2
    //      n^2 × n^2 = n^4
    //      n^4 × n^4 = n^8
    //      n^8 × n^4 = n^12
    //      n^12 × n^2 = n^14
    //      n^14 × n = n^15
    //
    // However it is yet possible to compute it in only five multiplications:
    //
    //      n × n = n2
    //      n^2 × n = n3
    //      n^3 × n^3 = n6
    //      n^6 × n^6 = n12
    //      n^12 × n^3 = n15
    //
    // We shall define m(k) to be the minimum number of multiplications to compute n^k;
    // for example m(15) = 5.
    //
    // For 1 ≤ k ≤ 200, find ∑ m(k).
    let mut cache: Vec<Vec<Vec<usize>>> = vec![vec![vec![1]]];
    let mut sum = 0;
    for k in 2..=200 {
        let mut min_exponent: Vec<Vec<usize>> = Vec::new();
        let mut min_length = usize::MAX;
        for t in &cache {
            for exp in t {
                for (a, b) in exp.into_iter().cartesian_product(exp.into_iter()) {
                    if a <= b && a + b == k {
                        let mut exponent = exp.clone();
                        exponent.push(k);
                        if exponent.len() == min_length {
                            min_exponent.push(exponent);
                        } else if exponent.len() < min_length {
                            min_length = exponent.len();
                            min_exponent.clear();
                            min_exponent.push(exponent);
                        }
                    }
                }
            }
        }
        cache.push(min_exponent);
        sum += min_length - 1;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem122() {
        let result = problem122();
        assert_eq!(result, "1582");
    }
}
