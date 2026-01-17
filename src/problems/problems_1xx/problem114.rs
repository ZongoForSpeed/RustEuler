use crate::register_problem;

register_problem!(114, "Counting block combinations I", problem114);

pub fn problem114() -> String {
    // A row measuring seven units in length has red blocks with a minimum length of three units placed on it, such that
    // any two red blocks (which are allowed to be different lengths) are separated by at least one black square. There
    // are exactly seventeen ways of doing this.
    //
    // How many ways can a row measuring fifty units in length be filled?
    //
    // NOTE: Although the example above does not lend itself to the possibility, in general it is permitted to mix block
    // sizes. For example, on a row measuring eight units in length you could use red (3), black (1), and red (4).
    let limit = 50;
    let mut values: Vec<u64> = vec![1, 1, 1, 2];
    for n in 4..=limit {
        values.push(2 * values[n - 1] - values[n - 2] + values[n - 4]);
    }

    values[limit].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem114() {
        let result = problem114();
        assert_eq!(result, "16475640049");
    }
}
