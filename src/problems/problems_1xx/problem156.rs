use rayon::iter::ParallelIterator;
use crate::register_problem;
use std::collections::{HashMap, HashSet};
use rayon::iter::IntoParallelIterator;
use crate::maths::digits::Digits;

register_problem!(156, "Counting Digits", problem156);

fn count_s(limite: i64, m: i64, d: i64) -> i64 {
    let mut gn: HashMap<(i64, i64), HashSet<i64>> = HashMap::new();
    let mut f = 0;
    let mut s = 0;
    for b in 0..m {
        f += b.count_digit(d, 10) as i64;
        for x in 0..10 {
            let g = (b + 1) * x + f - b;
            gn.entry((g, x)).or_insert_with(HashSet::new).insert(b);
        }
        if f == b {
            s += b;
        }
    }

    let mut fa = 0;
    for a in 1..limite / m {
        let x = a.count_digit(d, 10) as i64;
        let g = m * a - m * fa - a * f;
        fa += x;
        if let Some(find) = gn.get(&(g, x)) {
            for b in find {
                s += m * a + b;
            }
        }
    }
    println!("S({d}) = {}", s);
    s
}


pub fn problem156() -> String {
    // Starting from zero the natural numbers are written down in base 10 like this:
    //
    //                              0 1 2 3 4 5 6 7 8 9 10 11 12....
    //
    // Consider the digit d=1. After we write down each number n, we will update the number of ones that have occurred
    // and call this number f(n,1). The first values for f(n,1), then, are as follows:
    //
    //                                      n	f(n,1)
    //                                      0	0
    //                                      1	1
    //                                      2	1
    //                                      3	1
    //                                      4	1
    //                                      5	1
    //                                      6	1
    //                                      7	1
    //                                      8	1
    //                                      9	1
    //                                      10	2
    //                                      11	4
    //                                      12	5
    //
    // Note that f(n,1) never equals 3.
    // So the first two solutions of the equation f(n,1)=n are n=0 and n=1. The next solution is n=199981.
    //
    // In the same manner the function f(n,d) gives the total number of digits d that have been written down after the
    // number n has been written.
    // In fact, for every digit d ≠ 0, 0 is the first solution of the equation f(n,d)=n.
    //
    // Let s(d) be the sum of all the solutions for which f(n,d)=n.
    // You are given that s(1)=22786974071.
    //
    // Find ∑ s(d) for 1 ≤ d ≤ 9.
    //
    // Note: if, for some n, f(n,d)=n for more than one value of d this value of n is counted again for every value of d
    // for which f(n,d)=n.
    let limit = 1000000000000;
    let m = 100000;

    (1..10).into_par_iter().map(|d| count_s(limit, m, d)).sum::<i64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem156() {
        let result = problem156();
        assert_eq!(result, "21295121502550");
    }
}
