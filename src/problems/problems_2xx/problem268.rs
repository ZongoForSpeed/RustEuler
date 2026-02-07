use crate::register_problem;

register_problem!(
    268,
    "Counting numbers with at least four distinct prime factors less than 100",
    problem268
);

/// It can be verified that there are 23 positive integers less than 1000 that are divisible by at least four
/// distinct primes less than 100.
///
/// Find how many positive integers less than 10**16 are divisible by at least four distinct primes less than 100.
pub fn problem268() -> String {
    const N: u64 = 10_000_000_000_000_000;
    let primes = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];

    let mut res = 0;

    let max:u64 = 1 << primes.len();
    for n in 0..max {
        let count = n.count_ones();
        if count < 4 {
            continue;
        }
        let mut m = n;
        let mut product = 1;
        while m != 0 {
            let bit = m.trailing_zeros() as usize;
            product *= primes[bit];
            if product > N {
                break;
            }
            m &= m - 1;
        }

        if product < N {
            let coeff = (count - 1) * (count - 2) * (count - 3) / 6;
            let term = (N / product) * coeff as u64;
            if count % 2 == 1 {
                res -= term;
            } else {
                res += term;
            }
        }
    }

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem268() {
        assert_eq!(problem268(), "785478606870985");
    }
}
