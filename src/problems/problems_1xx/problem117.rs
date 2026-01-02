use crate::register_problem;
use crate::utils::mpz_number::MpzNumber;

register_problem!(117, "Red, green, and blue tiles", problem117);

pub fn problem117() -> String {
    // Using a combination of black square tiles and oblong tiles chosen from: red tiles measuring two units, green
    // tiles measuring three units, and blue tiles measuring four units, it is possible to tile a row measuring five
    // units in length in exactly fifteen different ways.
    //
    // How many ways can a row measuring fifty units in length be tiled?
    //
    // NOTE: This is related to Problem 116.
    let length = 50;
    let mut result = MpzNumber::new();

    for s4 in (0..=length).step_by(4) {
        for s3 in (0..=length - s4).step_by(3) {
            for s2 in (0..=length - s3 - s4).step_by(2) {
                let n4 = s4 / 4;
                let n3 = s3 / 3;
                let n2 = s2 / 2;
                let n1 = length - s2 - s3 - s4;

                let n = MpzNumber::factorial(n1 + n2 + n3 + n4);
                let d = MpzNumber::factorial(n1)
                    * MpzNumber::factorial(n2)
                    * MpzNumber::factorial(n3)
                    * MpzNumber::factorial(n4);

                result += n / d;
            }
        }
    }

    result.to_string()
}
