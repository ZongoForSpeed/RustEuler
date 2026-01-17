use crate::register_problem;

register_problem!(135, "Same differences", problem135);

pub fn problem135() -> String {
    // Given the positive integers, x, y, and z, are consecutive terms of an arithmetic progression, the least value of
    // the positive integer, n, for which the equation, x² − y² − z² = n, has exactly two solutions is n = 27:
    //
    //                              34² − 27² − 20² = 12² − 9² − 6² = 27
    //
    // It turns out that n = 1155 is the least value which has exactly ten solutions.
    //
    // How many values of n less than one million have exactly ten distinct solutions?
    let limit = 1000000;
    let mut counts = vec![0; limit];

    for x in 1..limit {
        for m in (x + 2) / 3..limit {
            let n = (m + x) * (3 * m - x);
            if n < limit {
                counts[n] += 1;
            } else {
                break;
            }
        }
    }

    counts.into_iter().filter(|&x| x == 10).count().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem135() {
        let result = problem135();
        assert_eq!(result, "4989");
    }
}
