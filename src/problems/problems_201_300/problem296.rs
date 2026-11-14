use crate::maths::arithmetic::Arithmetic;
use crate::register_problem;

register_problem!(296, "Angular Bisector and Tangent", problem296);

/// Given is an integer sided triangle ABC with BC ≤ AC ≤ AB.
/// k is the angular bisector of angle ACB.
/// m is the tangent at C to the circumscribed circle of ABC.
/// n is a line parallel to m through B.
/// The intersection of n and k is called E.
///
/// p296_bisector.gif
/// How many triangles ABC with a perimeter not exceeding 100 000 exist such that BE has integral length?
pub fn problem296() -> String {
    let n: u64 = 100_000;
    let mut resultat = 0;
    for a in 1..=n / 3 {
        for b in a..=(n - a) / 2 {
            let d = (a + b) / u64::gcd(a, b);
            let cmin = b;
            let cmax = (a + b - 1).min(n - a - b);
            resultat += cmax / d - (cmin - 1) / d;
        }
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem296() {
        assert_eq!(problem296(), "1137208419");
    }
}
