use crate::register_problem;

register_problem!(208, "Robot Walks", problem208);

pub fn problem208() -> String {
    // A robot moves in a series of one-fifth circular arcs (72°), with a free choice of a clockwise
    // or an anticlockwise arc for each step, but no turning on the spot.
    //
    // One of 70932 possible closed paths of 25 arcs starting northward is
    //
    // Given that the robot starts facing North, how many journeys of 70 arcs in length can it take
    // that return it, after the final arc, to its starting position?
    // (Any arc may be traversed multiple times.)
    let maximum = 14;
    let mut choix = vec![vec![0u64; maximum]; maximum];

    choix[0][0] = 1;
    for i in 1..maximum {
        choix[i][0] = 1;
        choix[i][i] = 1;
    }

    for i in 1..maximum {
        for j in 1..i {
            choix[i][j] = choix[i - 1][j - 1] + choix[i - 1][j];
        }
    }

    let mut result = 0;
    let choix_max = &choix[maximum - 1];

    for n in 0..maximum {
        result += choix_max[n].pow(5);
        if n > 0 {
            result += choix_max[n].pow(4) * choix_max[n - 1].pow(1);
            result += choix_max[n].pow(3) * choix_max[n - 1].pow(2);
            result += choix_max[n].pow(2) * choix_max[n - 1].pow(3);
            result += choix_max[n].pow(1) * choix_max[n - 1].pow(4);
        }
    }

    result *= 2;
    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem208::problem208;

    #[test]
    fn test_problem208() {
        let result = problem208();
        assert_eq!(result, "331951449665644800");
    }
}
