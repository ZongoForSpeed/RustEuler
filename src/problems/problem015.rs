use crate::maths::timer::ScopeTimer;
use crate::utils::mpz_nombre::MpzNombre;

pub fn problem015() -> u64 {
    let _timer = ScopeTimer::new("Problem 15 Lattice paths", false);
    // Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down,
    // there are exactly 6 routes to the bottom right corner.
    //
    // How many such routes are there through a 20×20 grid?
    let result = MpzNombre::binomial_ui(40, 20);
    println!("C(40, 20) = {}", result);
    result.get_ui()
}
