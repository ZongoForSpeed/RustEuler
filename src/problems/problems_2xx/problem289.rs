use crate::register_problem;
use std::collections::BTreeMap;

register_problem!(
    289,
    "Eulerian Cycles",
    problem289
);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    nb_colonnes: usize,
    nb_lignes: usize,

    nb_lignes_remplies: usize,
    position_ligne: usize,
    // identifiants des composantes connexes de chaque bloc le long du bord en croissance
    connected_id: Vec<usize>,
    // vrai si les bords extérieurs sont fusionnés
    merged: Vec<bool>,
}

impl Position {
    fn new(nb_colonnes: usize, nb_lignes: usize) -> Self {
        Self {
            nb_colonnes,
            nb_lignes,
            nb_lignes_remplies: 0,
            position_ligne: 0,
            connected_id: vec![0; nb_colonnes + 1],
            merged: vec![false; nb_colonnes],
        }
    }

    fn maximum(&self) -> usize {
        *self.connected_id.iter().max().unwrap()
    }

    fn deplacer(&self, m: u16) -> Option<Position> {
        let mut p = self.clone();
        if p.ajouter(m >= 12, (m % 4) as usize, ((m / 4) % 3) as usize) {
            Some(p)
        } else {
            None
        }
    }

    fn ajouter(&mut self, mergetopleft: bool, mergebotleft: usize, mergebotright: usize) -> bool {
        let pl = self.position_ligne;
        let nc = self.nb_colonnes;

        if pl == 0 && (mergetopleft || mergebotleft == 1 || mergebotleft == 2) {
            return false;
        }

        if mergebotleft == 1 && self.merged[pl - 1] {
            return false;
        }

        if mergebotleft == 2 && self.merged[pl] {
            return false;
        }

        if mergebotright == 2 && (pl == nc - 1 || self.merged[pl + 1]) {
            return false;
        }

        if mergetopleft && mergebotleft == 1 {
            return false;
        }

        if mergebotright == 1 && mergebotleft == 3 {
            return false;
        }

        let mut id = 0;

        // fusion avec le carré à gauche
        if mergetopleft || mergebotleft == 1 {
            if self.connected_id[pl - 1] == 0 {
                return false;
            }

            id = self.connected_id[pl - 1];
        }

        if pl > 0 {
            self.merged[pl - 1] = mergetopleft;
        }

        // fusion avec le carré en bas à gauche
        if mergebotleft == 2 {
            let voisin = self.connected_id[pl];
            if voisin == 0 || voisin == id {
                return false;
            }

            if id != 0 {
                self.merge(id, voisin);
            }

            id = self.connected_id[pl];
        }

        // fusion avec le carré en dessous
        if mergebotleft == 3 || mergebotright == 1 {
            let voisin = self.connected_id[pl + 1];
            if voisin == 0 || voisin == id {
                return false;
            }

            if id != 0 {
                self.merge(id, voisin);
            }

            id = self.connected_id[pl + 1];
        }

        self.merged[pl] = mergebotright != 0;

        // fusion avec le carré en bas à droite
        if mergebotright == 2 {
            let voisin = self.connected_id[pl + 2];
            if voisin == 0 || voisin == id {
                return false;
            }

            if id != 0 {
                self.merge(id, voisin);
            }

            if id != self.connected_id[pl + 1]
                && self.connected_id[pl + 1] != 0
                && self.count(self.connected_id[pl + 1]) == 1
            {
                return false;
            }

            id = self.connected_id[pl + 2];
            self.connected_id[pl + 1] = self.connected_id[pl + 2];

            if pl + 1 < nc {
                self.merged[pl + 1] = true;
            }
        }

        if id == 0 {
            id = 1 + self.maximum();
        }

        let old_id = self.connected_id[pl];
        self.connected_id[pl] = id;

        self.position_ligne += 1;

        // vérifie si un composant inaccessible a été créé
        if old_id != 0 && self.count(old_id) == 0 {
            return false;
        }

        self.normalise();

        // passe à la ligne suivante si la ligne est pleine
        if self.position_ligne == nc {
            if self.connected_id[nc] != 0 && self.count(self.connected_id[nc]) == 1 {
                return false;
            }

            self.nb_lignes_remplies += 1;
            self.position_ligne = 0;

            for i in (1..self.connected_id.len()).rev() {
                self.connected_id[i] = self.connected_id[i - 1];
            }

            for i in (1..self.merged.len()).rev() {
                self.merged[i] = self.merged[i - 1];
            }

            self.connected_id[0] = 0;
            self.merged[0] = false;
        }

        true
    }

    fn merge(&mut self, nouveau: usize, ancien: usize) {
        for id in self.connected_id.iter_mut() {
            if *id == ancien {
                *id = nouveau;
            }
        }
    }

    fn count(&self, id: usize) -> usize {
        self.connected_id.iter().filter(|&&i| i == id).count()
    }

    fn swap(&mut self, id1: usize, id2: usize) {
        for id in self.connected_id.iter_mut() {
            if *id == id2 {
                *id = id1;
            } else if *id == id1 {
                *id = id2;
            }
        }
    }

    fn normalise(&mut self) {
        let mut suivant = 1;
        for i in 0..self.connected_id.len() {
            if self.connected_id[i] > suivant {
                let id = self.connected_id[i];
                self.swap(id, suivant);
            }
            if self.connected_id[i] == suivant {
                suivant += 1;
            }
        }
    }
}

/// Let C(x,y) be a circle passing through the points (x, y), (x, y+1), (x+1, y) and (x+1, y+1).
///
/// For positive integers m and n, let E(m,n) be a configuration which consists of the m·n circles:
/// { C(x,y): 0 ≤ x < m, 0 ≤ y < n, x and y are integers }
///
/// An Eulerian cycle on E(m,n) is a closed path that passes through each arc exactly once.
/// Many such paths are possible on E(m,n), but we are only interested in those which are not self-crossing: A
/// non-crossing path just touches itself at lattice points, but it never crosses itself.
///
/// The image below shows E(3,3) and an example of an Eulerian non-crossing path.
///
/// p289_euler.gif
///
/// Let L(m,n) be the number of Eulerian non-crossing paths on E(m,n).
/// For example, L(1,2) = 2, L(2,2) = 37 and L(3,3) = 104290.
///
/// Find L(6,10) mod 10**10.
pub fn problem289() -> String {
    let modulo: u64 = 10_000_000_000;
    let nb_colonnes = 6;
    let nb_lignes = 10;

    let mut cache: BTreeMap<Position, u64> = BTreeMap::new();
    cache.insert(Position::new(nb_colonnes, nb_lignes), 1);

    loop {
        let (p, n) = cache.pop_first().unwrap();

        if p.nb_lignes_remplies == nb_lignes {
            cache.insert(p, n);
            break;
        }

        for m in 0..24 {
            if let Some(p2) = p.deplacer(m) {
                let compteur = cache.entry(p2).or_insert(0);
                *compteur += n;
                *compteur %= modulo;
            }
        }
    }

    let mut resultat = 0;
    for (p, &n) in &cache {
        if p.maximum() == 1 {
            resultat += n;
            resultat %= modulo;
        }
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem289() {
        assert_eq!(problem289(), "6567944538");
    }
}
