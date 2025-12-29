use crate::register_problem;

register_problem!(115, "Counting block combinations II", problem115);

pub fn problem115() -> String {
    // NOTE: This is a more difficult version of Problem 114.
    //
    // A row measuring n units in length has red blocks with a minimum length of m units placed on it, such that any two
    // red blocks (which are allowed to be different lengths) are separated by at least one black square.
    //
    // Let the fill-count function, F(m, n), represent the number of ways that a row can be filled.
    //
    // For example, F(3, 29) = 673135 and F(3, 30) = 1089155.
    //
    // That is, for m = 3, it can be seen that n = 30 is the smallest value for which the fill-count function first
    // exceeds one million.
    //
    // In the same way, for m = 10, it can be verified that F(10, 56) = 880711 and F(10, 57) = 1148904, so n = 57 is the
    // least value for which the fill-count function first exceeds one million.
    //
    // For m = 50, find the least value of n for which the fill-count function first exceeds one million.3
    let limit = 1000000u32;
    let m = 50;
    let mut values = vec![1; m];
    values.push(2);

    for n in m + 1.. {
        let mut v = values.last().unwrap() + 2;
        for k in 1..n - m {
            v += values[k];
        }
        values.push(v);
        if v > limit {
            return n.to_string();
        }
    }

    panic!()
}
