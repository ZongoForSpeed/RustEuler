use crate::register_problem;

register_problem!(
    291,
    "Panaitopol Primes",
    problem291
);

/// A prime number p is called a Panaitopol prime if (x**4 - y**4)/(x**3 + y**3) for some positive integers x and y.
///
/// Find how many Panaitopol primes are less than 5×10**15.
pub fn problem291() -> String {
    let limite: usize = 50_000_000;

    let mut dp: Vec<u64> = (0..=limite as u64).map(|i| 2 * i * i + 2 * i + 1).collect();

    let mut resultat = 0;
    for i in 1..limite {
        let ii = i as u64;
        if dp[i] == 2 * ii * ii + 2 * ii + 1 {
            resultat += 1;
        }

        if dp[i] > 1 {
            let p = dp[i];
            for j in (i..limite).step_by(p as usize) {
                while dp[j] % p == 0 {
                    dp[j] /= p;
                }
            }

            // p - 1 - i peut déborder (comme en non signé en C++) : la boucle est alors vide
            let debut = p.wrapping_sub(1).wrapping_sub(ii) as usize;
            for j in (debut..limite).step_by(p as usize) {
                while dp[j] % p == 0 {
                    dp[j] /= p;
                }
            }
        }
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem291() {
        assert_eq!(problem291(), "4037526");
    }
}
