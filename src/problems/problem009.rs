use crate::maths::timer::ScopeTimer;

pub fn problem009(limite: u64) -> u64 {
    let _timer = ScopeTimer::new("Problem 9 Special Pythagorean triplet", true);
    // A Pythagorean triplet is a set of three natural numbers, a < b < c, for which, a² + b² = c²
    // For example, 3² + 4² = 9 + 16 = 25 = 5².
    //
    // There exists exactly one Pythagorean triplet for which a + b + c = 1000.
    // Find the product abc.
    for a in 1..limite / 3 {
        for b in a + 1..limite / 2 {
            let c = limite - a - b;
            if a * a + b * b == c * c {
                println!("solution 009: {}", a * b * c);
                return a * b * c;
            }
        }
    }
    0
}
