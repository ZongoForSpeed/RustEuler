use crate::register_problem;

register_problem!(3, "Even Fibonacci numbers", problem003);

pub fn problem003() -> String {
    // The prime factors of 13195 are 5, 7, 13 and 29.
    //
    // What is the largest prime factor of the number 600851475143 ?
    let mut n:u64 = 600851475143; // 600851475143;
    let mut count = 2;
    while n != 1 {
        if n % count == 0 {
            n /= count;
        } else {
            count += 1;
        }
    }
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem003() {
        let result = problem003();
        assert_eq!(result, "6857");
    }
}
