use crate::register_problem;

register_problem!(
    197,
    "Investigating the behaviour of a recursively defined sequence",
    problem197
);

fn f(x: f64) -> f64 {
    f64::powf(2., 30.403243784 - x * x).floor() * 0.000000001
}

pub fn problem197() -> String {
    // Given is the function f(x) = ⌊2^(30.403243784-x²)⌋ × 10-9 ( ⌊ ⌋ is the floor-function), the sequence un is
    // defined by u_0 = -1 and u_n+1 = f(u_n).
    //
    // Find u_n + u_n+1 for n = 1012.
    // Give your answer with 9 digits after the decimal point.
    let mut u_n = -1.;
    let mut u_n1 = f(u_n);

    let result;
    loop {
        let u_n2 = f(u_n1);
        if (u_n2 - u_n).abs() < f64::EPSILON {
            result = u_n + u_n1;
            break;
        }

        u_n = u_n1;
        u_n1 = u_n2;
    }

    format!("{:.9}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem197() {
        let result = problem197();
        assert_eq!(result, "1.710637717");
    }
}
