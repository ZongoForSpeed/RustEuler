use crate::register_problem;
use num_integer::Roots;

register_problem!(198, "Ambiguous Numbers", problem198);

pub fn problem198() -> String {
    // A best approximation to a real number x for the denominator bound d is a rational number r/s
    // (in reduced form) with s ≤ d, so that any rational number p/q which is closer to x than r/s
    // has q > d.
    //
    // Usually the best approximation to a real number is uniquely determined for all denominator
    // bounds. However, there are some exceptions, e.g. 9/40 has the two best approximations 1/4 and
    // 1/5 for the denominator bound 6. We shall call a real number x ambiguous, if there is at least
    // one denominator bound for which x possesses two best approximations. Clearly, an ambiguous
    // number is necessarily rational.
    //
    // How many ambiguous numbers x = p/q, 0 < x < 1/100, are there whose denominator q does not
    // exceed 10^8?
    let limit: u64 = 100000000;
    let borne = 100;

    let m = (limit / 2).sqrt();

    let mut stack = Vec::new();
    for i in borne..m {
        stack.push(i);
    }

    let mut a = m;
    let mut result = limit / 2 - borne / 2;
    while let Some(&b) = stack.last() {
        if 2 * a * b > limit {
            a = b;
            stack.pop();
        } else {
            result += 1;
            stack.push(a + b);
        }
    }

    result.to_string()
}
