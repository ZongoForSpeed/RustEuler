use itertools::iproduct;
use crate::register_problem;

register_problem!(147, "Rectangles in cross-hatched grids", problem147);

pub fn problem147() -> String {
    // In a 3x2 cross-hatched grid, a total of 37 different rectangles could be situated within that grid as indicated
    // in the sketch.
    //
    // There are 5 grids smaller than 3x2, vertical and horizontal dimensions being important, i.e. 1x1, 2x1, 3x1, 1x2
    // and 2x2. If each of them is cross-hatched, the following number of different rectangles could be situated within
    // those smaller grids:
    //
    //      1x1: 1
    //      2x1: 4
    //      3x1: 8
    //      1x2: 4
    //      2x2: 18
    //
    // Adding those to the 37 of the 3x2 grid, a total of 72 different rectangles could be situated within 3x2 and
    // smaller grids.
    //
    // How many different rectangles could be situated within 47x43 and smaller grids?
    let mut result = 0;

    for (m,n) in iproduct!(1..=43, 1..=47) {
        for (x, y) in iproduct!(1..=m, 1..=n) {
            result += (m - x + 1) * (n - y + 1);
        }
        
        for (x, y) in iproduct!(1..=2 * m, 1..=2 * m) {
            let z = (x + y - 1) / 2;
            let w = x + y - 1 - z;
            if z > m || w > m || z > n || w > n {
                continue;
            }
            if ((x | y) & 1) == 0 {
                result += (m - z) * (n - z) + (m - w) * (n - w);
            } else {
                result += (m - z) * (n - w) + (m - w) * (n - z);
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem147() {
        let result = problem147();
        assert_eq!(result, "846910284");
    }
}
