use crate::register_problem;

register_problem!(226, "A Scoop of Blancmange", problem226);

fn circle(x: f64) -> f64 {
    0.5 - (0.0625 - (x - 0.25) * (x - 0.25)).sqrt()
}

fn s(x: f64) -> f64 {
    let n = x.floor();
    if x - n < n + 1. - x {
        x - n
    } else {
        n + 1. - x
    }
}

fn blancmange(x: f64, precision: u16) -> f64 {
    let mut result = 0.;
    let mut exponent = 1.;
    for _ in 0..precision {
        result += s(exponent * x) / exponent;
        exponent *= 2.;
    }

    result
}

pub fn problem226() -> String {
    // The blancmange curve is the set of points (x,y) such that 0 ≤ x ≤ 1 and y = Sum(s(2^n.x)/2^n, n = 0..Inf),
    // where s(x) = the distance from x to the nearest integer.
    //
    // The area under the blancmange curve is equal to ½, shown in pink in the diagram below.
    //
    // https://projecteuler.net/project/images/p226_scoop2.gif
    //
    // Let C be the circle with centre (¼,½) and radius ¼, shown in black in the diagram.
    //
    // What area under the blancmange curve is enclosed by C?
    // Give your answer rounded to eight decimal places in the form 0.abcdefgh
    let mut result = 0.;
    let dx = 0.000001;

    for n in 0..500000 {
        let x = n as f64 * dx;
        let y = blancmange(x, 50) - circle(x);
        if y > 0. {
            result += y * dx;
        }
    }

    format!("{:.8}", result)
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem226::problem226;

    #[test]
    fn test_problem226() {
        let result = problem226();
        assert_eq!(result, "0.11316017");
    }
}
