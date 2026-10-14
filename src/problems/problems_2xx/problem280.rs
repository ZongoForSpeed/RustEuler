use crate::register_problem;
use std::collections::HashMap;

register_problem!(
    280,
    "Ant and seeds",
    problem280
);

type Cle = (usize, usize, u32, u32);

/// Résolution du système A.c = b par élimination de Gauss avec pivot partiel.
fn resolution(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Vec<f64> {
    let n = b.len();
    for k in 0..n {
        let pivot = (k..n)
            .max_by(|&i, &j| a[i][k].abs().total_cmp(&a[j][k].abs()))
            .unwrap();
        a.swap(k, pivot);
        b.swap(k, pivot);

        let ligne_k = a[k].clone();
        for i in k + 1..n {
            let f = a[i][k] / ligne_k[k];
            for j in k..n {
                a[i][j] -= f * ligne_k[j];
            }
            b[i] -= f * b[k];
        }
    }

    let mut c = vec![0.0; n];
    for i in (0..n).rev() {
        let somme: f64 = (i + 1..n).map(|j| a[i][j] * c[j]).sum();
        c[i] = (b[i] - somme) / a[i][i];
    }

    c
}

fn algorithme(cache: &mut HashMap<Cle, f64>, sx: usize, sy: usize, haut: u32, bas: u32) -> f64 {
    let clef = (sx, sy, haut, bas);
    if let Some(&resultat) = cache.get(&clef) {
        return resultat;
    }

    let mut a = vec![vec![0.0; 25]; 25];
    let mut b = vec![0.0; 25];

    for x in 0..5 {
        for y in 0..5 {
            let p = x + 5 * y;
            a[p][p] = 1.0;

            if y == 0 && bas & (1 << x) != 0 {
                if haut == 0 {
                    continue;
                }
                let _bas = bas & !(1 << x);
                b[p] = algorithme(cache, x, 4 - y, _bas, haut);
                continue;
            }

            let mut t = 0;
            if x < 4 {
                t += 1;
            }
            if x > 0 {
                t += 1;
            }
            if y < 4 {
                t += 1;
            }
            if y > 0 {
                t += 1;
            }

            let f = 1.0 / t as f64;

            b[p] = 1.0;
            if x < 4 {
                a[p][x + 1 + y * 5] = -f;
            }
            if x > 0 {
                a[p][x - 1 + y * 5] = -f;
            }
            if y < 4 {
                a[p][x + y * 5 + 5] = -f;
            }
            if y > 0 {
                a[p][x + y * 5 - 5] = -f;
            }
        }
    }

    let c = resolution(a, b);
    let resultat = c[sx + sy * 5];
    cache.insert(clef, resultat);

    resultat
}

/// A laborious ant walks randomly on a 5x5 grid. The walk starts from the central square. At each step, the ant
/// moves to an adjacent square at random, without leaving the grid; thus there are 2, 3 or 4 possible moves at each
/// step depending on the ant's position.
///
/// At the start of the walk, a seed is placed on each square of the bas row.
/// When the ant isn't carrying a seed and reaches a square of the bas row containing a seed, it will start to carry
/// the seed. The ant will drop the seed on the first empty square of the haut row it eventually reaches.
///
/// What's the expected number of steps until all seeds have been dropped in the top row?
///
/// Give your answer rounded to 6 decimal places.
pub fn problem280() -> String {
    let mut cache = HashMap::new();
    let resultat = algorithme(&mut cache, 2, 2, 31, 31);

    format!("{:.6}", resultat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem280() {
        assert_eq!(problem280(), "430.088247");
    }
}
