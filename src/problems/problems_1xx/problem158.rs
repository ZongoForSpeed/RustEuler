use crate::register_problem;

register_problem!(
    158,
    "Exploring strings for which only one character comes lexicographically after its neighbour to the left",
    problem158
);

pub fn problem158() -> String {
    // Taking three different letters from the 26 letters of the alphabet, character strings of length three can be formed.
    // Examples are 'abc', 'hat' and 'zyx'.
    // When we study these three examples we see that for 'abc' two characters come lexicographically
    // after its neighbour to the left.
    // For 'hat' there is exactly one character that comes lexicographically after its neighbour to the left. For 'zyx'
    // there are zero characters that come lexicographically after its neighbour to the left.
    // In all there are 10400 strings of length 3 for which exactly one character comes lexicographically after its
    // neighbour to the left.
    //
    // We now consider strings of n ≤ 26 different characters from the alphabet.
    // For every n, p(n) is the number of strings of length n for which exactly one character comes lexicographically
    // after its neighbour to the left.
    //
    // What is the maximum value of p(n)?
    let mut result:u64 = 0;
    let mut p2 = 1;

    let mut c = 1;
    for n in 1..27 {
        c *= 26 - n + 1;
        c /= n;
        p2 <<= 1;
        // p(n) = (2^n-n-1) * C(26,n)
        let p = (p2 - n - 1) * c;
        result = std::cmp::max(result, p);
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem158() {
        let result = problem158();
        assert_eq!(result, "409511334375");
    }
}
