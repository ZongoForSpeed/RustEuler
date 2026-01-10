use crate::register_problem;
use std::collections::HashMap;
use std::ops::AddAssign;

register_problem!(205, "Dice Game", problem205);

type Dictionary = HashMap<u64, u64>;

fn fusion_d(d1: &Dictionary, d2: &Dictionary) -> Dictionary {
    let mut result = Dictionary::new();
    for (&k1, &v1) in d1 {
        for (&k2, &v2) in d2 {
            result.entry(k1 + k2).or_insert(0).add_assign(v1 * v2);
        }
    }
    result
}

fn fusion_v(d1: &Dictionary, d2: &Vec<u64>) -> Dictionary {
    let mut result = Dictionary::new();
    for (&k1, &v1) in d1 {
        for d in d2 {
            result.entry(k1 + d).or_insert(0).add_assign(v1);
        }
    }
    result
}

pub fn problem205() -> String {
    // A Hamming number is a positive number which has no prime factor larger than 5.
    // So the first few Hamming numbers are 1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15.
    // There are 1105 Hamming numbers not exceeding 10^8.
    //
    // We will call a positive number a generalised Hamming number of type n, if it has no prime
    // factor larger than n. Hence the Hamming numbers are the generalised Hamming numbers of type 5.
    //
    // How many generalised Hamming numbers of type 100 are there which don't exceed 10^9?
    let d4 = vec![1, 2, 3, 4];
    let d6 = vec![1, 2, 3, 4, 5, 6];

    let zero = Dictionary::from([(0, 1)]);
    let peter1 = fusion_v(&zero, &d4);
    let peter2 = fusion_d(&peter1, &peter1);
    let peter4 = fusion_d(&peter2, &peter2);
    let peter8 = fusion_d(&peter4, &peter4);
    let peter9 = fusion_d(&peter8, &peter1);

    let colin1 = fusion_v(&zero, &d6);
    let colin2 = fusion_d(&colin1, &colin1);
    let colin4 = fusion_d(&colin2, &colin2);
    let colin6 = fusion_d(&colin4, &colin2);

    let mut sum = 0;
    for (&peter, &p9) in &peter9 {
        for (&colin, &c6) in &colin6 {
            if peter > colin {
                sum += p9 * c6;
            }
        }
    }

    let result = sum as f64 * f64::powi(4., -9) * f64::powi(6., -6);
    format!("{:.7}", result)
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem205::problem205;

    #[test]
    fn test_problem205() {
        let result = problem205();
        assert_eq!(result, "0.5731441");
    }
}
