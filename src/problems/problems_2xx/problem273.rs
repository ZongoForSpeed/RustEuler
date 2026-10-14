use crate::maths::integer_root::isqrt;
use crate::maths::primes::crible2;
use crate::register_problem;

register_problem!(
    273,
    "Sum of Squares",
    problem273
);

type Complexe = (i64, i64);

fn gauss(p: i64) -> Option<Complexe> {
    let mut a = 1;
    while 2 * a * a < p + 1 {
        let bb = p - a * a;
        let b = isqrt(bb);
        if b * b == bb {
            return Some(if a < b { (a, b) } else { (b, a) });
        }
        a += 1;
    }

    None
}

fn produit(z: Complexe, w: Complexe) -> Complexe {
    (z.0 * w.0 - z.1 * w.1, z.0 * w.1 + z.1 * w.0)
}

fn algorithme(racines: &[Complexe], z: Complexe) -> i64 {
    match racines.split_first() {
        None => z.0.abs().min(z.1.abs()),
        Some((&r, reste)) => {
            algorithme(reste, z)
                + algorithme(reste, produit(z, r))
                + algorithme(reste, produit(z, (r.0, -r.1)))
        }
    }
}

/// Consider equations of the form: a² + b² = N, 0 ≤ a ≤ b, a, b and N integer.
///
/// For N=65 there are two solutions:
///
/// a=1, b=8 and a=4, b=7.
///
/// We call S(N) the sum of the values of a of all solutions of a² + b² = N, 0 ≤ a ≤ b, a, b and N integer.
///
/// Thus S(65) = 1 + 4 = 5.
///
/// Find ∑S(N), for all squarefree N only divisible by primes of the form 4k+1 with 4k+1 < 150.
pub fn problem273() -> String {
    let taille = 150;
    let mut premiers = Vec::new();
    crible2::<i64, _>(taille, |p| premiers.push(p));
    premiers.retain(|p| p % 4 == 1);

    let racines: Vec<Complexe> = premiers.into_iter().filter_map(gauss).collect();

    let mut resultat = 0;
    for i in 0..racines.len() {
        resultat += algorithme(&racines[i + 1..], racines[i]);
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem273() {
        assert_eq!(problem273(), "2032447591196869022");
    }
}
