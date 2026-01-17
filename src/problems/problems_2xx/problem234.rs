use crate::maths::primes::crible235;
use crate::register_problem;
use num_integer::Roots;

register_problem!(234, "Semidivisible numbers", problem234);

pub fn problem234() -> String {
    // For an integer n ≥ 4, we define
    // the lower prime square root of n, denoted by lps(n), as the largest prime ≤ √n
    // and the upper prime square root of n, ups(n), as the smallest prime ≥ √n.
    //
    // So, for example, lps(4) = 2 = ups(4), lps(1000) = 31, ups(1000) = 37.
    // Let us call an integer n ≥ 4 semidivisible, if one of lps(n) and ups(n) divides n, but not both.
    //
    // The sum of the semidivisible numbers not exceeding 15 is 30, the numbers are 8, 10 and 12.
    // 15 is not semidivisible because it is a multiple of both lps(15) = 3 and ups(15) = 5.
    // As a further example, the sum of the 92 semidivisible numbers up to 1000 is 34825.
    //
    // What is the sum of all semidivisible numbers not exceeding 999966663333 ?
    let limit = 999966663333;
    let borne = limit.sqrt() + 1;

    let mut premiers: Vec<usize> = Vec::new();
    crible235(borne, |p| premiers.push(p));

    let mut resultat = 0;
    for w in premiers.windows(2) {
        let p1 = w[0];
        let p2 = w[1];
        for n in (p1 * p1 + p1..std::cmp::min(p2 * p2, limit)).step_by(p1) {
            if n % p2 != 0 {
                resultat += n;
            }
        }
        for n in ((p1 * p1) + 1..=p2 * p2 - p2).rev().step_by(p2) {
            if n < limit && n % p1 != 0 {
                resultat += n;
            }
        }
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem234::problem234;

    #[test]
    fn test_problem234() {
        let result = problem234();
        assert_eq!(result, "1259187438574927161");
    }
}
