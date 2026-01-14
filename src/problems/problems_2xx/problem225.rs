use crate::register_problem;

register_problem!(225, "Tribonacci non-divisors", problem225);

fn test(n: u64) -> bool {
    let mut t1 = 1;
    let mut t2 = 1;
    let mut t3 = 3;

    loop {
        let s = (t1 + t2 + t3) % n;
        t1 = t2;
        t2 = t3;
        t3 = s;

        if t1 * t2 * t3 == 0 {
            return false;
        } else if t1 * t2 * t3 == 1 {
            return true;
        }
    }
}

pub fn problem225() -> String {
    // The sequence 1, 1, 1, 3, 5, 9, 17, 31, 57, 105, 193, 355, 653, 1201 ... is defined by
    // T1 = T2 = T3 = 1 and
    // Tn = Tn-1 + Tn-2 + Tn-3.
    //
    // It can be shown that 27 does not divide any terms of this sequence.
    // In fact, 27 is the first odd number with this property.
    //
    // Find the 124th odd number that does not divide any terms of the above sequence.
    let taille = 124;
    let mut count = 0;
    for p in (27..).step_by(2) {
        if test(p) {
            count += 1;
            if count == taille {
                return p.to_string();
            }
        }
    }

    panic!("No solution found !");
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem225::problem225;

    #[test]
    fn test_problem225() {
        let result = problem225();
        assert_eq!(result, "2009");
    }
}
