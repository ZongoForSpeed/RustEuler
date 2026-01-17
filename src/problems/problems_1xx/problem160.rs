use crate::maths::power::Power;
use crate::register_problem;

register_problem!(160, "Factorial trailing digits", problem160);

pub fn problem160() -> String {
    // For any N, let f(N) be the last five digits before the trailing zeroes in N!.
    // For example,
    //
    //      9! = 362880 so f(9)=36288
    //      10! = 3628800 so f(10)=36288
    //      20! = 2432902008176640000 so f(20)=17664
    //
    // Find f(1,000,000,000,000)
    let mut limit = 1000000000000;
    let mask = 100000;

    // f(1,000,000,000,000) = f(2560000)
    while limit % 5 == 0 {
        limit /= 5;
    }
    limit *= u64::power(5, 4);

    let mut result = 1;
    let mut factor2 = 0;
    let mut factor5 = 0;
    for n in 1..=limit {
        let mut m = n;
        while m % 5 == 0 {
            factor5 += 1;
            m /= 5;
        }
        while m % 2 == 0 {
            factor2 += 1;
            m /= 2;
        }

        result = (result * m) % mask;
    }

    result *= u64::power_mod(2, factor2 - factor5, mask);
    result %= mask;

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem160() {
        let result = problem160();
        assert_eq!(result, "16576");
    }
}
