use crate::register_problem;

register_problem!(242, "Odd Triplets", problem242);

fn c(n: u64) -> u64 {
    let sum = u64::count_ones((n - 1) / 4);
    1 << sum
}

fn odd_triplets(n: u64) -> u64 {
    if n < 1 {
        0
    } else if n == 1 {
        1
    } else if n % 4 != 1 {
        odd_triplets(n - (n + 3) % 4)
    } else if n % 8 == 1 {
        c(n) + odd_triplets(n - 4)
    } else {
        3 * odd_triplets((n - 3) / 2)
    }
}
/// Given the set {1,2,...,n}, we define f(n,k) as the number of its k-element subsets with an odd sum of elements.
/// For example, f(5,3) = 4, since the set {1,2,3,4,5} has four 3-element subsets having an odd sum of elements,
/// i.e.: {1,2,4}, {1,3,5}, {2,3,4} and {2,4,5}.
///
/// When all three values n, k and f(n,k) are odd, we say that they make an odd-triplet [n,k,f(n,k)].
///
/// There are exactly five odd-triplets with n ≤ 10, namely:
/// [1,1,f(1,1) = 1], [5,1,f(5,1) = 3], [5,5,f(5,5) = 1], [9,1,f(9,1) = 5] and [9,9,f(9,9) = 1].
///
/// How many odd-triplets are there with n ≤ 1012 ?
pub fn problem242() -> String {
    let result = odd_triplets(u64::pow(10, 12));
    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem242::problem242;

    #[test]
    fn test_problem242() {
        let result = problem242();
        assert_eq!(result, "997104142249036713");
    }
}
