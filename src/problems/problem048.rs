use puissance::puissance_m;
use crate::maths::puissance;
use crate::maths::timer::ScopeTimer;

fn serie(limit: u128) -> u128 {
    let modulo: u128 = 10_000_000_000;
    let mut result: u128 = 0;
    for n in 1..=limit {
        result += puissance_m(n, n, modulo);
        result %= modulo;
    }
    result
}

pub fn problem048() -> u128 {
    let _timer = ScopeTimer::new("Problem 48 Self powers", false);
    // The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317.
    //
    // Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.
    println!("Serie 10 = {}", serie(10));
    let s1000 = serie(1000);
    println!("Serie 1000 = {}", s1000);
    s1000
}
