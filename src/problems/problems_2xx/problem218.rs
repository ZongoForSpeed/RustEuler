use crate::maths::power::Power;
use crate::maths::pythagorean::pythagorean_limit;
use crate::register_problem;

register_problem!(218, "Perfect right-angled triangles", problem218);

pub fn problem218() -> String {
    // Consider the right angled triangle with sides a=7, b=24 and c=25. The area of this triangle is 84, which is
    // divisible by the perfect numbers 6 and 28.
    // Moreover it is a primitive right angled triangle as gcd(a,b)=1 and gcd(b,c)=1.
    // Also c is a perfect square.
    //
    // We will call a right angled triangle perfect if
    // -it is a primitive right angled triangle
    // -its hypotenuse is a perfect square
    //
    // We will call a right angled triangle super-perfect if
    // -it is a perfect right angled triangle and
    // -its area is a multiple of the perfect numbers 6 and 28.
    //
    // How many perfect right-angled triangles with c ≤ 10^16 exist that are not super-perfect?
    let limit = u128::power(10, 8);

    let mut result = 0;
    for (x, y, _) in pythagorean_limit(limit) {
        let a = y * y - x * x;
        let b = 2 * x * y;

        let area = a * b / 2;
        if area % 6 != 0 || area % 28 != 0 {
            result += 1;
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem218::problem218;

    #[test]
    fn test_problem218() {
        let result = problem218();
        assert_eq!(result, "0");
    }
}
