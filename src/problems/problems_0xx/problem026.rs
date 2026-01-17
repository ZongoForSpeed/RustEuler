use crate::register_problem;

fn longueur_cycle(n: u64) -> usize {
    let mut restes: Vec<u64> = Vec::new();
    let mut reste = 1;
    while !restes.contains(&reste) {
        restes.push(reste);
        while reste % n == reste {
            reste *= 10;
        }
        reste = reste % n;
    }
    restes.len()
}

register_problem!(26, "Reciprocal cycles", problem026);

pub fn problem026() -> String {
    // A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators
    // 2 to 10 are given:
    //
    //    1/2    =     0.5
    //    1/3    =     0.(3)
    //    1/4    =     0.25
    //    1/5    =     0.2
    //    1/6    =     0.1(6)
    //    1/7    =     0.(142857)
    //    1/8    =     0.125
    //    1/9    =     0.(1)
    //    1/10    =     0.1
    //
    // Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring
    // cycle.
    //
    // Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.
    let mut l_max = 0;
    let mut result = 0;
    for n in 2..1000 {
        if n % 2 != 0 && n % 5 != 0 {
            let l = longueur_cycle(n);
            if l > l_max {
                l_max = l;
                result = n;
            }
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem026() {
        let result = problem026();
        assert_eq!(result, "983");
    }
}