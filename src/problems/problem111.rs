use crate::maths::primes::Primes;
use crate::register_problem;

register_problem!(111, "Primes with runs", problem111);

pub fn problem111() -> String {
    // Considering 4-digit primes containing repeated digits it is clear that they cannot all be the same:
    // 1111 is divisible by 11, 2222 is divisible by 22, and so on. But there are nine 4-digit primes containing three
    // ones:
    //
    //                  1117, 1151, 1171, 1181, 1511, 1811, 2111, 4111, 8111
    //
    // We shall say that M(n, d) represents the maximum number of repeated digits for an n-digit prime where d is the
    // repeated digit, N(n, d) represents the number of such primes, and S(n, d) represents the sum of these primes.
    //
    // So M(4, 1) = 3 is the maximum number of repeated digits for a 4-digit prime where one is the repeated digit,
    // there are N(4, 1) = 9 such primes, and the sum of these primes is S(4, 1) = 22275. It turns out that for d = 0,
    // it is only possible to have M(4, 0) = 2 repeated digits, but there are N(4, 0) = 13 such cases.
    //
    // In the same way we obtain the following results for 4-digit primes.
    //
    //      Digit, d    M(4, d)     N(4, d)     S(4, d)
    //      0           2           13	        67061
    //      1           3	        9	        22275
    //      2           3	        1	        2221
    //      3           3	        12	        46214
    //      4           3	        2	        8888
    //      5           3	        1	        5557
    //      6           3	        1	        6661
    //      7           3	        9       	57863
    //      8           3	        1	        8887
    //      9           3	        7           48073
    // For d = 0 to 9, the sum of all S(4, d) is 273700.
    //
    // Find the sum of all S(10, d).
    let dim = 10;
    let mask = 1111111111;
    let mut sums = vec![0; dim];

    for d in 0..10 {
        match d {
            0 => {
                for di in 1..10 {
                    for dj in 1..10 {
                        let p: u64 = di * 1000000000 + dj;
                        if p.miller_rabin(25) {
                            sums[0] += p;
                        }
                    }
                }
            }
            2 | 8 => {
                for p1 in 0..9 {
                    let power1 = 10u64.pow(p1);
                    for p2 in p1 + 1..10 {
                        let power2 = 10u64.pow(p2);
                        for dj in 0..10 {
                            if p2 == 9 && dj == 0 {
                                continue;
                            }
                            for dk in 0..10 {
                                let p = d * (mask - power1 - power2) + dk * power1 + dj * power2;
                                if p.miller_rabin(25) {
                                    sums[d as usize] += p;
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                for p1 in 0..10 {
                    let power1 = 10u64.pow(p1);
                    for dj in 0..10 {
                        let p = d * (mask - power1) + dj * power1;
                        if p.miller_rabin(25) {
                            sums[d as usize] += p;
                        }
                    }
                }
            }
        }
    }

    let result = sums.iter().sum::<u64>();
    result.to_string()
}
