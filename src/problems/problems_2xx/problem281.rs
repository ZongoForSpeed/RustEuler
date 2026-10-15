use crate::maths::arithmetic::Arithmetic;
use crate::maths::primes::crible235;
use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;

register_problem!(
    281,
    "Pizza Toppings",
    problem281
);

fn combinaison(m: u64, n: u64) -> MpzNumber {
    let factorielle_n = MpzNumber::factorial(n);
    let mut puissance = MpzNumber::from_u64(1);
    for _ in 0..m {
        puissance *= &factorielle_n;
    }

    MpzNumber::factorial(m * n) / puissance
}

fn f(m: u64, n: u64, premiers: &[u64]) -> MpzNumber {
    let mut somme = MpzNumber::from_u64(0);
    for d in 1..n + 1 {
        if n % d == 0 {
            let terme = combinaison(m, d) * (n / d).phi(premiers);
            somme += &terme;
        }
    }

    somme / (m * n)
}

/// You are given a pizza (perfect circle) that has been cut into m·n equal pieces and you want to have exactly one
/// topping on each slice.
///
/// Let f(m,n) denote the number of ways you can have toppings on the pizza with m different toppings (m ≥ 2), using
/// each topping on exactly n slices (n ≥ 1).
/// Reflections are considered distinct, rotations are not.
///
/// Thus, for instance, f(2,1) = 1, f(2,2) = f(3,1) = 2 and f(3,2) = 16. f(3,2) is shown below:
///
/// p281_pizza.gif
/// Find the sum of all f(m,n) such that f(m,n) ≤ 10**15.
pub fn problem281() -> String {
    let mut premiers = Vec::new();
    crible235::<u64, _>(1000, |p| premiers.push(p));

    let limite = MpzNumber::from_u64(1_000_000_000_000_000);
    let mut resultat = MpzNumber::from_u64(0);

    let mut m = 2;
    while f(m, 1, &premiers) <= limite {
        let mut n = 1;
        while f(m, n, &premiers) <= limite {
            resultat += &f(m, n, &premiers);
            n += 1;
        }
        m += 1;
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem281() {
        assert_eq!(problem281(), "1485776387445623");
    }
}
