use crate::maths::power::Power;
use crate::register_problem;
use std::collections::HashMap;

register_problem!(
    169,
    "Exploring the number of different ways a number can be expressed as a sum of powers of 2",
    problem169
);

fn f(cache: &mut HashMap<u128, u64>, n: u128) -> u64 {
    if let Some(&x) = cache.get(&n) {
        return x;
    }

    // f(n) = f(n/2) + f(n/2 - 1) si n pair
    // f(n) = f((n-1)/2) si n impair
    let result ;
    if n == 0 {
        result = 1;
    } else if n % 2 == 0 {
        result = f(cache, n / 2) + f(cache, n / 2 - 1);
    } else {
        result = f(cache, n / 2);
    }

    cache.insert(n, result);
    result
}

pub fn problem169() -> String {
    // Define f(0)=1 and f(n) to be the number of different ways n can be expressed as a sum of integer powers of 2
    // using each power no more than twice.
    //
    // For example, f(10)=5 since there are five different ways to express 10:
    //
    // 1 + 1 + 8
    // 1 + 1 + 4 + 4
    // 1 + 1 + 2 + 2 + 4
    // 2 + 4 + 4
    // 2 + 8
    //
    // What is f(10^25)?
    let n = u128::power(10, 25);
    f(&mut HashMap::new(), n).to_string()
}
