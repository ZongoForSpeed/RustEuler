use crate::maths::digits::Digits;
use crate::register_problem;
use num::complex::Complex;
use std::ops::{Add, AddAssign};

register_problem!(220, "Heighway Dragon", problem220);

#[derive(Clone, Debug)]
struct Point {
    z: Complex<i64>,
    d: Complex<i64>,
}

impl Point {
    fn zero() -> Self {
        let z = Complex::new(0, 0);
        let d = Complex::new(0, 1);
        Self { z, d }
    }

    fn left(&mut self) {
        self.d *= Complex::new(0, 1);
    }

    fn right(&mut self) {
        self.d *= Complex::new(0, -1);
    }

    fn forward(&mut self) {
        self.z += self.d;
    }
}

impl AddAssign<&Point> for Point {
    fn add_assign(&mut self, rhs: &Point) {
        self.z += rhs.z * self.d / Complex::new(0, 1);
        self.d *= rhs.d / Complex::new(0, 1);
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Point {
        let mut copy = self.clone();
        copy += rhs;
        copy
    }
}

pub fn problem220() -> String {
    // Let D0 be the two-letter string "Fa". For n≥1, derive Dn from Dn-1 by the string-rewriting rules:
    //
    //                      "a" -> "aRbFR"
    //                      "b" -> "LFaLb"
    //
    // Thus, D0 = "Fa", D1 = "FaRbFR", D2 = "FaRbFRRLFaLbFR", and so on.
    //
    // These strings can be interpreted as instructions to a computer graphics program, with "F" meaning "draw forward
    // one unit", "L" meaning "turn left 90 degrees", "R" meaning "turn right 90 degrees", and "a" and "b" being
    // ignored. The initial position of the computer cursor is (0,0), pointing up towards (0,1).
    //
    // Then Dn is an exotic drawing known as the Heighway Dragon of order n. For example, D10 is shown below; counting
    // each "F" as one step, the highlighted spot at (18,16) is the position reached after 500 steps.
    //
    // What is the position of the cursor after 10^12 steps in D50 ?
    // Give your answer in the form x,y with no spaces.
    let longueur: u64 = 1000000000000;
    let dragon = 50;

    let mut pl = Point::zero();
    pl.forward();
    pl.right();
    pl.forward();
    pl.right();

    let mut pr = Point::zero();
    pr.forward();
    pr.left();
    pr.forward();
    pr.right();

    let mut dragon_left = vec![pl];
    let mut dragon_right = vec![pr];

    for _ in 0..dragon {
        if let Some(last_left) = dragon_left.last()
            && let Some(last_right) = dragon_right.last()
        {
            let mut copy_left = last_left.clone();
            dragon_left.push(last_left + last_right);

            copy_left.left();
            copy_left.left();

            dragon_right.push(&copy_left + last_right);
        }
    }

    let c = longueur.extract_digits(2);
    let mut result = dragon_left[c.len() - 2].clone();

    for (i, &ci) in c.iter().enumerate().skip(1) {
        if ci == 1 {
            result += &dragon_left[c.len() - 2 - i];
            if c[i - 1] == 1 {
                result.left();
                result.left();
            }
        }
    }

    format!("{},{}", result.z.re, result.z.im)
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem220::problem220;

    #[test]
    fn test_problem220() {
        let result = problem220();
        assert_eq!(result, "139776,963904");
    }
}
