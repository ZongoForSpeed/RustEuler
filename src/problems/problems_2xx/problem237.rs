use crate::maths::matrix::Matrix;
use crate::register_problem;
use ndarray::{arr1, arr2};

register_problem!(237, "Tours on a 4 x n playing board", problem237);

pub fn problem237() -> String {
    // Let T(n) be the number of tours over a 4 × n playing board such that:
    //
    // The tour starts in the top left corner.
    // The tour consists of moves that are up, down, left, or right one square.
    // The tour visits each square exactly once.
    // The tour ends in the bottom left corner.
    // The diagram shows one tour over a 4 × 10 board:
    //
    //					https://projecteuler.net/project/images/p237.gif
    //
    //T(10) is 2329. What is T(1012) modulo 108?
    let n = 1000000000000;

    // T[n]=2*T[n-1]+2*T[n-2]-2*T[n-3]+T[n-4]
    let m = arr2(&[[2, 2, -2, 1],
        [1, 0, 0, 0],
        [0, 1, 0, 0],
        [0, 0, 1, 0],
    ]);

    let i = arr1(&[8, 4, 1, 1]);
    let m_n = i64::power_matrix_mod(&m, n - 4, 100000000);
    
    m_n.dot(&i)[0].to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem237::problem237;

    #[test]
    fn test_problem237() {
        let result = problem237();
        assert_eq!(result, "15836928");
    }
}
