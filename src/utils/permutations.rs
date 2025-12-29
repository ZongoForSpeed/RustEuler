use permutohedron::LexicalPermutation;

pub(crate) fn permutations<N: Ord + Clone>(
    mut v: Vec<N>,
) -> impl Iterator<Item = Vec<N>> {
    let mut first = true;

    std::iter::from_fn(move || {
        if first {
            first = false;
            return Some(v.clone());
        }

        if v.next_permutation() {
            Some(v.clone())
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {

        let expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1]
        ];

        let result :Vec<Vec<i32>> = permutations(vec![1, 2, 3]).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_permutations_duplicate() {
        let expected = vec![
            vec![1, 2, 2, 4],
            vec![1, 2, 4, 2],
            vec![1, 4, 2, 2],
            vec![2, 1, 2, 4],
            vec![2, 1, 4, 2],
            vec![2, 2, 1, 4],
            vec![2, 2, 4, 1],
            vec![2, 4, 1, 2],
            vec![2, 4, 2, 1],
            vec![4, 1, 2, 2],
            vec![4, 2, 1, 2],
            vec![4, 2, 2, 1]
        ];

        let result :Vec<Vec<i32>> = permutations(vec![1, 2, 2, 4]).collect();
        assert_eq!(result, expected);
    }
}

