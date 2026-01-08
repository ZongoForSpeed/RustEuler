use crate::register_problem;

register_problem!(193, "Squarefree Numbers", problem193);

pub fn problem193() -> String {
    // A positive integer n is called squarefree, if no square of a prime divides n, thus 1, 2, 3, 5, 6, 7, 10, 11 are
    // squarefree, but not 4, 8, 9, 12.
    //
    // How many squarefree numbers are there below 2^50?
    let limit = 1<<25;
    let mut mobius:Vec<i64> = vec![2; limit + 1];
    mobius[0] = 0;
    mobius[1] = 1;
    let max = 1<<50;
    let mut result = max;

    for x in 2..limit {
        if mobius[x] == 2 {
            mobius[x] = -1;
            for y in (2*x..limit).step_by(x) {
                mobius[y] = if mobius[y] == 2 {-1} else {-mobius[y]};
            }
            for y in (x*x..limit).step_by(x*x) {
                mobius[y] = 0;
            }
        }
        result += mobius[x] * max / (x*x) as i64;
    }

    result.to_string()
}
