use crate::maths::polygonal::Polygonal;
use crate::register_problem;

register_problem!(85, "Counting rectangles", problem085);

pub fn problem085() -> String {
    // By counting carefully it can be seen that a rectangular grid measuring 3 by 2 contains
    // eighteen rectangles:
    //
    // Although there exists no rectangular grid that contains exactly two million rectangles, find
    // the area of the grid with the nearest solution.
    let borne = 2000000;
    let limit = 2000;

    let mut minimum = i32::MAX;
    let mut maximum = 0;

    for i in 1..=limit {
        for j in 1..=i {
            let triangle_ij = i.triangular() * j.triangular();
            let value: i32 = borne - triangle_ij;
            if value.abs() < minimum {
                minimum = value.abs();
                maximum = i * j;
            }
            if triangle_ij > borne {
                break;
            }
        }
    }
    maximum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem085() {
        let result = problem085();
        assert_eq!(result, "2772");
    }
}
