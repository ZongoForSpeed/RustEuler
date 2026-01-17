use crate::register_problem;

register_problem!(235, "Semidivisible numbers", problem235);

fn s_function(r: f64) -> f64 {
    let mut s = 0.;
    for k in 1..=5000 {
        s += (900. - 3. * k as f64) * r.powi(k - 1);
    }

    return s;
}
pub fn problem235() -> String {
    // Given is the arithmetic-geometric sequence u(k) = (900-3k)rk-1.
    // Let s(n) = Σk=1...nu(k).
    //
    // Find the value of r for which s(5000) = -600,000,000,000.
    //
    // Give your answer rounded to 12 places behind the decimal point.
    let objective = -600000000000.;
    let mut r = 1.;
    let mut dr = 0.125;

    let mut s = 0.;
    while f64::abs(s - objective) > 1. {
        s = s_function(r);
        if s > objective {
            r += dr;
        } else {
            r -= dr;
        }
        dr /= 2.;
    }

    format!("{:.12}", r)
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem235::problem235;

    #[test]
    fn test_problem235() {
        let result = problem235();
        assert_eq!(result, "1.002322108633");
    }
}
