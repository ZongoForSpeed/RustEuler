use crate::register_problem;

register_problem!(9, "Special Pythagorean triplet", problem009);

pub fn problem009() -> String {
    // A Pythagorean triplet is a set of three natural numbers, a < b < c, for which, a² + b² = c²
    // For example, 3² + 4² = 9 + 16 = 25 = 5².
    //
    // There exists exactly one Pythagorean triplet for which a + b + c = 1000.
    // Find the product abc.
    let limite: u64 = 1000;
    for a in 1..limite / 3 {
        for b in a + 1..limite / 2 {
            let c = limite - a - b;
            if a * a + b * b == c * c {
                return (a * b * c).to_string();
            }
        }
    }
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem009() {
        let result = problem009();
        assert_eq!(result, "31875000");
    }
}