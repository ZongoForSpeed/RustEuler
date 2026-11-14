use crate::register_problem;

register_problem!(307, "Chip Defects", problem307);

/// Somme de Neumaier : compense les erreurs d'arrondi (f64 n'a pas la précision du long double du C++).
#[derive(Default)]
struct Somme {
    somme: f64,
    compensation: f64,
}

impl Somme {
    fn ajouter(&mut self, x: f64) {
        let t = self.somme + x;
        if self.somme.abs() >= x.abs() {
            self.compensation += (self.somme - t) + x;
        } else {
            self.compensation += (x - t) + self.somme;
        }
        self.somme = t;
    }

    fn valeur(&self) -> f64 {
        self.somme + self.compensation
    }
}

/// k defects are randomly distributed amongst n integrated-circuit chips produced by a factory (any number of
/// defects may be found on a chip and each defect is independent of the other defects).
///
/// Let p(k,n) represent the probability that there is a chip with at least 3 defects.
///
/// For instance p(3,7) ≈ 0.0204081633.
///
/// Find p(20 000, 1 000 000) and give your answer rounded to 10 decimal places in the form 0.abcdefghij
pub fn problem307() -> String {
    let n: u64 = 1_000_000;
    let k: u64 = 20_000;

    let total = k as f64 * (n as f64).ln();

    let mut favorable = Somme::default();
    for i in n - k + 1..n + 1 {
        favorable.ajouter((i as f64).ln());
    }

    let mut resultat = Somme::default();
    resultat.ajouter((favorable.valeur() - total).exp());
    let mut p = n - k + 1;
    let mut d: u64 = 1;
    let mut factoriel = Somme::default();
    let mut tmp = Somme::default();
    for i in (0..k).step_by(2) {
        tmp.ajouter(((k - i) as f64).ln() + ((k - i) as f64 - 1.0).ln() - 2.0f64.ln());
        favorable.ajouter(-(p as f64).ln());
        factoriel.ajouter((d as f64).ln());
        resultat.ajouter((tmp.valeur() + favorable.valeur() - factoriel.valeur() - total).exp());

        p += 1;
        d += 1;
    }

    format!("{:.10}", 1.0 - resultat.valeur())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem307() {
        assert_eq!(problem307(), "0.7311720251");
    }
}
