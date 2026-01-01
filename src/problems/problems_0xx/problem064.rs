use crate::maths::integer_root;
use crate::register_problem;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use Entry::{Occupied, Vacant};
use crate::maths::polygonal::Polygonal;

fn period(r: i32) -> Option<i32> {
    if r.is_square() {
        return None;
    }

    let mut p = integer_root::isqrt(r);
    let mut q = 1;

    let mut reminders: HashMap<(i32, i32), i32> = HashMap::new();

    for n in 1.. {
        q = (r - (p * p)) / q;
        let f = (((integer_root::isqrt(r) + p) as f64) / (q as f64)).floor() as i32;
        p = -(p - (f * q));
        match reminders.entry((p, q)) {
            Vacant(e) => {
                e.insert(n);
            }
            Occupied(e) => {
                return Some(n - e.get());
            }
        }
    }

    None
}
register_problem!(64, "Odd period square roots", problem064);

pub fn problem064() -> String {
    // It can be seen that the sequence is repeating. For conciseness, we use the notation √23 = [4;(1,3,1,8)], to
    // indicate that the block (1,3,1,8) repeats indefinitely.
    //
    // The first ten continued fraction representations of (irrational) square roots are:
    //
    // √2=[1;(2)], period=1
    // √3=[1;(1,2)], period=2
    // √5=[2;(4)], period=1
    // √6=[2;(2,4)], period=2
    // √7=[2;(1,1,1,4)], period=4
    // √8=[2;(1,4)], period=2
    // √10=[3;(6)], period=1
    // √11=[3;(3,6)], period=2
    // √12= [3;(2,6)], period=2
    // √13=[3;(1,1,1,1,6)], period=5
    //
    // Exactly four continued fractions, for N ≤ 13, have an odd period.
    //
    // How many continued fractions for N ≤ 10000 have an odd period?
    let mut result = 0;
    for n in 1..=10000 {
        if period(n).is_some_and(|p| p % 2 == 1) {
            result += 1;
        }
    }
    result.to_string()
}
