use crate::maths::arithmetic::Arithmetic;
use crate::maths::primes::crible235;
use crate::register_problem;

register_problem!(214, "Totient Chains", problem214);

pub fn problem214() -> String {
    // Let φ be Euler's totient function, i.e. for a natural number n, φ(n) is the number of k,
    // 1 ≤ k ≤ n, for which gcd(k,n) = 1.
    //
    // By iterating φ, each positive integer generates a decreasing chain of numbers ending in 1.
    // E.g. if we start with 5 the sequence 5,4,2,1 is generated.
    // Here is a listing of all chains with length 4:
    //
    //                                      5,4,2,1
    //                                      7,6,2,1
    //                                      8,4,2,1
    //                                      9,6,2,1
    //                                      10,4,2,1
    //                                      12,4,2,1
    //                                      14,6,2,1
    //                                      18,6,2,1
    //
    // Only two of these chains start with a prime, their sum is 12.
    //
    // What is the sum of all primes less than 40000000 which generate a chain of length 25?
    let limit = 40000000;
    let chain = 25;
    let mut primes: Vec<usize> = Vec::new();
    crible235(limit, |p| primes.push(p));

    let mut result = 0;
    for &p in &primes {
        if p >= limit {
            break;
        }

        let mut length = 1;
        let mut m = p;

        while m != 1 && length < chain {
            m = m.phi(&primes);
            length += 1;
        }

        if m == 1 && length == chain {
            result += p;
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem214::problem214;

    #[test]
    fn test_problem214() {
        let result = problem214();
        assert_eq!(result, "1677366278943");
    }
}
