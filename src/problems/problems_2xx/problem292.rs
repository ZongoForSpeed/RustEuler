use crate::maths::integer_root::isqrt;
use crate::register_problem;
use std::collections::HashMap;

register_problem!(
    292,
    "Pythagorean Polygons",
    problem292
);

type Triplet = (i32, i32, i32);

fn generer_triplets(longueur: i32, triplets: &mut Vec<Triplet>) {
    for dx in -longueur..=longueur {
        for dy in -longueur..=longueur {
            let carre = dx * dx + dy * dy;
            let racine = isqrt(carre);
            if racine * racine == carre && racine < longueur {
                triplets.push((dx, dy, racine));
            }
        }
    }
}

fn angle(t: &Triplet) -> f64 {
    (t.1 as f64).atan2(t.0 as f64)
}

fn colineaire(a: &Triplet, b: &Triplet) -> bool {
    a.0 * b.1 == a.1 * b.0
}

fn suivant(triplets: &[Triplet], n: usize) -> usize {
    let mut i = n + 1;
    while i < triplets.len() && colineaire(&triplets[n], &triplets[i]) {
        i += 1;
    }

    i
}

fn algorithme(
    cache: &mut HashMap<(i32, i32, i32, usize), u64>,
    triplets: &[Triplet],
    perimetre: i32,
    x: i32,
    y: i32,
    l: i32,
    i: usize,
) -> u64 {
    if x == 0 && y == 0 && l > 0 {
        return 1;
    }

    let clef = (x, y, l, i);
    if let Some(&resultat) = cache.get(&clef) {
        return resultat;
    }

    let mut resultat = 0;
    for k in i..triplets.len() {
        let (dx, dy, dl) = triplets[k];
        if l != dl || x != -dx || y != -dy {
            let nx = x + dx;
            let ny = y + dy;
            let nl = l + dl;
            let reste = perimetre - nl;

            if nx * nx + ny * ny <= reste * reste && reste >= 0 {
                resultat += algorithme(cache, triplets, perimetre, nx, ny, nl, suivant(triplets, k));
            }
        }
    }

    cache.insert(clef, resultat);
    resultat
}

/// We shall define a pythagorean polygon to be a convex polygon with the following properties:
///
///      - there are at least three vertices,
///      - no three vertices are aligned,
///      - each vertex has integer coordinates,
///      - each edge has integer length.
///
/// For a given integer n, define P(n) as the number of distinct pythagorean polygons for which the perimeter is ≤ n.
/// Pythagorean polygons should be considered distinct as long as none is a translation of another.
///
/// You are given that P(4) = 1, P(30) = 3655 and P(60) = 891045.
/// Find P(120).
pub fn problem292() -> String {
    let perimetre = 120;

    let mut triplets = Vec::new();
    generer_triplets(perimetre, &mut triplets);
    triplets.sort_by(|t1, t2| angle(t1).partial_cmp(&angle(t2)).unwrap());

    let mut cache = HashMap::new();
    let resultat = algorithme(&mut cache, &triplets, perimetre, 0, 0, 0, 0);

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem292() {
        assert_eq!(problem292(), "3600060866");
    }
}
