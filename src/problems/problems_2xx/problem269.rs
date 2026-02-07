use std::collections::HashMap;
use crate::register_problem;

register_problem!(269, "Polynomials with at least one integer root", problem269);

type Cache = HashMap<([i16; 10], usize), i64>;

fn algorithm(cache: &mut Cache, residue: [i16; 10], digits: usize) -> i64 {
    if let Some(&res) = cache.get(&(residue, digits)) {
        return res;
    }

    let mut result = 0;
    if digits == 0 {
        if residue.contains(&0) {
            result = 1;
        }
    } else {
        for d in 0..10 {
            let mut new_residue = [0; 10];
            for i in 0..10 {
                let r = residue[i as usize] as i64;
                let next_r = d as i64 - r * i as i64;

                if i > 1 {
                    let max_residue = next_r * (1 - i as i64 * i as i64);
                    if max_residue > 8 || max_residue < -9 * i as i64 {
                        new_residue[i as usize] = i16::MIN;
                    } else {
                        new_residue[i as usize] = next_r as i16;
                    }
                } else {
                    new_residue[i as usize] = next_r as i16;
                }
            }
            result += algorithm(cache, new_residue, digits - 1);
        }
    }

    cache.insert((residue, digits), result);
    result
}

/// A root or zero of a polynomial P(x) is a solution to the equation P(x) = 0.
/// Define Pn as the polynomial whose coefficients are the digits of n.
/// For example, P5703(x) = 5x**3 + 7x² + 3.
///
/// We can see that:
///
///      Pn(0) is the last digit of n,
///      Pn(1) is the sum of the digits of n,
///      Pn(10) is n itself.
///
/// Define Z(k) as the number of positive integers, n, not exceeding k for which the polynomial Pn has at least one
/// integer root.
///
/// It can be verified that Z(100 000) is 14696.
///
/// What is Z(10**16)?
pub fn problem269() -> String {
    let residue = [0; 10];
    let mut cache = Cache::new();
    let result = algorithm(&mut cache, residue, 16);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem269() {
        assert_eq!(problem269(), "1311109198529286");
    }
}
