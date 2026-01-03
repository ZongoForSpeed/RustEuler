use crate::register_problem;

register_problem!(
    174,
    "Counting the number of \"hollow\" square laminae that can form one, two, three, ... distinct arrangements",
    problem174
);

pub fn problem174() -> String {
    // We shall define a square lamina to be a square outline with a square "hole" so that the shape
    // possesses vertical and horizontal symmetry.
    //
    // Given eight tiles it is possible to form a lamina in only one way: 3x3 square with a 1x1 hole
    // in the middle. However, using thirty-two tiles it is possible to form two distinct laminae.
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
    // If t represents the number of tiles used, we shall say that t = 8 is type L(1) and t = 32 is
    // type L(2).
    //
    // Let N(n) be the number of t ≤ 1000000 such that t is type L(n); for example, N(15) = 832.
    //
    // What is ∑ N(n) for 1 ≤ n ≤ 10? We shall define a square lamina to be a square outline with a
    // square "hole" so that the shape
    let limit = 1000000;
    let mut result = 0;
    for n in (4..=limit).step_by(4) {
        let mut count = 0;
        for d in (2..).step_by(2).take_while(|d| d * d < n) {
            if n % (2 * d) == 0 {
                count += 1;
            }
        }

        if 0 < count && count < 11 {
            result += 1;
        }
    }

    result.to_string()
}
