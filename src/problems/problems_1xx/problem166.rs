use crate::register_problem;
use itertools::iproduct;

register_problem!(166, "Criss Cross", problem166);

pub fn problem166() -> String {
    // A 4x4 grid is filled with digits d, 0 ≤ d ≤ 9.
    //
    // It can be seen that in the grid
    //
    //                                       6 3 3 0
    //                                       5 0 4 3
    //                                       0 7 1 4
    //                                       1 2 4 5
    //
    // the sum of each row and each column has the value 12. Moreover the sum of each diagonal is
    // also 12.
    //
    // In how many ways can you fill a 4x4 grid with the digits d, 0 ≤ d ≤ 9 so that each row, each
    // column, and both diagonals have the same sum?
    let mut result = 0;

    for (a11, a12, a13, a14) in iproduct!(0..10, 0..10, 0..10, 0..10) {
        let d = a11 + a12 + a13 + a14;
        for (a21, a22, a23) in iproduct!(0..10, 0..10, 0..10) {
            let a24 = d - (a21 + a22 + a23);
            if a24 < 10 && a24 >= 0 {
                for a31 in 0..10 {
                    let a41 = d - (a11 + a21 + a31);
                    if a41 < 10 && a41 >= 0 {
                        let a32 = d - (a41 + a23 + a14);
                        if a32 < 10 && a32 >= 0 && (a32 + a23 + a14) == (a11 + a21 + a31) {
                            let a42 = d - (a12 + a22 + a32);
                            if a42 < 10 && a42 >= 0 {
                                for a33 in 0..10 {
                                    if d >= (a31 + a32 + a33) && d >= (a13 + a23 + a33) {
                                        let a34 = d - (a31 + a32 + a33);
                                        let a43 = d - (a13 + a23 + a33);
                                        if a34 < 10
                                            && a34 >= 0
                                            && a43 < 10
                                            && a43 >= 0
                                            && (a41 + a42 + a43) == (a11 + a22 + a33)
                                            && (a41 + a42 + a43) == (a14 + a24 + a34)
                                        {
                                            let a44 = d - (a41 + a42 + a43);
                                            if a44 < 10 && a44 >= 0 {
                                                result += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem166() {
        let result = problem166();
        assert_eq!(result, "7130034");
    }
}
