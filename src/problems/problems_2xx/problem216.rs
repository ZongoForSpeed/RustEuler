use crate::register_problem;

register_problem!(
    216,
    "Investigating the primality of numbers of the form 2n²-1",
    problem216
);

pub fn problem216() -> String {
    // Consider numbers t(n) of the form t(n) = 2n²-1 with n > 1.
    // The first such numbers are 7, 17, 31, 49, 71, 97, 127 and 161.
    // It turns out that only 49 = 7*7 and 161 = 7*23 are not prime.
    // For n ≤ 10000 there are 2202 numbers t(n) that are prime.
    //
    // How many numbers t(n) are prime for n ≤ 50,000,000 ?
    let limit = 50000000;
    let mut result = 0;

    let mut p = vec![0; limit + 1];
    for n in 1..=limit {
        p[n] = 2 * n * n - 1;
    }

    for n in 2..=limit {
        let t = p[n];
        if t == 1 {
            continue;
        }
        if t == 2 * n * n - 1 {
            result += 1;
        }

        for k in 1.. {
            let i = t * k - n;
            if i > limit {
                break;
            }
            while p[i] % t == 0 {
                p[i] /= t;
            }
        }

        for k in 1.. {
            let i = t * k + n;
            if i > limit {
                break;
            }
            while p[i] % t == 0 {
                p[i] /= t;
            }
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem216::problem216;

    #[test]
    fn test_problem216() {
        let result = problem216();
        assert_eq!(result, "5437849");
    }
}
