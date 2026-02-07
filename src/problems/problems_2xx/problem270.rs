use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;

register_problem!(
    270,
    "Polynomials with at least one integer root",
    problem270
);

fn sign(n: u64) -> i64 {
    if n % 2 == 0 { 1 } else { -1 }
}

fn g0(n: u64) -> MpzNumber {
    MpzNumber::catalan(n - 2)
}

fn g1(n: u64, k1: u64) -> MpzNumber {
    let mut somme = MpzNumber::new();
    for j in 0.. {
        if 2 * j >= k1 + 1 {
            break;
        }
        somme += sign(j) * MpzNumber::binomial_ui(k1 - j, j) * g0(n - j);
    }

    somme
}

fn g2(n: u64, k1: u64, k2: u64) -> MpzNumber {
    let mut somme = MpzNumber::new();
    for j in 0.. {
        if 2 * j >= k1 + 1 {
            break;
        }
        somme += sign(j) * MpzNumber::binomial_ui(k1 - j, j) * g1(n - j, k2);
    }

    somme
}

fn g3(n: u64, k1: u64, k2: u64, k3: u64) -> MpzNumber {
    let mut somme = MpzNumber::new();
    for j in 0.. {
        if 2 * j >= k1 + 1 {
            break;
        }
        somme += sign(j) * MpzNumber::binomial_ui(k1 - j, j) * g2(n - j, k2, k3);
    }

    somme
}

fn g4(n: u64, k1: u64, k2: u64, k3: u64, k4: u64) -> MpzNumber {
    let mut somme = MpzNumber::new();
    for j in 0.. {
        if 2 * j >= k1 + 1 {
            break;
        }
        somme += sign(j) * MpzNumber::binomial_ui(k1 - j, j) * g3(n - j, k2, k3, k4);
    }

    somme
}

fn c(n: u64) -> MpzNumber {
    g4(4 * n, n, n, n, n)
}

/// A square piece of paper with integer dimensions N×N is placed with a corner at the origin and
/// two of its sides along the x- and y-axes. Then, we cut it up respecting the following rules:
///
///  . We only make straight cuts between two points lying on different sides of the square, and
///    having integer coordinates.
///  . Two cuts cannot cross, but several cuts can meet at the same border point.
///  . Proceed until no more legal cuts can be made.
///
/// Counting any reflections or rotations as distinct, we call C(N) the number of ways to cut an N×N
/// square. For example, C(1) = 2 and C(2) = 30 (shown below).
///
///                          p270_CutSquare.gif
///
/// What is C(30) mod 10**8 ?
pub fn problem270() -> String {
    let result = c(30) % 100000000;
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem270() {
        assert_eq!(problem270(), "82282080");
    }
}
