use crate::register_problem;

register_problem!(
    275,
    "Balanced Sculptures",
    problem275
);

type Point = (i64, i64);

/// Grille de booléens indexée par (x, y) avec x dans [-taille, taille] et y dans [0, taille].
/// L'ordre des index (x majeur, y mineur) correspond à l'ordre lexicographique des points.
struct Grille {
    cellules: Vec<bool>,
    occupees: Vec<Point>,
    largeur: usize,
    hauteur: usize,
    decalage: i64,
}

impl Grille {
    fn new(taille: i64) -> Self {
        let largeur = (2 * taille + 1) as usize;
        let hauteur = (taille + 1) as usize;
        Self {
            cellules: vec![false; largeur * hauteur],
            occupees: Vec::new(),
            largeur,
            hauteur,
            decalage: taille,
        }
    }

    fn index(&self, (x, y): Point) -> Option<usize> {
        let xi = x + self.decalage;
        if xi < 0 || xi >= self.largeur as i64 || y < 0 || y >= self.hauteur as i64 {
            return None;
        }

        Some(xi as usize * self.hauteur + y as usize)
    }

    fn contains(&self, p: &Point) -> bool {
        self.index(*p).is_some_and(|i| self.cellules[i])
    }

    fn insert(&mut self, p: Point) {
        let i = self.index(p).expect("point hors de la grille");
        if !self.cellules[i] {
            self.cellules[i] = true;
            self.occupees.push(p);
        }
    }

    fn remove(&mut self, p: &Point) {
        if let Some(i) = self.index(*p)
            && self.cellules[i]
        {
            self.cellules[i] = false;
            let position = self.occupees.iter().position(|q| q == p).unwrap();
            self.occupees.swap_remove(position);
        }
    }

    fn premier(&self) -> Option<Point> {
        self.occupees.iter().min().copied()
    }
}

fn symetrique(p: &Grille) -> bool {
    p.occupees.iter().all(|&(x, y)| p.contains(&(-x, y)))
}

fn somme_x(p: &Grille) -> i64 {
    p.occupees.iter().map(|&(x, _)| x).sum()
}

fn algorithme(
    n: i64,
    p: &mut Grille,
    f: &mut Grille,
    r: &mut Grille,
    x: i64,
    minimum_x: i64,
    maximum_x: i64,
) -> i64 {
    let min_change = n * (2 * minimum_x - n - 1) / 2;
    let max_change = n * (2 * maximum_x + n + 1) / 2;

    if x + min_change > 0 || x + max_change < 0 {
        return 0;
    }

    if n == 0 {
        if somme_x(p) != 0 {
            return 0;
        }
        if symetrique(p) {
            return 2;
        }

        return 1;
    }

    let Some(f0) = f.premier() else {
        return 0;
    };
    f.remove(&f0);
    r.insert(f0);
    let mut resultat = algorithme(n, p, f, r, x, minimum_x, maximum_x);
    r.remove(&f0);
    f.insert(f0);

    if p.contains(&f0) {
        return resultat;
    }

    p.insert(f0);
    f.remove(&f0);

    const DP: [Point; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut nouveaux = Vec::new();
    for (dx, dy) in DP {
        let next = (f0.0 + dx, f0.1 + dy);
        if next.1 <= 0 || f.contains(&next) || r.contains(&next) || p.contains(&next) {
            continue;
        }

        nouveaux.push(next);
        f.insert(next);
    }

    let x = x + f0.0;
    let minimum_x = minimum_x.min(f0.0);
    let maximum_x = maximum_x.max(f0.0);

    resultat += algorithme(n - 1, p, f, r, x, minimum_x, maximum_x);
    p.remove(&f0);
    for next in &nouveaux {
        f.remove(next);
    }
    f.insert(f0);

    resultat
}

/// Let us define a balanced sculpture of order n as follows:
///
///    . A polyomino made up of n+1 tiles known as the blocks (n tiles) and the plinth (remaining tile);
///    . the plinth has its centre at position (x = 0, y = 0);
///    . the blocks have y-coordinates greater than zero (so the plinth is the unique lowest tile);
///    . the centre of mass of all the blocks, combined, has x-coordinate equal to zero.
///
/// When counting the sculptures, any arrangements which are simply reflections about the y-axis, are not counted as
/// distinct. For example, the 18 balanced sculptures of order 6 are shown below; note that each pair of mirror
/// images (about the y-axis) is counted as one sculpture:
///
/// p275_sculptures2.gif
///
/// There are 964 balanced sculptures of order 10 and 360505 of order 15.
/// How many balanced sculptures are there of order 18?
pub fn problem275() -> String {
    let n = 18;

    // Une cellule posée est à distance au plus n+1 de l'origine, en x comme en y.
    let mut p = Grille::new(n + 1);
    let mut f = Grille::new(n + 1);
    let mut r = Grille::new(n + 1);
    p.insert((0, 0));
    f.insert((0, 1));

    let resultat = algorithme(n, &mut p, &mut f, &mut r, 0, 0, 0) / 2;

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem275() {
        assert_eq!(problem275(), "15030564");
    }
}
