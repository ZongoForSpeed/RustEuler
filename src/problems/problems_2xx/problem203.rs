use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;
use std::collections::HashSet;
use num_traits::Zero;

register_problem!(203, "Squarefree Binomial Coefficients", problem203);

fn square_factor(n: &MpzNumber, primes: &Vec<u64>) -> bool {
    let mut copy_n = n.clone();
    let zero = MpzNumber::zero();
    for &p in primes {
        let pp = MpzNumber::from_u64(p * p);
        if &copy_n < &pp {
            return false;
        }
        if &copy_n % p == zero {
            let mut count = 0;
            while &copy_n % p == zero {
                copy_n /= p;
                count += 1;
            }
            if count > 1 {
                return true;
            }
        }
    }
    false
}

pub fn problem203() -> String {
    // The binomial coefficients nCk can be arranged in triangular form, Pascal's triangle, like this:
    //
    //                                             1
    //                                          1		1
    //                                      1		2		1
    //                                  1		3		3		1
    //                              1		4		6		4		1
    //                          1		5		10		10		5		1
    //                      1		6		15		20		15		6		1
    //                  1		7		21		35		35		21		7		1
    //                                          .........
    //
    // It can be seen that the first eight rows of Pascal's triangle contain twelve distinct numbers:
    // 1, 2, 3, 4, 5, 6, 7, 10, 15, 20, 21 and 35.
    //
    // A positive integer n is called squarefree if no square of a prime divides n. Of the twelve distinct numbers in
    // the first eight rows of Pascal's triangle, all except 4 and 20 are squarefree. The sum of the distinct squarefree
    // numbers in the first eight rows is 105.
    //
    // Find the sum of the distinct squarefree numbers in the first 51 rows of Pascal's triangle.
    let primes: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

    let limit = 50;
    let mut square_free: HashSet<MpzNumber> = HashSet::new();
    for n in 0..limit {
        for p in 0..n {
            let cnp = MpzNumber::binomial_ui(n, p);
            if !square_factor(&cnp, &primes) {
                square_free.insert(cnp);
            }
        }
    }

    square_free.into_iter().sum::<MpzNumber>().to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem203::problem203;

    #[test]
    fn test_problem203() {
        let result = problem203();
        assert_eq!(result, "34029210557338");
    }
}
