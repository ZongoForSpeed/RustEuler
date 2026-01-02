use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelIterator;
use crate::register_problem;

register_problem!(150, "Searching a triangular array for a sub-triangle having minimum-sum", problem150);

fn min_sum_i(m: &Vec<Vec<i64>>, i: usize) -> i64 {
    let mut result = 0;
    for j in 0..=i {
        let mut somme = 0;
        for k in i..m.len() {
            somme += m[k][j..k - i + j + 1].iter().sum::<i64>();
            result = std::cmp::min(result, somme);
        }
    }
    result
}

pub fn problem150() -> String {
    // In a triangular array of positive and negative integers, we wish to find a sub-triangle such that the sum of the
    // numbers it contains is the smallest possible.
    //
    // In the example below, it can be easily verified that the marked triangle satisfies this condition having a sum
    // of −42.
    //
    // matrice m { {15},
    //             {-14, -7},
    //             {20, -13, -5},
    //             {-3, 8, 23, -26},
    //             {1, -4, -5, -18, 5},
    //             {-16, 31, 2, 9, 28, 3}
    //           };
    //
    // We wish to make such a triangular array with one thousand rows, so we generate 500500 pseudo-random numbers sk in
    // the range ±2^19, using a type of random number generator (known as a Linear Congruential Generator) as follows:
    //
    //      t := 0
    //      for k = 1 up to k = 500500:
    //          t := (615949*t + 797807) modulo 2^20
    //          sk := t−2^19
    //
    // Thus: s1 = 273519, s2 = −153582, s3 = 450905 etc
    //
    // Our triangular array is then formed using the pseudo-random numbers thus:
    //
    //                                      s1
    //                                    s2  s3
    //                                  s4  s5  s6
    //                                s7  s8  s9  s10
    //                              ...
    //
    // Sub-triangles can start at any element of the array and extend down as far as we like (taking-in the two elements
    // directly below it from the next row, the three elements directly below from the row after that, and so on).
    // The "sum of a sub-triangle" is defined as the sum of all the elements it contains.
    // Find the smallest possible sub-triangle sum.
    let two20:i64 = 1 << 20;
    let two19:i64 = 1 << 19;
    let mut t:i64 = 0;

    let mut m = Vec::new();
    let mut line = Vec::new();
    for _ in 0..500500 {
        t = (615949 * t + 797807) % two20;
        line.push(t - two19);
        if line.len() > m.len() {
            m.push(line.clone());
            line = Vec::new();
        }
    }

    let result = (0..m.len()).into_par_iter().map(|i| min_sum_i(&m, i)).min().unwrap();
    result.to_string()
}
