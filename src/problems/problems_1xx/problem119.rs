use crate::maths::digits::Digits;
use crate::register_problem;

register_problem!(119, "Digit power sum", problem119);

pub fn problem119() -> String {
    // The number 512 is interesting because it is equal to the sum of its digits raised to some power: 5 + 1 + 2 = 8,
    // and 8^3 = 512. Another example of a number with this property is 614656 = 28^4.
    //
    // We shall define an to be the nth term of this sequence and insist that a number must contain at least two digits
    // to have a sum.
    //
    // You are given that a2 = 512 and a10 = 614656.
    //
    // Find a30.
    let length:u128 = 18;
    let borne = 10u128.pow(length as u32);

    let mut result = vec![0];
    for d in 2.. 9*length {
        for n in std::iter::successors(Some(d), |&n| Some(n * d)).take_while(|&n| n < borne) {
            if n > 10 && n.sum_digits(10) == d {
                result.push(n);
            }
        }
    }


    result.sort();
    println!("a_n = {:?}", result);
    result[30].to_string()
}
