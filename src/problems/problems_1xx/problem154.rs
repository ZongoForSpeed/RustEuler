use crate::maths::arithmetic::Arithmetic;
use crate::register_problem;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

register_problem!(154, "Exploring Pascal's pyramid", problem154);

pub fn problem154() -> String {
    // A triangular pyramid is constructed using spherical balls so that each ball rests on exactly three balls of the
    // next lower level.
    //
    // Then, we calculate the number of paths leading from the apex to each position:
    //
    // A path starts at the apex and progresses downwards to any of the three spheres directly below the current
    // position.
    //
    // Consequently, the number of paths to reach a certain position is the sum of the numbers immediately above it
    // (depending on the position, there are up to three numbers above it).
    //
    // The result is Pascal's pyramid and the numbers at each level n are the coefficients of the trinomial expansion
    // (x + y + z)^n.
    //
    // How many coefficients in the expansion of (x + y + z)^200000 are multiples of 10^12?
    let n = 200000;

    let mut twos = vec![0; n + 1];
    let mut fives = vec![0; n + 1];

    for i in 1..=n {
        twos[i] = twos[i - 1] + i.count_factor(2);
        fives[i] = fives[i - 1] + i.count_factor(5);
    }

    // C(n,i,j) = n! / i!j!(n-i-j)!
    let result = (0..=n)
        .into_par_iter()
        .map(|i| {
            let fni = fives[n] - fives[i];
            let tni = twos[n] - twos[i];

            let mut count = 0;
            for j in 0..=n - i {
                if fni - fives[j] - fives[n - i - j] >= 12 && tni - twos[j] - twos[n - i - j] >= 12
                {
                    count += 1;
                }
            }
            count
        })
        .sum::<usize>();
    result.to_string()
}
