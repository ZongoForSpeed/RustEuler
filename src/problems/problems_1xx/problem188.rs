use crate::maths::power::Power;
use crate::register_problem;

fn cycle(p: u64, m: u64) -> u64 {
    let mut n = p;
    let mut i = 1;
    while n != 1 {
        i += 1;
        n = (n * p) % m;
    }
    i
}

register_problem!(188, "The hyperexponentiation of a number", problem188);

pub fn problem188() -> String {
    // The hyperexponentiation or tetration of a number a by a positive integer b, denoted by a↑↑b or ba,
    // is recursively defined by:
    //
    // a↑↑1 = a,
    // a↑↑(k+1) = a(a↑↑k).
    //
    // Thus we have e.g. 3↑↑2 = 33 = 27, hence 3↑↑3 = 327 = 7625597484987 and 3↑↑4 is roughly
    // 10^(3.6383346400240996*10^12).
    //
    // Find the last 8 digits of 1777↑↑1855.
    let mut mask = vec![100000000];
    while let Some(&last) = mask.last()
        && last > 2
    {
        mask.push(cycle(1777, last));
    }

    let mut result = 1;
    for &m in mask.iter().rev() {
        result = u64::power_mod(1777, result, m);
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem188() {
        let result = problem188();
        assert_eq!(result, "95962097");
    }
}
