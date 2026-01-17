use crate::register_problem;

register_problem!(
    164,
    "Numbers for which no three consecutive digits have a sum greater than a given value",
    problem164
);

fn count(mut counter: &mut Vec<Vec<Vec<u64>>>, d1: usize, d2: usize, reste: usize) -> u64 {
    if reste == 0 {
        return 1;
    }
    if counter[d1][d2][reste] == 0 {
        for i in 0..10 - d1 - d2 {
            counter[d1][d2][reste] += count(&mut counter, d2, i, reste - 1);
        }
    }
    counter[d1][d2][reste]
}

pub fn problem164() -> String {
    // How many 20 digit numbers n (without any leading zero) exist such that no three consecutive
    // digits of n have a sum greater than 9?
    let limite = 20;
    let mut counter: Vec<Vec<Vec<u64>>> = vec![vec![vec![0; limite]; 10]; 10];

    let mut resultat = 0;
    for i in 1..10 {
        resultat += count(&mut counter, 0, i, limite - 1);
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem164() {
        let result = problem164();
        assert_eq!(result, "378158756814587");
    }
}
