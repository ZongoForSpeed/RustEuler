use num_traits::PrimInt;

pub(crate) fn fibonacci<N>() -> impl Iterator<Item = N>
where
    N: PrimInt + std::ops::DivAssign,
{
    std::iter::successors(Some((N::zero(), N::one())), |(a, b)| Some((*b, a.add(*b))))
        .map(|(a, _)| a)
}

pub(crate) fn fibonacci_limit<N>(limite: N) -> impl Iterator<Item = N>
where
    N: PrimInt + std::ops::DivAssign + 'static,
{
    fibonacci::<N>().take_while(move |&a| a < limite)
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
