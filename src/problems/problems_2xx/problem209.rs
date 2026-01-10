use crate::maths::factorial::Factorial;
use crate::register_problem;
use disjoint::DisjointSet;

register_problem!(209, "Circular Logic", problem209);

fn combinaison(n: u64) -> u64 {
    let mut result = 0;
    for k in 0..=(n/2) {
        if k == 0 {
            result += 1;
        } else {
            result += n * u64::binomial(n - k - 1, k - 1) / k;
        }
    }
    result
}

pub fn problem209() -> String {
    // A k-input binary truth table is a map from k input bits (binary digits, 0 [false] or 1 [true]) to 1 output bit.
    // For example, the 2-input binary truth tables for the logical AND and XOR functions are:
    //
    //                  x	y	x AND y         x	y	x XOR y
    //                  0	0	    0           0	0	    0
    //                  0	1	    0           0	1	    1
    //                  1	0	    0           1	0	    1
    //                  1	1	    1           1	1	    0
    //
    // How many 6-input binary truth tables, τ, satisfy the formula
    //
    // τ(a, b, c, d, e, f) AND τ(b, c, d, e, f, a XOR (b AND c)) = 0
    //
    // for all 6-bit inputs (a, b, c, d, e, f) ?
    let limite = 1 << 6;

    let mut forest = DisjointSet::with_len(limite);

    for i in 0..limite {
        let a = i >> 5 & 1;
        let b = i >> 4 & 1;
        let c = i >> 3 & 1;
        let j = (i << 1 & 63) + (a ^ (b & c));

        // println!("{i} -> {j}");
        forest.join(i, j);
    }

    println!("{:?}", forest.sets());

    let mut result = 1;
    for set in forest.sets() {
        result *= combinaison(set.len() as u64);
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem209::problem209;

    #[test]
    fn test_problem209() {
        let result = problem209();
        assert_eq!(result, "15964587728784");
    }
}
