use crate::register_problem;
use std::collections::HashMap;

register_problem!(
    290,
    "Digital Signature",
    problem290
);

fn somme_chiffres(mut n: i64) -> i64 {
    let mut somme = 0;
    while n > 0 {
        somme += n % 10;
        n /= 10;
    }

    somme
}

/// How many integers 0 ≤ n < 10**18 have the property that the sum of the digits of n equals the sum of digits of 137n?
pub fn problem290() -> String {
    const TAILLE: i64 = 18;

    // clé : (différence, retenue), valeur : compteur
    let mut table: HashMap<(i64, i64), u64> = HashMap::from([((0, 0), 1)]);

    for _ in 0..TAILLE {
        let mut suivant: HashMap<(i64, i64), u64> = HashMap::new();
        for d in 0..10 {
            for (&(difference, retenue), &compteur) in &table {
                // Pour tous les nombre x ayant une difference D et une retenue R
                // alors dx a une retenue de n / 10
                //            et une difference de difference + d - n%10
                // où 137*d + retenue
                let n = 137 * d + retenue;
                *suivant.entry((difference + d - n % 10, n / 10)).or_insert(0) += compteur;
            }
        }
        table = suivant;
    }

    let mut resultat = 0;
    for (&(difference, retenue), &compteur) in &table {
        if somme_chiffres(retenue) == difference {
            resultat += compteur;
        }
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem290() {
        assert_eq!(problem290(), "20444710234716473");
    }
}
