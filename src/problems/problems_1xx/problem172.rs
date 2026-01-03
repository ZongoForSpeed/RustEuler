use crate::register_problem;

register_problem!(
    172,
    "Investigating numbers with few repeated digits",
    problem172
);

pub fn problem172() -> String {
    // How many 18-digit numbers n (without leading zeros) are there such that no digit occurs more
    // than three times in n?
    let size: usize = 18;
    let mut factorials = vec![1];
    for n in 1..=size {
        factorials.push(factorials.last().unwrap() * n);
    }

    let mut result = 0;

    for i0 in 0..4 {
        for i1 in 0..4 {
            for i2 in 0..4 {
                for i3 in 0..4 {
                    for i4 in 0..4 {
                        for i5 in 0..4 {
                            for i6 in 0..4 {
                                for i7 in 0..4 {
                                    for i8 in 0..4 {
                                        for i9 in 0..4 {
                                            if i0 + i1 + i2 + i3 + i4 + i5 + i6 + i7 + i8 + i9
                                                == size
                                            {
                                                result += factorials[size]
                                                    / factorials[i0]
                                                    / factorials[i1]
                                                    / factorials[i2]
                                                    / factorials[i3]
                                                    / factorials[i4]
                                                    / factorials[i5]
                                                    / factorials[i6]
                                                    / factorials[i7]
                                                    / factorials[i8]
                                                    / factorials[i9];
                                                if i0 > 0 {
                                                    result -= factorials[size - 1]
                                                        / factorials[i0 - 1]
                                                        / factorials[i1]
                                                        / factorials[i2]
                                                        / factorials[i3]
                                                        / factorials[i4]
                                                        / factorials[i5]
                                                        / factorials[i6]
                                                        / factorials[i7]
                                                        / factorials[i8]
                                                        / factorials[i9];
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    result.to_string()
}
