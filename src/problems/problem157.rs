use crate::maths::arithmetic::Arithmetic;
use crate::maths::power::Power;
use crate::maths::primes::crible235;
use crate::register_problem;
use itertools::{iproduct, Itertools};

register_problem!(157, "Solving the diophantine equation 1/a+1/b= p/10n", problem157);

fn solve_diophantine(primes: &Vec<u64>, n: u64) -> u64 {
    let m = u64::power(10, n);
    let mut d2 = Vec::new();
    let mut d5 = Vec::new();
    for p in 0..=n {
        d2.push(u64::power(2, p));
        d5.push(u64::power(5, p));
    }

    let divisors = iproduct!(d2, d5).map(|(i, j)| i * j).unique().sorted().collect::<Vec<u64>>();

    let mut result_n = 0;
    for b in &divisors {
        for a in &divisors {
            if a > b || a * b > m {
                break
            }
            if u64::gcd(*a, *b) != 1 {
                continue;
            }

            let p = m * (a + b) / a / b;
            result_n += p.number_of_divisors(primes);
        }
    }

    println!("Solutions({n}) = {result_n}");
    result_n
}

pub fn problem157() -> String {
    // Consider the diophantine equation 1/a+1/b= p/10^n with a, b, p, n positive integers and a ≤ b.
    // For n=1 this equation has 20 solutions that are listed below:
    //
    // 1/1+1/1=20/10	1/1+1/2=15/10	1/1+1/5=12/10	1/1+1/10=11/10	1/2+1/2=10/10
    // 1/2+1/5=7/10	    1/2+1/10=6/10	1/3+1/6=5/10	1/3+1/15=4/10	1/4+1/4=5/10
    // 1/4+1/20=3/10	1/5+1/5=4/10	1/5+1/10=3/10	1/6+1/30=2/10	1/10+1/10=2/10
    // 1/11+1/110=1/10	1/12+1/60=1/10	1/14+1/35=1/10	1/15+1/30=1/10	1/20+1/20=1/10
    //
    // How many solutions has this equation for 1 ≤ n ≤ 9?
    let mut primes = Vec::new();
    crible235(100000000, |p| primes.push(p));

    let mut result = 0;
    for n in 1..=9 {
        result += solve_diophantine(&primes, n);
    }

    result.to_string()
}
