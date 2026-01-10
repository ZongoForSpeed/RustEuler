use crate::register_problem;
use num_integer::Roots;

register_problem!(210, "Obtuse Angled Triangles", problem210);

pub fn problem210() -> String {
    // Consider the set S(r) of points (x,y) with integer coordinates satisfying |x| + |y| ≤ r.
    //
    // Let O be the point (0,0) and C the point (r/4,r/4).
    // Let N(r) be the number of points B in S(r), so that the triangle OBC has an obtuse angle,
    // i.e. the largest angle α satisfies 90°<α<180°.
    //
    // So, for example, N(4)=24 and N(8)=100.
    // What is N(1,000,000,000)?
    let r: i64 = 1000000000;

    let k = r / 4;
    let r_square = k * k / 2;

    let mut count = 0;

    let mut x = k / 2;
    let mut y = k / 2;
    while x > 0 {
        let x_square = x * x;
        let y_square = (y + 1) * (y + 1);
        if x_square + y_square < r_square {
            y += 1;
        }

        count += y - x;
        x -= 1;
    }

    let result = 24 * k * k + 8 * count + 4 * r_square.sqrt() + (k - 2);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem210::problem210;

    #[test]
    fn test_problem210() {
        let result = problem210();
        assert_eq!(result, "1598174770174689458");
    }
}
