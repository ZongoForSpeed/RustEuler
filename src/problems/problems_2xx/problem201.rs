use crate::register_problem;

register_problem!(201, "Subsets with a unique sum", problem201);

pub fn problem201() -> String {
    // For any set A of numbers, let sum(A) be the sum of the elements of A.
    // Consider the set B = {1,3,6,8,10,11}.
    // There are 20 subsets of B containing three elements, and their sums are:
    //
    //                                  sum({1,3,6}) = 10,
    //                                  sum({1,3,8}) = 12,
    //                                  sum({1,3,10}) = 14,
    //                                  sum({1,3,11}) = 15,
    //                                  sum({1,6,8}) = 15,
    //                                  sum({1,6,10}) = 17,
    //                                  sum({1,6,11}) = 18,
    //                                  sum({1,8,10}) = 19,
    //                                  sum({1,8,11}) = 20,
    //                                  sum({1,10,11}) = 22,
    //                                  sum({3,6,8}) = 17,
    //                                  sum({3,6,10}) = 19,
    //                                  sum({3,6,11}) = 20,
    //                                  sum({3,8,10}) = 21,
    //                                  sum({3,8,11}) = 22,
    //                                  sum({3,10,11}) = 24,
    //                                  sum({6,8,10}) = 24,
    //                                  sum({6,8,11}) = 25,
    //                                  sum({6,10,11}) = 27,
    //                                  sum({8,10,11}) = 29.
    //
    // Some of these sums occur more than once, others are unique.
    // For a set A, let U(A,k) be the set of unique sums of k-element subsets of A, in our example
    // we find U(B,3) = {10,12,14,18,21,25,27,29} and sum(U(B,3)) = 156.
    //
    // Now consider the 100-element set S = {1², 2², ... , 100²}.
    // S has 100891344545564193334812497256 50-element subsets.
    //
    // Determine the sum of all integers which are the sum of exactly one of the 50-element subsets
    // of S, i.e. find sum(U(S,50)).
    let squares = (1..101).map(|k| k * k).collect::<Vec<usize>>();

    let k_max = 50;
    let min = squares[..50].iter().sum::<usize>();
    let max = squares[50..].iter().sum::<usize>();

    let mut somme = vec![vec![0; max + 1]; k_max + 1];
    somme[0][0] = 1;
    for c in squares {
        let mut tmp = somme.clone();
        tmp[0][0] = 1;
        for k in 0..k_max {
            for s in 0..c {
                tmp[k + 1][s] = somme[k + 1][s];
            }
            for s in c..=max {
                tmp[k + 1][s] = somme[k][s - c] + somme[k + 1][s];
            }
        }
        somme = tmp;
    }

    let mut result = 0;
    for n in min..=max {
        if somme[k_max][n] == 1 {
            result += n;
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem201::problem201;

    #[test]
    fn test_problem201() {
        let result = problem201();
        assert_eq!(result, "115039000");
    }
}
