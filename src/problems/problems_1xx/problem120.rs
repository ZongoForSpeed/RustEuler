use crate::register_problem;

register_problem!(120, "Square remainders", problem120);

pub fn problem120() -> String {
    // Let r be the remainder when (a−1)^n + (a+1)^n is divided by a².
    //
    // For example, if a = 7 and n = 3, then r = 42: 63 + 83 = 728 ≡ 42 mod 49. And as n varies, so too will r, but
    // for a = 7 it turns out that rmax = 42.
    //
    // For 3 ≤ a ≤ 1000, find ∑ rmax.
    let mut result: u64 = 0;
    for a in 3..=1000 {
        result += 2 * a * ((a * a - 1) / (2 * a));
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem120() {
        let result = problem120();
        assert_eq!(result, "333082500");
    }
}
