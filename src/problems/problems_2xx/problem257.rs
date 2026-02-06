use crate::maths::arithmetic::Arithmetic;
use crate::register_problem;
use num_integer::Roots;

register_problem!(257, "Angular Bisectors", problem257);

/// Given is an integer sided triangle ABC with sides a ≤ b ≤ c. (AB = c, BC = a and AC = b).
/// The angular bisectors of the triangle intersect the sides at points E, F and G (see picture below).
///
/// p257_bisector.gif
/// The segments EF, EG and FG partition the triangle ABC into four smaller triangles: AEG, BFE, CGF and EFG.
/// It can be proven that for each of these four triangles the ratio area(ABC)/area(subtriangle) is rational.
/// However, there exist triangles for which some or all of these ratios are integral.
///
/// How many triangles ABC with perimeter≤100,000,000 exist so that the ratio area(ABC) / area(AEG) is integral?
pub fn problem257() -> String {
    let limit = 100_000_000;

    let mut count = limit / 3;

    for q in 1..limit.sqrt() {
        for p in (q + 1..).take_while(|&p| p * p <= 2 * q * q) {
            // Cas (a+b)*(a+c) = 2*b*c
            if u64::gcd(p, q) == 1 {
                let d = if p % 2 == 0 { 2 } else { 1 };
                let r = (p + q) * (p + 2 * q) / d;
                count += limit / r;
            }
        }

        for p in (q + 1..).take_while(|&p| p * p <= 3 * q * q) {
            // Cas (a+b)*(a+c) = 3*b*c
            if u64::gcd(p, q) == 1 {
                let d = (if p % 3 == 0 { 3 } else { 1 }) * (if (p + q) % 2 == 0 { 2 } else { 1 });
                let r = (p + q) * (p + 3 * q) / d;
                count += limit / r;
            }
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem257::problem257;

    #[test]
    fn test_problem257() {
        let result = problem257();
        assert_eq!(result, "139012411");
    }
}
