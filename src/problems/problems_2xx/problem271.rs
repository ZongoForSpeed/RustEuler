use crate::register_problem;

register_problem!(
    271,
    "Modular Cubes, part 1",
    problem271
);

fn s(n: u128) -> u128 {
    let mut somme = 0;
    let increment: u128 = 2 * 3 * 5 * 11 * 17 * 23 * 29 * 41;

    for x in (increment + 1..n).step_by(increment as usize) {
        let x3 = x * x % n * x % n;
        if x3 == 1 {
            somme += x;
        }
    }

    somme
}

/// For a positive number n, define S(n) as the sum of the integers x, for which 1<x<n and x3≡1 mod n.
///
/// When n=91, there are 8 possible values for x, namely : 9, 16, 22, 29, 53, 74, 79, 81. Thus,
/// S(91)=9+16+22+29+53+74+79+81=363.
///
/// Find S(13082761331670030).
pub fn problem271() -> String {
    s(13082761331670030).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem271() {
        assert_eq!(problem271(), "4617456485273129588");
    }
}
