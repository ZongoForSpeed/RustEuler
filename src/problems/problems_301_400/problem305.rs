use crate::maths::digits::Digits;
use crate::register_problem;
use std::collections::BTreeSet;

register_problem!(305, "Reflexive Position", problem305);

fn calcul_position(n: u64) -> u64 {
    let s: String = (0..n * n).map(|i| i.to_string()).collect();
    let ns = n.to_string();

    let mut position = 0;
    for _ in 0..n {
        let debut = position + 1;
        position = debut + s[debut..].find(&ns).unwrap();
    }

    position as u64
}

/// Position (1-based) du premier chiffre de n dans S = 123456789101112…
fn d(n: u64) -> u64 {
    let mut i = 1;
    let mut t = 1;
    let mut c = 1;

    while 10 * i <= n {
        c += t * i * 9;
        i *= 10;
        t += 1;
    }

    c += t * (n - i);
    c
}

/// Toutes les chaînes de n chiffres, zéros de tête compris, dans l'ordre lexicographique.
fn enumeration(n: usize) -> Vec<String> {
    let mut resultat = vec![String::new()];
    for _ in 0..n {
        resultat = resultat
            .iter()
            .flat_map(|x| (b'0'..=b'9').map(move |y| format!("{}{}", x, y as char)))
            .collect();
    }

    resultat
}

fn f(n: u64) -> u64 {
    let ns = n.to_string();
    let mut restant = n;
    let mut i = 0;

    loop {
        let mut cc = BTreeSet::new();
        for dd in enumeration(i) {
            for j in 0..ns.len() + 1 {
                let x = format!("{}{}{}", &ns[j..], dd, &ns[..j]);
                if !x.starts_with('0') {
                    cc.insert(x.parse::<u64>().unwrap());
                }
            }

            if !dd.is_empty() && !dd.starts_with('0') {
                for j in 1..dd.len() {
                    let x = format!("{}{}{}", &dd[..j], ns, &dd[j..]);
                    cc.insert(x.parse::<u64>().unwrap());
                }
            }
        }

        for &x in &cc {
            let xx = format!("{}{}", x, x + 1);
            for j in 0..x.count_digits(10) {
                if xx.get(j..j + ns.len()) == Some(ns.as_str()) {
                    restant -= 1;
                    if restant == 0 {
                        return d(x) + j as u64;
                    }
                }
            }
        }

        i += 1;
    }
}

/// Let's call S the (infinite) string that is made by concatenating the consecutive positive integers (starting from
/// 1) written down in base 10. Thus, S = 1234567891011121314151617181920212223242...
///
/// It's easy to see that any number will show up an infinite number of times in S.
///
/// Let's call f(n) the starting position of the nth occurrence of n in S.
/// For example, f(1)=1, f(5)=81, f(12)=271 and f(7780)=111111365.
///
/// Find ∑f(3k) for 1≤k≤13.
pub fn problem305() -> String {
    let mut resultat = 0;

    let limite = 13;
    for k in 1..=limite {
        let n = 3u64.pow(k);
        let ff = if n == 243 { calcul_position(n) } else { f(n) };
        resultat += ff;
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem305() {
        assert_eq!(problem305(), "18174995535140");
    }
}
