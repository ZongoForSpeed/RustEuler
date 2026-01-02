use crate::register_problem;

register_problem!(159, "Digital root sums of factorisations", problem159);

pub fn problem159() -> String {
    // A composite number can be factored many different ways. For instance, not including multiplication by one, 24 can
    // be factored in 7 distinct ways:
    //
    //       24 = 2x2x2x3
    //       24 = 2x3x4
    //       24 = 2x2x6
    //       24 = 4x6
    //       24 = 3x8
    //       24 = 2x12
    //       24 = 24
    //
    // Recall that the digital root of a number, in base 10, is found by adding together the digits of that number, and
    // repeating that process until a number is arrived at that is less than 10. Thus the digital root of 467 is 8.
    //
    // We shall call a Digital Root Sum (DRS) the sum of the digital roots of the individual factors of our number.
    // The chart below demonstrates all of the DRS values for 24.
    //
    //                  Factorisation   Digital Root Sum
    //                        2x2x2x3                  9
    //                          2x3x4                  9
    //                          2x2x6                 10
    //                            4x6                 10
    //                            3x8                 11
    //                           2x12                  5
    //                             24                  6
    //
    // The maximum Digital Root Sum of 24 is 11.
    // The function mdrs(n) gives the maximum Digital Root Sum of n. So mdrs(24)=11.
    // Find ∑mdrs(n) for 1 < n < 1,000,000.
    let limite = 1000000;
    let mut max_digital_root_sum = vec![0; limite];
    max_digital_root_sum[0] = 0;
    max_digital_root_sum[1] = 0;
    for n in 2..limite {
        let k = n % 9;
        max_digital_root_sum[n] = if k > 0 { k } else { 9 };
    }

    for i in 2..=limite / 2 {
        for j in (2..).take_while(|j| i*j < limite) {
            max_digital_root_sum[i * j] = std::cmp::max(max_digital_root_sum[i * j], max_digital_root_sum[i] + max_digital_root_sum[j]);
        }
    }

    println!("max_digital_root_sum[24] = {}", max_digital_root_sum[24]);
    max_digital_root_sum.iter().sum::<usize>().to_string()
}
