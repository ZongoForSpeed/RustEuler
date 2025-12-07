use crate::maths::arithmetique::somme_diviseurs;
use crate::maths::premiers;
use crate::maths::timer::ScopeTimer;

pub fn problem021() -> u64 {
    let _timer = ScopeTimer::new("Problem 21 Amicable numbers", false);
    // Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
    // If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.
    //
    // For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
    //
    // Evaluate the sum of all the amicable numbers under 10000.
    let mut premiers: Vec<u64> = Vec::new();
    premiers::crible2(10000, |p| premiers.push(p));
    let mut diviseurs: Vec<u64> = Vec::new();
    diviseurs.reserve(10000);
    diviseurs.push(0);
    let mut resultat:u64 = 0;
    for n in 1..10000 {
        let d = somme_diviseurs(n, &premiers) - n;
        diviseurs.push(d);
        if d < n && diviseurs[d as usize] == n {
            resultat += d + n;
        }
    }
    resultat
}
