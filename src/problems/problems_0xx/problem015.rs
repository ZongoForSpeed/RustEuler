use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;

register_problem!(15, "Lattice paths", problem015);

pub fn problem015() -> String {
    // Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down,
    // there are exactly 6 routes to the bottom right corner.
    //
    // How many such routes are there through a 20×20 grid?
    let result = MpzNumber::binomial_ui(40, 20);
    println!("C(40, 20) = {}", result);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem015() {
        let result = problem015();
        assert_eq!(result, "137846528820");
    }
}