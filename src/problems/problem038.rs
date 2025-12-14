use crate::maths::chiffres::boucle_chiffre;
use crate::maths::timer::ScopeTimer;
use std::collections::HashMap;
use std::ops::AddAssign;


pub(crate) fn pandigital(n: u64) -> bool
{
    let mut chiffres: HashMap<u64, usize> = HashMap::new();
    boucle_chiffre(n, 10, |digit| {
        chiffres.entry(digit).or_insert(0).add_assign(1);
    });

    if chiffres.contains_key(&0) {
        return false;
    }

    chiffres.values().all(|digit| *digit < 2)
}
fn algorithme(n: u64) -> u64 {
    if !pandigital(n) {
        return 0;
    }

    let mut result: u64 = n;
    let mut ss = String::new();
    ss.push_str(n.to_string().as_str());
    for m in 2..10 {
        let mn = m * n;
        ss.push_str(mn.to_string().as_str());
        let r: u64 = ss.parse().unwrap();
        if !pandigital(r) {
            return result;
        } else {
            result = r;
        }
    }
    result
}
pub fn problem038() -> u64 {
    let _timer = ScopeTimer::new("Problem 38 Pandigital multiples", false);
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
    result
}
