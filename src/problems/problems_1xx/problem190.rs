use crate::register_problem;

register_problem!(190, "Maximising a weighted product", problem190);

pub fn problem190() -> String {
    // Let Sm = (x1, x2, ... , xm) be the m-tuple of positive real numbers with x1 + x2 + ... + xm = m for which
    // Pm = x1 * x2² * ... * xm^m is maximised.
    //
    // For example, it can be verified that [P10] = 4112 ([ ] is the integer part function).
    //
    // Find Σ[Pm] for 2 ≤ m ≤ 15.
    let mut result = 0;
    for m in 2..=15 {
        let mut product = 1.;
        for i in 1..=m {
            product *= ((2. * i as f64) / (m as f64 + 1.)).powi(i);
        }
        result += product.floor() as u64;
    }

    result.to_string()
}
