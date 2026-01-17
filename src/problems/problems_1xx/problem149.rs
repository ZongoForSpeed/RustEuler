use crate::register_problem;

register_problem!(149, "Searching for a maximum-sum subsequence", problem149);

fn maximum(v: &Vec<i64>) -> i64 {
    let mut max_tmp = 0;
    let mut max_total = 0;
    for n in v {
        max_tmp = std::cmp::max(max_tmp + *n, 0);
        max_total = std::cmp::max(max_total, max_tmp);
    }
    max_total
}

pub fn problem149() -> String {
    // Looking at the table below, it is easy to verify that the maximum possible sum of adjacent numbers in any
    // direction (horizontal, vertical, diagonal or anti-diagonal) is 16 (= 8 + 7 + 1).
    //
    //	                                           −2   5   3   2
    //                                              9  −6   5   1
    //                                              3   2   7   3
    //                                             −1   8  −4   8
    //
    // Now, let us repeat the search, but on a much larger scale:
    //
    // First, generate four million pseudo-random numbers using a specific form of what is known as a
    // "Lagged Fibonacci Generator":
    //
    // For 1 ≤ k ≤ 55, sk = [100003 − 200003.k + 300007.k^3] (modulo 1000000) − 500000.
    // For 56 ≤ k ≤ 4000000, sk = [sk−24 + sk−55 + 1000000] (modulo 1000000) − 500000.
    //
    // Thus, s10 = −393027 and s100 = 86613.
    //
    // The terms of s are then arranged in a 2000×2000 table, using the first 2000 numbers to fill the
    // first row (sequentially), the next 2000 numbers to fill the second row, and so on.
    //
    // Finally, find the greatest sum of (any number of) adjacent entries in any direction (horizontal,
    // vertical, diagonal or anti-diagonal).
    let size = 2000;
    let mut s = vec![0; size * size + 1];
    for k in 1..56 {
        let ik = k as i64;
        s[k] = (100003 - 200003 * ik + 300007 * ik * ik * ik) % 1000000 - 500000;
    }

    for k in 56..size * size {
        s[k] = (s[k - 24] + s[k - 55] + 1000000) % 1000000 - 500000;
    }

    let mut m: Vec<Vec<i64>> = vec![vec![0; size]; size];

    for (n, v) in s.iter().enumerate().skip(1) {
        m[(n - 1) / size][(n - 1) % size] = *v;
    }

    let mut result = 0;

    for i in 0..size {
        // Somme ligne
        result = std::cmp::max(result, maximum(&m[i]));

        {
            // Somme colonne
            let mut v = Vec::with_capacity(size);
            for j in &m {
                v.push(j[i]);
            }
            result = std::cmp::max(result, maximum(&v));
        }

        {
            // Somme diagonale avec i = 0
            let mut v = Vec::with_capacity(size);
            for j in 0..size - i {
                v.push(m[i + j][j]);
            }

            result = std::cmp::max(result, maximum(&v));
        }

        {
            // Somme diagonale avec j = 0
            let mut v = Vec::with_capacity(size);
            for j in 0..size - i {
                v.push(m[j][i + j]);
            }

            result = std::cmp::max(result, maximum(&v));
        }

        {
            // Somme anti-diagonale avec i = 0
            let mut v = Vec::with_capacity(size);
            for j in 0..=i {
                v.push(m[i - j][j]);
            }

            result = std::cmp::max(result, maximum(&v));
        }

        {
            // Somme anti-diagonale avec i = m.size() - 1
            let mut v = Vec::with_capacity(size);
            for j in 0..size - i {
                v.push(m[i + j][size - 1 - j]);
            }

            result = std::cmp::max(result, maximum(&v));
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem149() {
        let result = problem149();
        assert_eq!(result, "52852124");
    }
}
