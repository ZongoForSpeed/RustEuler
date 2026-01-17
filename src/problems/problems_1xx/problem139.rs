use crate::maths::pythagorean::pythagorean_limit;
use crate::register_problem;

register_problem!(139, "Pythagorean tiles", problem139);

pub fn problem139() -> String {
    // Let (a, b, c) represent the three sides of a right angle triangle with integral length sides.
    // It is possible to place four such triangles together to form a square with length c.
    //
    // For example, (3, 4, 5) triangles can be placed together to form a 5 by 5 square with a 1 by 1 hole in the middle
    // and it can be seen that the 5 by 5 square can be tiled with twenty-five 1 by 1 squares.
    //
    // However, if (5, 12, 13) triangles were used then the hole would measure 7 by 7 and these could not be used to
    // tile the 13 by 13 square.
    //
    // Given that the perimeter of the right triangle is less than one-hundred million, how many Pythagorean triangles
    // would allow such a tiling to take place?
    let limit: u64 = 100000000;

    let mut result = 0;
    for (a, b, c) in pythagorean_limit(limit) {
        let abc = a + b + c;
        let ba = b - a;
        if abc < limit && c % ba == 0 {
            result += limit / abc;
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem139() {
        let result = problem139();
        assert_eq!(result, "10057761");
    }
}
