use crate::register_problem;
use num_integer::Roots;

register_problem!(
    173,
    "Using up to one million tiles how many different \"hollow\" square laminae can be formed?",
    problem173
);

pub fn problem173() -> String {
    // We shall define a square lamina to be a square outline with a square "hole" so that the shape possesses vertical
    // and horizontal symmetry. For example, using exactly thirty-two square tiles we can form two different square
    // laminae:
    //
    //              XXXXXX  XXXXXXXXX
    //              XXXXXX  X       X
    //              XX  XX  X       X
    //              XX  XX  X       X
    //              XXXXXX  X       X
    //              XXXXXX  X       X
    //                      X       X
    //                      X       X
    //                      XXXXXXXXX
    //
    // With one-hundred tiles, and not necessarily using all of the tiles at one time, it is possible to form forty-one
    // different square laminae.
    //
    // Using up to one million tiles how many different square laminae can be formed?
    let limit = 1000000;
    let mut result: u64 = 0;
    for n in 1..(limit + 1) / 2 {
        result += ((n * n + limit).sqrt() - n) / 2;
    }

    result.to_string()
}
