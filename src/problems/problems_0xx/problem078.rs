use crate::register_problem;

register_problem!(78, "Counting summations", problem078);

fn parity(k: usize) -> i32 {
    if k % 2 == 0 { -1 } else { 1 }
}

pub fn problem078() -> String {
    // It is possible to write five as a sum in exactly six different ways:
    //
    //      4 + 1
    //      3 + 2
    //      3 + 1 + 1
    //      2 + 2 + 1
    //      2 + 1 + 1 + 1
    //      1 + 1 + 1 + 1 + 1
    //
    // How many different ways can one hundred be written as a sum of at least two positive integers?
    let mut partition: Vec<i32> = vec![1];
    // https://en.wikipedia.org/wiki/Partition_function_(number_theory)
    for n in 1.. {
        let mut p = 0;
        for k in 1..=n {
            if 2 * n >= k * (3 * k - 1) {
                p += parity(k) * partition[n - k * (3 * k - 1) / 2];
            }
            if 2 * n >= k * (3 * k + 1) {
                p += parity(k) * partition[n - k * (3 * k + 1) / 2];
            }
        }
        if p % 1000000 == 0 {
            return n.to_string();
        }
        partition.push(p % 1000000);
    }

    unreachable!()
}
