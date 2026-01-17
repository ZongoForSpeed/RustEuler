use crate::maths::digits::conversion;
use crate::register_problem;
use fraction::Fraction;
use num_traits::Zero;
use std::collections::HashSet;

register_problem!(93, "Arithmetic expressions", problem093);

fn test(n: &Vec<Fraction>, mut p: &mut HashSet<u64>) {
    let len = n.len();
    if len == 1 {
        let front = n[0];
        if let Some(numer) = front.numer()
            && let Some(denom) = front.denom()
            && numer % denom == 0
        {
            p.insert(numer / denom);
        }
        return;
    }

    for i in 0..len {
        for j in i + 1..len {
            let a = n[i];
            let b = n[j];
            let mut m = n.clone();
            m.remove(j);
            m[i] = a + b;
            test(&m, &mut p);
            m[i] = a * b;
            test(&m, &mut p);
            if a >= b {
                m[i] = a - b;
                test(&m, &mut p);
            }
            if a <= b {
                m[i] = b - a;
                test(&m, &mut p);
            }
            if !a.is_zero() {
                m[i] = b / a;
                test(&m, &mut p);
            }
            if !b.is_zero() {
                m[i] = a / b;
                test(&m, &mut p);
            }
        }
    }
}

pub fn problem093() -> String {
    // By using each of the digits from the set, {1, 2, 3, 4}, exactly once, and making use of the four arithmetic
    // operations (+, −, *, /) and brackets/parentheses, it is possible to form different positive integer targets.
    //
    // For example,
    //
    //      8 = (4 * (1 + 3)) / 2
    //      14 = 4 * (3 + 1 / 2)
    //      19 = 4 * (2 + 3) − 1
    //      36 = 3 * 4 * (2 + 1)
    //
    // Note that concatenations of the digits, like 12 + 34, are not allowed.
    //
    // Using the set, {1, 2, 3, 4}, it is possible to obtain thirty-one different target numbers of which 36 is the
    // maximum, and each of the numbers 1 to 28 can be obtained before encountering the first non-expressible number.
    //
    // Find the set of four distinct digits, a < b &lt c < d, for which the longest set of consecutive positive integers,
    // 1 to n, can be obtained, giving your answer as a string: abcd.
    let iteration = HashSet::from_iter(1..10000);
    let mut result = Vec::new();
    let mut maximum: u64 = 0;
    for a in 0..10 {
        for b in a + 1..10 {
            for c in b + 1..10 {
                for d in c + 1..10 {
                    let v: Vec<Fraction> = vec![
                        Fraction::from(a),
                        Fraction::from(b),
                        Fraction::from(c),
                        Fraction::from(d),
                    ];
                    let mut e = HashSet::new();

                    test(&v, &mut e);

                    let difference = iteration.difference(&e);
                    let min = difference.min().unwrap();
                    if min > &maximum {
                        result = vec![a, b, c, d];
                        maximum = *min;
                    }
                }
            }
        }
    }

    conversion(&result, 10).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem093() {
        let result = problem093();
        assert_eq!(result, "1258");
    }
}
