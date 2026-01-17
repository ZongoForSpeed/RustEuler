use crate::maths::polygonal::Polygonal;
use crate::maths::primes::Primes;
use crate::register_problem;

register_problem!(196, "Prime triplets", problem196);

fn triangle(i: u128, j: u128) -> u128 {
    if i == 0 || j == 0 {
        return 0;
    }

    if i == 1 && j == 1 {
        return 1;
    }

    if j > i {
        return 0;
    }

    u128::triangular(i - 1) + j
}

fn test(i: u128, j: u128) -> bool {
    let t_ij = triangle(i, j);
    if t_ij % 2 == 0 {
        return false;
    }

    u128::miller_rabin(t_ij, 5)
}

fn prime_triplet(i: u128, j: u128, recursive: bool) -> bool {
    let mut p = Vec::new();
    let mut count = 0;

    if test(i + 1, j - 1) {
        count += 1;
        p.push((i + 1, j - 1));
    }

    if test(i + 1, j) {
        count += 1;
        p.push((i + 1, j));
        if count > 1 {
            return true;
        }
    }

    if test(i + 1, j + 1) {
        count += 1;
        p.push((i + 1, j + 1));
        if count > 1 {
            return true;
        }
    }
    if test(i - 1, j - 1) {
        count += 1;
        p.push((i - 1, j - 1));
        if count > 1 {
            return true;
        }
    }
    if test(i - 1, j) {
        count += 1;
        p.push((i - 1, j));
        if count > 1 {
            return true;
        }
    }
    if test(i - 1, j + 1) {
        count += 1;
        p.push((i - 1, j + 1));
        if count > 1 {
            return true;
        }
    }

    if recursive
        && count == 1
        && let Some(&front) = p.first()
    {
        return prime_triplet(front.0, front.1, false);
    }

    false
}

fn s(line: u128) -> u128 {
    let min = u128::triangular(line - 1);
    let max = u128::triangular(line);

    // std::set<nombre> premiers;
    let mut result = 0;

    let mut prime = u128::next_prime(min, 5);
    while prime <= max {
        if prime_triplet(line, prime - min, true) {
            result += prime;
        }

        prime = u128::next_prime(prime, 5);
    }

    result
}

pub fn problem196() -> String {
    // Build a triangle from all positive integers in the following way:
    //
    //        1
    //        2  3
    //        4  5  6
    //        7  8  9 10
    //       11 12 13 14 15
    //       16 17 18 19 20 21
    //       22 23 24 25 26 27 28
    //       29 30 31 32 33 34 35 36
    //       37 38 39 40 41 42 43 44 45
    //       46 47 48 49 50 51 52 53 54 55
    //       56 57 58 59 60 61 62 63 64 65 66
    //       . . .
    //
    // Each positive integer has up to eight neighbours in the triangle.
    //
    // A set of three primes is called a prime triplet if one of the three primes has the other two as neighbours in the
    // triangle.
    //
    // For example, in the second row, the prime numbers 2 and 3 are elements of some prime triplet.
    //
    // If row 8 is considered, it contains two primes which are elements of some prime triplet, i.e. 29 and 31.
    // If row 9 is considered, it contains only one prime which is an element of some prime triplet: 37.
    //
    // Define S(n) as the sum of the primes in row n which are elements of any prime triplet. Then S(8)=60 and S(9)=37.
    //
    // You are given that S(10000)=950007619.
    //
    // Find  S(5678027) + S(7208785).
    println!("S(10000) = {}", s(10000));
    let result = s(5678027) + s(7208785);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem196() {
        let result = problem196();
        assert_eq!(result, "322303240771079935");
    }
}
