use crate::register_problem;
use std::collections::{BTreeMap, HashMap};
use std::ops::AddAssign;
use crate::utils::mpz_number::MpzNumber;

register_problem!(253, "Tidying up", problem253);

type Dictionary = BTreeMap<u32, MpzNumber>;

fn algorithm(mut cache: &mut HashMap<Vec<u32>, Dictionary>, sizes: &Vec<u32>) -> Dictionary {
    if let Some(found) = cache.get(sizes) {
        return found.clone();
    }

    let sum = sizes.iter().sum();
    if sum == 1 && sizes[1] == 1 {
        let result = Dictionary::from([(1, 1.into())]);
        result
    } else {
        let mut result = Dictionary::new();
        let mut copy_sizes = sizes.clone();
        for n in 0..sizes.len() {
            let size = sizes[n];
            if size == 0 {
                continue;
            }
            copy_sizes[n] -= 1;
            if n == 1 {
                let tmp = algorithm(&mut cache, &copy_sizes);
                join(&mut result, tmp, sum, size);
            } else {
                for m1 in 0..n {
                    let m2 = n - m1 - 1;
                    if m1 != 0 {
                        copy_sizes[m1] += 1;
                    }
                    if m2 != 0 {
                        copy_sizes[m2] += 1;
                    }

                    let tmp = algorithm(&mut cache, &copy_sizes);
                    join(&mut result, tmp, sum, size);
                    
                    if m1 != 0 {
                        copy_sizes[m1] -= 1;
                    }
                    if m2 != 0 {
                        copy_sizes[m2] -= 1;
                    }
                }
            }
            copy_sizes[n] += 1;
        }

        cache.insert(sizes.clone(), result.clone());
        result
    }
}

fn join(result: &mut BTreeMap<u32, MpzNumber>, tmp: Dictionary, sum: u32, size: u32) {
    for (k, v) in tmp {
        if k < sum {
            result.entry(sum).or_insert_with(MpzNumber::new).add_assign(v * size);
        } else {
            result.entry(k).or_insert_with(MpzNumber::new).add_assign(v * size);
        }
    }
}

/// A small child has a “number caterpillar” consisting of forty jigsaw pieces, each with one number on it, which,
/// when connected together in a line, reveal the numbers 1 to 40 in order.
///
/// Every night, the child's father has to pick up the pieces of the caterpillar that have been scattered across the
/// play room. He picks up the pieces at random and places them in the correct order.
/// As the caterpillar is built up in this way, it forms distinct segments that gradually merge together.
/// The number of segments starts at zero (no pieces placed), generally increases up to about eleven or twelve, then
/// tends to drop again before finishing at a single segment (all pieces placed).
///
/// For example:
///
///                      Piece Placed    Segments So Far
///                      12              1
///                      4	            2
///                      29	            3
///                      6	            4
///                      34	            5
///                      5	            4
///                      35	            4
///                      …	            …
///
/// Let M be the maximum number of segments encountered during a random tidy-up of the caterpillar.
/// For a caterpillar of ten pieces, the number of possibilities for each M is
/// 
///                                  M	Possibilities
///                                  1	512      
///                                  2	250912      
///                                  3	1815264      
///                                  4	1418112      
///                                  5	144000      
/// so the most likely value of M is 3 and the average value is 385643⁄113400 = 3.400732, rounded to six decimal places.
///
/// The most likely value of M for a forty-piece caterpillar is 11; but what is the average value of M?
///
/// Give your answer rounded to six decimal places.
pub fn problem253() -> String {
    let n = 40;

    let mut input = vec![0; n];
    input.push(1);

    let mut cache = HashMap::new();
    let combination = algorithm(&mut cache, &input);

    let mut numer = MpzNumber::new();
    let mut denom = MpzNumber::new();

    println!("combination: {:?}", combination);
    for (k, v) in combination {
        numer += k * &v;
        denom += &v;
    }

    let mask = u32::pow(10, 7);
    numer *= mask;
    numer /= denom;
    let result = numer.get_f64() / mask as f64;

    format!("{:.6}", result)
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem253::problem253;

    #[test]
    fn test_problem253() {
        let result = problem253();
        assert_eq!(result, "11.492847");
    }
}
