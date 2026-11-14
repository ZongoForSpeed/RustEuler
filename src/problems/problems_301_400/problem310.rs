use crate::register_problem;
use std::collections::HashMap;

register_problem!(310, "Nim Square", problem310);

/// Alice and Bob play the game Nim Square.
/// Nim Square is just like ordinary three-heap normal play Nim, but the players may only remove a square number of
/// stones from a heap. The number of stones in the three heaps is represented by the ordered triple (a,b,c).
/// If 0≤a≤b≤c≤29 then the number of losing positions for the next player is 1160.
///
/// Find the number of losing positions for the next player if 0≤a≤b≤c≤100000.
pub fn problem310() -> String {
    let piles = 3;
    let taille: u64 = 100_000;

    // nombre de Grundy de chaque tas : mex des valeurs atteignables en retirant un carré
    let mut etats = vec![0u64; taille as usize + 1];
    for i in 0..=taille as usize {
        let mut j = 0;
        loop {
            let mut reachable = false;
            let mut k = 1;
            while k * k <= i {
                if etats[i - k * k] == j {
                    reachable = true;
                }
                k += 1;
            }

            if !reachable {
                break;
            }
            j += 1;
        }
        etats[i] = j;
    }

    let mut compteurs: HashMap<u64, u64> = HashMap::new();
    for &e in &etats {
        *compteurs.entry(e).or_insert(0) += 1;
    }

    // nombre de triplets (ordonnés) de tas dont le XOR des nombres de Grundy est k
    let mut total: HashMap<u64, u64> = HashMap::from([(0, 1)]);
    for _ in 0..piles {
        let mut nouveau_total: HashMap<u64, u64> = HashMap::new();
        for (&k1, &v1) in &total {
            for (&k2, &v2) in &compteurs {
                *nouveau_total.entry(k1 ^ k2).or_insert(0) += v1 * v2;
            }
        }
        total = nouveau_total;
    }

    let mut resultat = total.get(&0).copied().unwrap_or(0);
    if let Some(&c0) = compteurs.get(&0) {
        resultat += (3 * taille + 5) * c0;
    }

    (resultat / 6).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem310() {
        assert_eq!(problem310(), "2586528661783");
    }
}
