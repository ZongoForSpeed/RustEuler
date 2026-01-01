use crate::maths::polygonal::Polygonal;
use crate::register_problem;

register_problem!(94, "Almost equilateral triangles", problem094);

pub fn problem094() -> String {
    // It is easily proved that no equilateral triangle exists with integral length sides and
    // integral area. However, the almost equilateral triangle 5-5-6 has an area of 12 square units.
    //
    // We shall define an almost equilateral triangle to be a triangle for which two sides are equal
    // and the third differs by no more than one unit.
    //
    // Find the sum of the perimeters of all almost equilateral triangles with integral side lengths
    // and area and whose perimeters do not exceed one billion (1,000,000,000).
    let limit:u64 = 1000000000;
    let mut result = 0;
    for k in 1..(limit - 2) / 6 {
        let n = 3 * k * k + 2 * k;
        if n.is_square() {
            result += 6 * k + 4;
        }

        let n = 3 * k * k + 4 * k + 1;
        if n.is_square() {
            result += 6 * k + 2;
        }
    }

    result.to_string()
}
