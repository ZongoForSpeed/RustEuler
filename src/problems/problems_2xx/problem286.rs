use crate::register_problem;

register_problem!(
    286,
    "Scoring probabilities",
    problem286
);

/// Probabilité d'obtenir exactement 20 points sur les 50 tirs, un tir à distance x réussissant avec probabilité 1 - x/q.
/// Loi de Poisson binomiale, calculée par programmation dynamique (que des termes positifs, donc stable).
fn probabilite(q: f64) -> f64 {
    let mut dp = [0.0f64; 21];
    dp[0] = 1.0;
    for x in 1..=50 {
        let p = 1.0 - x as f64 / q;
        for k in (1..=20).rev() {
            dp[k] = dp[k] * (1.0 - p) + dp[k - 1] * p;
        }
        dp[0] *= 1.0 - p;
    }

    dp[20]
}

/// Barbara is a mathematician and a basketball player. She has found that the probability of scoring a point when
/// shooting from a distance x is exactly (1 - x/q), where q is a real constant greater than 50.
///
/// During each practice run, she takes shots from distances x = 1, x = 2, ..., x = 50 and, according to her records,
/// she has precisely a 2 % chance to score a total of exactly 20 points.
///
/// Find q and give your answer rounded to 10 decimal places.
pub fn problem286() -> String {
    let objectif = 0.02;

    // la probabilité décroît quand q augmente : dichotomie sur q
    let mut bas = 50.0;
    let mut haut = 1000.0;
    for _ in 0..200 {
        let milieu = (bas + haut) / 2.0;
        if probabilite(milieu) > objectif {
            bas = milieu;
        } else {
            haut = milieu;
        }
    }

    format!("{:.10}", (bas + haut) / 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem286() {
        assert_eq!(problem286(), "52.6494571953");
    }
}
