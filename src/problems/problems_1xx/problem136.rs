use crate::register_problem;

register_problem!(136, "Singleton difference", problem136);

pub fn problem136() -> String {
    // The positive integers, x, y, and z, are consecutive terms of an arithmetic progression.
    // Given that n is a positive integer, the equation, x² − y² − z² = n, has exactly one solution when n = 20:
    //
    //                                          13² − 10² − 7² = 20
    //
    // In fact there are twenty-five values of n below one hundred for which the equation has a unique solution.
    //
    // How many values of n less than fifty million have exactly one solution?
    let limit = 50000000;
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

    counts.into_iter().filter(|&x| x == 1).count().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem136() {
        let result = problem136();
        assert_eq!(result, "2544559");
    }
}
