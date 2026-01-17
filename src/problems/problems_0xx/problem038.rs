use crate::maths::digits::Digits;
use crate::register_problem;

fn algorithme(n: u64) -> u64 {
    if !n.pandigital(10) {
        return 0;
    }

    let mut result: u64 = n;
    let mut ss = String::new();
    ss.push_str(n.to_string().as_str());
    for m in 2..10 {
        let mn = m * n;
        ss.push_str(mn.to_string().as_str());
        let r: u64 = ss.parse().unwrap();
        if !r.pandigital(10) {
            return result;
        } else {
            result = r;
        }
    }
    result
}
register_problem!(38, "Pandigital multiples", problem038);

pub fn problem038() -> String {
    // Take the number 192 and multiply it by each of 1, 2, and 3:
    //
    //    192 × 1 = 192
    //    192 × 2 = 384
    //    192 × 3 = 576
    //
    // By concatenating each product we get the 1 to 9 pandigital, 192384576.
    // We will call 192384576 the concatenated product of 192 and (1,2,3)
    //
    // The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5, giving the pandigital,
    // 918273645, which is the concatenated product of 9 and (1,2,3,4,5).
    //
    // What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated product of an integer
    // with (1,2, ... , n) where n > 1?
    let mut result: u64 = 0;
    for i in 1..100000 {
        if i % 5 != 0 {
            result = std::cmp::max(result, algorithme(i));
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem038() {
        let result = problem038();
        assert_eq!(result, "932718654");
    }
}