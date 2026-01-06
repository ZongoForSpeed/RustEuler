use std::cmp::Ordering;

const EPSILON: f64 = 1e-9;

pub(crate) fn equal_epsilon(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

pub(crate) fn compare_epsilon(a: f64, b: f64) -> Ordering {
    if equal_epsilon(a, b) {
        Ordering::Equal
    } else if a < b {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}