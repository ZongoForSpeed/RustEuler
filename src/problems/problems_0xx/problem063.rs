use crate::register_problem;

register_problem!(63, "Powerful digit counts", problem063);

pub fn problem063() -> String {
    // The 5-digit number, 16807=7^5, is also a fifth power. Similarly, the 9-digit number,
    // 134217728=8^9,is a ninth power.
    //
    // How many n-digit positive integers exist which are also an nth power?
    let mut result = 0;
    let log_10 = 10f64.ln();

    for n in 1..10 {
        let log_n = (n as f64).ln();
        result += (log_10 / (log_10 - log_n)).floor() as u64;
    }
    result.to_string()
}
