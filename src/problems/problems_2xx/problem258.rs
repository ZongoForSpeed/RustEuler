use crate::register_problem;

register_problem!(258, "A lagged Fibonacci sequence", problem258);

fn multiply(x: &[u64], y: &[u64], modulus: u64, z: &mut [u64], res: &mut [u64]) {
    let n = x.len();
    z.fill(0);
    for i in 0..n {
        if x[i] == 0 {
            continue;
        }
        let xi = x[i];
        for j in 0..n {
            z[i + j] += xi * y[j];
        }
    }

    for i in (n..2 * n).rev() {
        let val = z[i] % modulus;
        if val == 0 {
            continue;
        }
        z[i - n] += val;
        z[i - n + 1] += val;
    }

    for i in 0..n {
        res[i] = z[i] % modulus;
    }
}

fn power_mod(m: &[u64], mut exponent: u64, modulus: u64) -> Vec<u64> {
    let n = m.len();
    let mut base = m.to_vec();
    let mut result = vec![0; n];
    result[0] = 1;

    let mut z = vec![0u64; n * 2];
    let mut tmp = vec![0; n];

    while exponent > 0 {
        if exponent & 1 == 1 {
            multiply(&result, &base, modulus, &mut z, &mut tmp);
            result.copy_from_slice(&tmp);
        }
        if exponent > 1 {
            multiply(&base, &base, modulus, &mut z, &mut tmp);
            base.copy_from_slice(&tmp);
        }
        exponent >>= 1;
    }
    result
}

/// A sequence is defined as:
///
/// gk = 1, for 0 ≤ k ≤ 1999
/// gk = gk-2000 + gk-1999, for k ≥ 2000.
/// Find gk mod 20092010 for k = 10^18.
pub fn problem258() -> String {
    let k = 1_000_000_000_000_000_000;
    let modulus = 20_092_010;

    let mut m = vec![0; 2000];
    m[1] = 1;

    let g = power_mod(&m, k, modulus);
    let result = g.iter().sum::<u64>() % modulus;

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem258::problem258;

    #[test]
    fn test_problem258() {
        let result = problem258();
        assert_eq!(result, "12747994");
    }
}
