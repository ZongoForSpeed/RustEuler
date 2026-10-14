use crate::register_problem;

register_problem!(
    276,
    "Primitive Triangles",
    problem276
);

fn nombre_triangles(p: u64) -> u64 {
    // http://mathworld.wolfram.com/IntegerTriangle.html
    if p % 2 == 1 {
        ((p + 3) * (p + 3) + 24) / 48
    } else {
        (p * p + 24) / 48
    }
}

/// Consider the triangles with integer sides a, b and c with a ≤ b ≤ c.
///
/// An integer sided triangle (a,b,c) is called primitive if gcd(a,b,c)=1.
///
/// How many primitive integer sided triangles exist with a perimeter not exceeding 10 000 000?
pub fn problem276() -> String {
    let taille = 10_000_000;
    let mut resultat = 0;

    let mut dp = vec![0u64; taille + 1];

    for i in 3..taille + 1 {
        let u = nombre_triangles(i as u64) - dp[i];
        resultat += u;
        for j in (2 * i..taille + 1).step_by(i) {
            dp[j] += u;
        }
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem276() {
        assert_eq!(problem276(), "5777137137739632912");
    }
}
