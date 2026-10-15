use crate::maths::arithmetic::Arithmetic;
use crate::maths::integer_root::isqrt;
use crate::register_problem;
use std::collections::BTreeSet;

register_problem!(
    295,
    "Lenticular holes",
    problem295
);

fn lenticular(x: i64, y: i64, a: i64, b: i64) -> bool {
    let r = x * x + y * y;
    for v in 1..b {
        let u = v * a / b;
        if (x - u) * (x - u) + (y + v) * (y + v) < r {
            return false;
        }
    }

    true
}

fn diophantienne(a: i64, b: i64, c: i64) -> (i64, i64) {
    // Résolution de l'equation ax + by = c
    let (mut u, mut v, _) = i64::bezout(a, b);

    u *= c;
    v *= -c;

    let p = std::cmp::min((u - a) / b, v / a);

    u -= p * b;
    v -= p * a;

    (u, v)
}

/// We call the convex area enclosed by two circles a lenticular hole if:
///
///      - The centres of both circles are on lattice points.
///      - The two circles intersect at two distinct lattice points.
///      - The interior of the convex area enclosed by both circles does not contain any lattice points.
///
/// Consider the circles:
/// C0: x²+y²=25
/// C1: (x+4)²+(y-4)²=1
/// C2: (x-12)²+(y-4)²=65
///
/// The circles C0, C1 and C2 are drawn in the picture below.
///
/// C0 and C1 form a lenticular hole, as well as C0 and C2.
///
/// We call an ordered pair of positive real numbers (r1, r2) a lenticular pair if there exist two circles with radii
/// r1 and r2 that form a lenticular hole. We can verify that (1, 5) and (5, √65) are the lenticular pairs of the
/// example above.
///
/// Let L(N) be the number of distinct lenticular pairs (r1, r2) for which 0 < r1 ≤ r2 ≤ N.
/// We can verify that L(10) = 30 and L(100) = 3442.
///
/// Find L(100 000).
pub fn problem295() -> String {
    let rayon_max: i64 = 100_000;
    let limite = 2 * isqrt(rayon_max);

    let mut rayons: Vec<BTreeSet<i64>> = Vec::new();
    for dx in (1..limite).step_by(2) {
        for dy in (1..=dx).step_by(2) {
            if i64::gcd(dx, dy) != 1 {
                continue;
            }

            let mut r = BTreeSet::new();

            let (mut x, mut y) = diophantienne(dx, dy, (dx * dx + dy * dy) / 2);
            loop {
                let rayon = x * x + y * y;
                if rayon > rayon_max * rayon_max {
                    break;
                }
                if lenticular(x, y, dx, dy) {
                    r.insert(rayon);
                }

                x += dy;
                y += dx;
            }

            if !r.is_empty() {
                rayons.push(r);
            }
        }
    }

    let mut resultat = 0;
    for i in 0..rayons.len() {
        let (avant, apres) = rayons.split_at_mut(i + 1);
        let ri = &avant[i];

        let mut rem: BTreeSet<(i64, i64)> = BTreeSet::new();
        for rj in apres.iter_mut() {
            // rem |= {(u, v) for u in Ri & Rj for v in Rj - Ri}
            // Rj -= Ri
            let intersection: Vec<i64> = ri.iter().filter(|r| rj.contains(r)).copied().collect();
            let difference: Vec<i64> = rj.iter().filter(|r| !ri.contains(r)).copied().collect();

            for &u in &intersection {
                for &v in &difference {
                    rem.insert((u, v));
                }
            }

            for n in ri {
                rj.remove(n);
            }
        }

        let taille = ri.len() as i64;
        resultat += taille * (taille + 1) / 2 + rem.len() as i64;
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem295() {
        assert_eq!(problem295(), "4884650818");
    }
}
