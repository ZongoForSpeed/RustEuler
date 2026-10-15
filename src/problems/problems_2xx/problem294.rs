use crate::register_problem;
use std::collections::HashMap;

register_problem!(
    294,
    "Sum of digits - experience #23",
    problem294
);

// clé : (somme des chiffres, reste modulo), valeur : compteur
type Dictionnaire = HashMap<(u64, u64), u64>;

fn puissance_modulaire(base: u64, exposant: u64, modulo: u64) -> u64 {
    let mut resultat = 1 % modulo;
    let mut b = base % modulo;
    let mut e = exposant;
    while e > 0 {
        if e & 1 == 1 {
            resultat = resultat * b % modulo;
        }
        b = b * b % modulo;
        e >>= 1;
    }

    resultat
}

fn algorithme(chiffres: u64, modulo: u64, masque: u64) -> Dictionnaire {
    if chiffres == 0 {
        return HashMap::from([((0, 0), 1)]);
    }

    let mut resultat = Dictionnaire::new();
    if chiffres % 2 == 0 {
        let recursion = algorithme(chiffres / 2, modulo, masque);
        let m = puissance_modulaire(10, chiffres / 2, modulo);

        for (&(somme1, reste1), &compteur1) in &recursion {
            for (&(somme2, reste2), &compteur2) in &recursion {
                let somme = somme1 + somme2;
                if somme > modulo {
                    continue;
                }

                let reste = (reste1 * m + reste2) % modulo;
                let compteur = resultat.entry((somme, reste)).or_insert(0);
                *compteur += compteur1 * compteur2;
                *compteur %= masque;
            }
        }
    } else {
        let recursion = algorithme(chiffres - 1, modulo, masque);
        for chiffre in 0..10 {
            for (&(somme1, reste1), &compteur1) in &recursion {
                let somme = somme1 + chiffre;
                if somme > modulo {
                    continue;
                }

                let reste = (reste1 * 10 + chiffre) % modulo;
                let compteur = resultat.entry((somme, reste)).or_insert(0);
                *compteur += compteur1;
                *compteur %= masque;
            }
        }
    }

    resultat
}

/// For a positive integer k, define d(k) as the sum of the digits of k in its usual decimal representation. Thus
/// d(42) = 4+2 = 6.
///
/// For a positive integer n, define S(n) as the number of positive integers k < 10**n with the following properties:
///
/// 		- k is divisible by 23 and
/// 		- d(k) = 23.
///
/// You are given that S(9) = 263626 and S(42) = 6377168878570056.
/// Find S(11**12) and give your answer mod 10**9.
pub fn problem294() -> String {
    let masque = 1_000_000_000;
    let modulo = 23;

    let n = 11u64.pow(12);
    let resultat = algorithme(n, modulo, masque).get(&(23, 0)).copied().unwrap_or(0);

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem294() {
        assert_eq!(problem294(), "789184709");
    }
}
