use crate::maths::arithmetic::Arithmetic;
use crate::register_problem;

register_problem!(
    279,
    "Triangles with integral sides and an integral angle",
    problem279
);

/// How many triangles are there with integral sides, at least one integral angle (measured in degrees), and a
/// perimeter that does not exceed 10**8?
pub fn problem279() -> String {
    let limite: u64 = 100_000_000;

    let mut resultat = limite / 3;
    let mut m = 2;
    while 2 * m * m < limite {
        for n in 1..m {
            if u64::gcd(m, n) == 1 {
                if (m - n) % 3 != 0 {
                    resultat += limite / (2 * m * m + 2 * n * n + 5 * m * n);
                    resultat += limite / (3 * m * m + 3 * m * n);
                    resultat += limite / (2 * m * m + n * n + 3 * m * n);
                }

                if (m - n) % 2 != 0 {
                    resultat += limite / (2 * m * m + 2 * m * n);
                }
            }
        }
        m += 1;
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem279() {
        assert_eq!(problem279(), "416577688");
    }
}
