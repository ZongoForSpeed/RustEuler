use crate::register_problem;
use std::path::Path;

register_problem!(99, "Largest exponential", problem099);

pub fn problem099() -> String {
    // Comparing two numbers written in index form like 211 and 37 is not difficult, as any calculator would confirm
    // that 2^11 = 2048 < 3^7 = 2187.
    //
    // However, confirming that 632382^518061 > 519432^525806 would be much more difficult, as both numbers contain over
    // three million digits.
    //
    // Using base_exp.txt (right click and 'Save Link/Target As...'), a 22K text file containing one thousand lines with
    // a base/exponent pair on each line, determine which line number has the greatest numerical value.
    //
    // NOTE: The first two lines in the file represent the numbers in the example given above.
    let path = Path::new("data/p099_base_exp.txt");
    let pairs: Vec<(f64, f64)> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|x| x.split_once(',').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect::<Vec<(f64, f64)>>();

    println!("{:?}", pairs);

    let mut maximum = 0f64;
    let mut result = 0;
    for (n, (a, b)) in pairs.iter().enumerate() {
        let log = a.ln() * b;
        if log > maximum {
            maximum = log;
            result = n + 1;
        }
    }

    result.to_string()
}
