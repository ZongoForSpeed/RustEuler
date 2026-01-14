use crate::maths::arithmetic::Arithmetic;
use crate::maths::primes::crible235;
use crate::register_problem;

register_problem!(221, "Alexandrian Integers", problem221);

pub fn problem221() -> String {
    // We shall call a positive integer A an "Alexandrian integer", if there exist integers p, q, r such that:
    //
    //      A = p · q · r       and        1/A = 1/p + 1/q + 1/r
    //
    // For example, 630 is an Alexandrian integer (p = 5, q = −7, r = −18). In fact, 630 is the 6th Alexandrian integer,
    // the first 6 Alexandrian integers being: 6, 42, 120, 156, 420 and 630.
    //
    // Find the 150000th Alexandrian integer.
    let limite = 150000;
    let mut primes = Vec::new();
    crible235(limite, |p| primes.push(p));

    let mut alexandrian = Vec::new();
    for p in 1..(limite as u128 * 2) / 3 {
        let pp = p * p + 1;
        let divisors = pp.divisors(&primes);
        for n in 0..divisors.len() / 2 {
            let d = divisors[n];
            alexandrian.push(p * (p + d) * (p + pp / d));
        }
    }

    alexandrian.sort();
    alexandrian[limite - 1].to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem221::problem221;

    #[test]
    fn test_problem221() {
        let result = problem221();
        assert_eq!(result, "1884161251122450");
    }
}
