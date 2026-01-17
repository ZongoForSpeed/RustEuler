use crate::register_problem;

register_problem!(
    181,
    "Investigating in how many ways objects of two different colours can be grouped",
    problem181
);

type Pair = (usize, usize);

fn increment(objects: &Pair, index: &mut Pair, head_room: &Pair) -> bool {
    if index.0 + head_room.0 < objects.0 {
        index.0 += 1;
        return true;
    }
    index.0 = 0;

    if index.1 + head_room.1 < objects.1 {
        index.1 += 1;
        return true;
    }
    index.1 = 0;
    false
}

pub fn problem181() -> String {
    // Having three black objects B and one white object W they can be grouped in 7 ways like this:
    //
    //      (BBBW)   (B,BBW)   (B,B,BW)   (B,B,B,W)   (B,BB,W)   (BBB,W)   (BB,BW)
    //
    // In how many ways can sixty black objects B and forty white objects W be thus grouped?
    let objects: Pair = (60, 40);

    let mut combination: Vec<Vec<u64>> = vec![vec![0; 40 + 1]; 60 + 1];
    combination[0][0] = 1;

    let zero: Pair = (0, 0);
    let mut addition: Pair = (0, 0);

    while increment(&objects, &mut addition, &zero) {
        let mut debut: Pair = (0, 0);
        combination[debut.0 + addition.0][debut.1 + addition.1] += combination[debut.0][debut.1];

        while increment(&objects, &mut debut, &addition) {
            combination[debut.0 + addition.0][debut.1 + addition.1] +=
                combination[debut.0][debut.1];
        }
    }

    combination[objects.0][objects.1].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem181() {
        let result = problem181();
        assert_eq!(result, "83735848679360680");
    }
}
