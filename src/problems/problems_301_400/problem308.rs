use crate::maths::arithmetic::Arithmetic;
use crate::maths::primes::crible2;
use crate::register_problem;

register_problem!(308, "An amazing Prime-generating Automaton", problem308);

fn conway_premier(n: u64, premiers: &[u64]) -> u64 {
    let diviseurs = n.divisors(premiers);
    let d = diviseurs[diviseurs.len() - 2];

    let mut resultat = n + d - 2 + (6 * n + 2) * (n - d);
    for k in d..n {
        resultat += 2 * (n / k);
    }

    resultat
}

/// A program written in the programming language Fractran consists of a list of fractions.
///
/// The internal state of the Fractran Virtual Machine is a positive integer, which is initially set to a seed value.
/// Each iteration of a Fractran program multiplies the state integer by the first fraction in the list which will
/// leave it an integer.
///
/// For example, one of the Fractran programs that John Horton Conway wrote for prime-generation consists of the
/// following 14 fractions:
///
/// 17/91, 78/85, 19/51, 23/38, 29/33, 77/29, 95/23,
/// 77/19, 1/17, 11/13, 13/11, 15/2, 1/7, 55/1.
///
/// Starting with the seed integer 2, successive iterations of the program produce the sequence:
/// 15, 825, 725, 1925, 2275, 425, ..., 68, 4, 30, ..., 136, 8, 60, ..., 544, 32, 240, ...
///
/// The powers of 2 that appear in this sequence are 2**2, 2**3, 2**5, ...
///
/// It can be shown that all the powers of 2 in this sequence have prime exponents and that all the primes appear as
/// exponents of powers of 2, in proper order!
///
/// If someone uses the above Fractran program to solve Project Euler Problem 7 (find the 10001st prime), how many
/// iterations would be needed until the program produces 2**10001st prime ?
pub fn problem308() -> String {
    let p: u64 = 104_743;
    let mut premiers = Vec::new();
    crible2::<u64, _>(p as usize, |q| premiers.push(q));

    let mut resultat = 0;
    for n in 2..p + 1 {
        resultat += conway_premier(n, &premiers);
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem308() {
        assert_eq!(problem308(), "1539669807660924");
    }
}
