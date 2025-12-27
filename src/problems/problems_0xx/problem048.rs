use crate::maths::power;
use crate::register_problem;
use power::power_mod;

fn serie(limit: u128) -> u128 {
    let modulo: u128 = 10_000_000_000;
    let mut result: u128 = 0;
    for n in 1..=limit {
        result += power_mod(n, n, modulo);
        result %= modulo;
    }
    result
}

register_problem!(48, "Self powers", problem048);

pub fn problem048() -> String {
    // The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317.
    //
    // Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.
    println!("Serie 10 = {}", serie(10));
    let s1000 = serie(1000);
    println!("Serie 1000 = {}", s1000);
    s1000.to_string()
}
