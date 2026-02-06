use crate::register_problem;
use fraction::GenericFraction;
use num_traits::{ConstZero, Zero};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

register_problem!(259, "Reachable Numbers", problem259);

type IFraction = GenericFraction<i64>;
type Cache = HashMap<(usize, usize), Rc<HashSet<IFraction>>>;

/// Returns all possible values that can be formed by an arithmetic expression
/// using digits in the range [start, end).
fn algorithm(
    cache: &mut Cache,
    digits: &[i64],
    start: usize,
    end: usize,
) -> Rc<HashSet<IFraction>> {
    if let Some(result) = cache.get(&(start, end)) {
        return Rc::clone(result);
    }

    let mut result = HashSet::new();

    // Base case: Concatenate all digits in the range [start, end)
    let mut f = IFraction::zero();
    for i in start..end {
        f *= 10;
        f += digits[i];
    }
    result.insert(f);

    // Recursive case: Split the range into two at every possible position
    for i in 1..(end - start) {
        let left_values = algorithm(cache, digits, start, start + i);
        let right_values = algorithm(cache, digits, start + i, end);

        for a in left_values.iter() {
            for b in right_values.iter() {
                result.insert(a + b);
                result.insert(a - b);
                result.insert(a * b);
                if !b.is_zero() {
                    result.insert(a / b);
                }
            }
        }
    }

    let result = Rc::new(result);
    cache.insert((start, end), Rc::clone(&result));
    result
}

/// A positive integer will be called reachable if it can result from an
/// arithmetic expression obeying the following rules:
///
///      . Uses the digits 1 through 9, in that order and exactly once each.
///      . Any successive digits can be concatenated (for example, using the
///      digits 2, 3 and 4 we obtain the number 234).
///      . Only the four usual binary arithmetic operations (addition,
///      subtraction, multiplication and division) are allowed.
///      . Each operation can be used any number of times, or not at all.
///      . Unary minus is not allowed.
///      . Any number of (possibly nested) parentheses may be used to define
///      the order of operations.
///
/// For example, 42 is reachable, since (1/23) * ((4*5)-6) * (78-9) = 42.
///
/// What is the sum of all positive reachable integers?
pub fn problem259() -> String {
    let digits: Vec<i64> = (1..=9).collect();

    let mut cache = HashMap::new();
    let reachable_values = algorithm(&mut cache, &digits, 0, digits.len());

    let mut result = 0;
    for &val in reachable_values.iter() {
        if val > IFraction::ZERO && val.denom() == Some(&1) {
            if let Some(&n) = val.numer() {
                result += n;
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem259::problem259;

    #[test]
    fn test_problem259() {
        let result = problem259();
        assert_eq!(result, "20101196798");
    }
}
