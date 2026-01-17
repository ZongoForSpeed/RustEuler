use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelIterator;
use crate::register_problem;

register_problem!(148, "Exploring Pascal's triangle", problem148);

fn pascal(n: u64, base: u64) -> u64 {
    let mut p = n;
    let mut r = 1;
    while p != 0 {
        r *= p % base + 1;
        p /= base;
    }

    r
}

pub fn problem148() -> String {
    // We can easily verify that none of the entries in the first seven rows of Pascal's triangle are divisible by 7:
    //
    //  	  	  	  	  	  	 1
    //  	  	  	  	  	 1 	  	 1
    //  	  	  	  	 1 	  	 2 	  	 1
    //  	  	  	 1 	  	 3 	  	 3 	  	 1
    //  	  	 1 	  	 4 	  	 6 	  	 4 	  	 1
    //  	 1 	  	 5 	  	10 	  	10 	  	 5 	  	 1
    //	1 	  	 6 	  	15 	  	20 	  	15 	  	 6 	  	 1
    //
    // However, if we check the first one hundred rows, we will find that only 2361 of the 5050 entries are not
    // divisible by 7.
    //
    // Find the number of entries which are not divisible by 7 in the first one billion (10^9) rows of
    // Pascal's triangle.
    let base = 7;
    (0..1000000000).into_par_iter().map(|n| pascal(n, base)).sum::<u64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem148() {
        let result = problem148();
        assert_eq!(result, "2129970655314432");
    }
}
