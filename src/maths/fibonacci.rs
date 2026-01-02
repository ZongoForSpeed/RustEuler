use num_traits::{One, Zero};
use std::ops::{Add, DivAssign};

pub(crate) fn fibonacci<N>() -> impl Iterator<Item = N>
where
    N: Zero + One + DivAssign + Add + Clone,
{
    std::iter::successors(Some((N::zero(), N::one())), next_fib).map(|(a, _)| a)
}

fn next_fib<N>(t: &(N, N)) -> Option<(N, N)>
where
    N: Zero + One + DivAssign + Add + Clone,
{
    let ab = t.0.clone() + t.1.clone();
    Some((t.1.clone(), ab))
}

pub(crate) fn fibonacci_limit<N>(limit: N) -> impl Iterator<Item = N>
where
    N: Zero + One + DivAssign + 'static + PartialOrd + Clone,
{
    fibonacci::<N>().take_while(move |a| *a < limit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let expected = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        let result: Vec<u64> = fibonacci().take(10).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fibonacci_limite() {
        let expected = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
        let result: Vec<u64> = fibonacci_limit(100).collect();
        assert_eq!(result, expected);
    }
}
