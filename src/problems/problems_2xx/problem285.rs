use crate::register_problem;

register_problem!(
    285,
    "Pythagorean odds",
    problem285
);

fn aire_section_disque(r: f64, x: f64, y: f64) -> f64 {
    // http://villemin.gerard.free.fr/GeomLAV/Cercle/aaaAIRE/RectangT.htm
    // acos(sqrt(r²-x²)/r) = atan2(x, sqrt(r²-x²)) : évite la perte de précision de acos près de 1
    r * r * (y / r).acos() / 2.0 - y * (r * r - y * y).sqrt() / 2.0
        - r * r * x.atan2((r * r - x * x).sqrt()) / 2.0
        + (r * r - x * x).sqrt() * x.abs() / 2.0
        - x * ((r * r - x * x).sqrt() - y)
}

/// Albert chooses a positive integer k, then two real numbers a, b are randomly chosen in the interval [0,1] with
/// uniform distribution.
///
/// The square root of the sum (k·a+1)² + (k·b+1)² is then computed and rounded to the nearest integer. If the result
/// is equal to k, he scores k points; otherwise he scores nothing.
///
/// For example, if k = 6, a = 0.2 and b = 0.85, then (k·a+1)2 + (k·b+1)2 = 42.05.
///
/// The square root of 42.05 is 6.484... and when rounded to the nearest integer, it becomes 6.
/// This is equal to k, so he scores 6 points.
///
/// It can be shown that if he plays 10 turns with k = 1, k = 2, ..., k = 10, the expected value of his total score,
/// rounded to five decimal places, is 10.20914.
///
/// If he plays 10**5 turns with k = 1, k = 2, k = 3, ..., k = 10**5, what is the expected value of his total score,
/// rounded to five decimal places?
pub fn problem285() -> String {
    let mut resultat = 0.0;
    // a et b appartient [0,1]
    // (k - 1/2)² < (ka+1)² + (kb+1)² < (k + 1/2)²
    // (1 - 1/(2k))² < (a+1/k)² + (b+1/k)² < (1 + 1/(2k))²
    // correspond aux equations des cercles :
    //      - de centre (-1/k, -1/k) et rayon (1 + 1/(2k))², C1
    //      - de centre (-1/k, -1/k) et rayon (1 - 1/(2k))², C2
    // la probabilité de gagner devient l'aire de l'intersection du carré
    // [0,1] x [0,1] avec le "donut" C2 - C1
    //
    // en translatant les cercles les cercles à l'origine, le carre devient :
    //      [1/k, 1+1/k] x [1/k, 1+1/k]
    for k in 1..=100000u64 {
        let kk = k as f64;
        let mut p = aire_section_disque(1.0 + 1.0 / (2.0 * kk), 1.0 / kk, 1.0 / kk);
        // dans le cas où k=1, l'aire de la petite section est nulle
        if k > 1 {
            p -= aire_section_disque(1.0 - 1.0 / (2.0 * kk), 1.0 / kk, 1.0 / kk);
        }
        resultat += kk * p;
    }

    format!("{:.5}", resultat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem285() {
        assert_eq!(problem285(), "157055.80999");
    }
}
