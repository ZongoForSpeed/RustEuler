use crate::register_problem;

register_problem!(138, "Special isosceles triangles", problem138);

fn _fibonacci(n: u64) -> (u64, u64) {
    if n == 0 {
        return (1, 0);
    }

    let (fk_1, fk) = _fibonacci(n / 2);
    if n % 2 == 0 {
        (fk * fk + fk_1 * fk_1, fk * (2 * fk_1 + fk))
    } else {
        (fk * (2 * fk_1 + fk), (fk_1 + fk) * (fk_1 + fk) + fk * fk)
    }
}

fn fibonacci(n: u64) -> u64 {
    _fibonacci(n).1
}

pub fn problem138() -> String {
    // Consider the isosceles triangle with base length, b = 16, and legs, L = 17.
    //
    // By using the Pythagorean theorem it can be seen that the height of the triangle, h = √(172 − 82) = 15, which is
    // one less than the base length.
    //
    // With b = 272 and L = 305, we get h = 273, which is one more than the base length, and this is the second smallest
    // isosceles triangle with the property that h = b ± 1.
    //
    // Find ∑ L for the twelve smallest isosceles triangles for which h = b ± 1 and b, L are positive integers.
    let mut result = 0;
    for n in 1..=12 {
        result += fibonacci(6 * n + 3) / 2;
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem138() {
        let result = problem138();
        assert_eq!(result, "1118049290473932");
    }
}
