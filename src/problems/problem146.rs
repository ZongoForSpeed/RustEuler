use crate::maths::primes::Primes;
use crate::register_problem;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

register_problem!(146, "Investigating a Prime Pattern ", problem146);

fn prime_pattern(n: u128) -> bool {
    let rem = n % 210;
    if rem == 10 || rem == 80 || rem == 130 || rem == 200 {
        let nn = n * n;
        let mut p = nn + 1;
        if !p.miller_rabin(25) {
            return false;
        }

        p = p.next_prime(25);
        if p != (nn + 3) {
            return false;
        }

        p = p.next_prime(25);
        if p != (nn + 7) {
            return false;
        }
        p = p.next_prime(25);
        if p != (nn + 9) {
            return false;
        }
        p = p.next_prime(25);
        if p != (nn + 13) {
            return false;
        }
        p = p.next_prime(25);
        if p == (nn + 27) {
            println!("Found {n} !");
            return true;
        }
    }
    false
}

pub fn problem146() -> String {
    // The smallest positive integer n for which the numbers n²+1, n²+3, n²+7, n²+9, n²+13, and n²+27 are consecutive
    // primes is 10. The sum of all such integers n below one-million is 1242490.
    //
    // What is the sum of all such integers n below 150 million?
    let range: Vec<u128> = (10..150000000).step_by(10).collect();
    range
        .into_par_iter()
        .filter(|n| prime_pattern(*n))
        .sum::<u128>()
        .to_string()
}
