use crate::register_problem;
use std::collections::BTreeSet;

register_problem!(300, "Protein folding", problem300);

type Point = (i8, i8);

fn generer(chemin: &mut Vec<Point>, profondeur: usize, resultat: &mut Vec<Vec<Point>>) {
    if chemin.len() == profondeur {
        resultat.push(chemin.clone());
        return;
    }

    let (x, y) = *chemin.last().unwrap();
    for suivant in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
        if !chemin.contains(&suivant) {
            chemin.push(suivant);
            generer(chemin, profondeur, resultat);
            chemin.pop();
        }
    }
}

fn voisin(p1: Point, p2: Point) -> bool {
    if p1.0 == p2.0 {
        p1.1 == p2.1 + 1 || p1.1 + 1 == p2.1
    } else if p1.1 == p2.1 {
        p1.0 == p2.0 + 1 || p1.0 + 1 == p2.0
    } else {
        false
    }
}

/// In a very simplified form, we can consider proteins as strings consisting of hydrophobic (H) and polar (P)
/// elements, e.g. HHPPHHHPHHPH.
/// For this problem, the orientation of a protein is important; e.g. HPP is considered distinct from PPH. Thus,
/// there are 2n distinct proteins consisting of n elements.
///
/// When one encounters these strings in nature, they are always folded in such a way that the number of H-H contact
/// points is as large as possible, since this is energetically advantageous.
/// As a result, the H-elements tend to accumulate in the inner part, with the P-elements on the outside.
/// Natural proteins are folded in three dimensions of course, but we will only consider protein folding in two
/// dimensions.
///
/// The figure below shows two possible ways that our example protein could be folded (H-H contact points are shown
/// with red dots).
///
/// p300_protein.gif
/// The folding on the left has only six H-H contact points, thus it would never occur naturally.
/// On the other hand, the folding on the right has nine H-H contact points, which is optimal for this string.
///
/// Assuming that H and P elements are equally likely to occur in any position along the string, the average number
/// of H-H contact points in an optimal folding of a random protein string of length 8 turns out to be
/// 850 / 2**8=3.3203125.
///
/// What is the average number of H-H contact points in an optimal folding of a random protein string of length 15?
/// Give your answer using as many decimal places as necessary for an exact result.
pub fn problem300() -> String {
    let longueur = 15;
    let nombre_proteines: u64 = 1 << longueur;

    let mut chemins = Vec::new();
    generer(&mut vec![(0, 0)], longueur, &mut chemins);

    // chaque voisinage est la liste des masques de bits des paires de positions en contact possible
    let mut voisinages: BTreeSet<Vec<u64>> = BTreeSet::new();
    for c in &chemins {
        let mut voisins = Vec::new();
        for n in 0..c.len() {
            for m in n + 1..c.len() {
                if voisin(c[n], c[m]) {
                    voisins.push((1 << n) + (1 << m));
                }
            }
        }

        voisinages.insert(voisins);
    }

    let mut somme: u64 = 0;
    for proteine in 0..nombre_proteines {
        let mut max_compteur = 0;
        for voisins in &voisinages {
            let compteur = voisins
                .iter()
                .filter(|&&masque| proteine & masque == masque)
                .count() as u64;
            max_compteur = max_compteur.max(compteur);
        }

        somme += max_compteur;
    }

    let resultat = somme as f64 / nombre_proteines as f64;

    format!("{:.13}", resultat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem300() {
        assert_eq!(problem300(), "8.0540771484375");
    }
}
