use crate::maths::puissance;
use crate::maths::timer::ScopeTimer;

pub fn problem040() -> u32 {
    let _timer = ScopeTimer::new("Problem 40 Champernowne's constant", false);
    // An irrational decimal fraction is created by concatenating the positive integers:
    //
    //                  0.123456789101112131415161718192021...
    //
    // It can be seen that the 12th digit of the fractional part is 1.
    //
    // If dn represents the nth digit of the fractional part, find the value of the following expression.
    //
    //                      d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000
    let mut builder = string_builder::Builder::new(1000000);
    for n in 0..1000000 {
        builder.append(n.to_string())
    }
    let s: String = builder.string().unwrap();

    let mut result = 1;
    for p in 0..7 {
        let pos: usize = puissance::puissance(10, p);
        result *= s.chars().nth(pos).unwrap().to_digit(10).unwrap();
    }
    result
}
