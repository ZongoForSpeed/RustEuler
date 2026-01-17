use crate::register_problem;
use crate::utils::mpq_fraction::MpqFraction;
use std::collections::HashMap;

register_problem!(151, "Paper sheets of standard sizes: an expected-value problem", problem151);

fn expected_value(mut cache: &mut HashMap<Vec<u16>, MpqFraction>, envelope: Vec<u16>) -> MpqFraction {
    if let Some(value) = cache.get(&envelope) {
        return value.clone();
    }

    let mut result = MpqFraction::new();
    if !envelope.is_empty() {
        for (i, sheet) in envelope.iter().enumerate() {
            let mut new_envelope = envelope.clone();
            new_envelope.remove(i);
            for j in sheet+1..=5 {
                new_envelope.push(j);
            }
            new_envelope.sort();
            result += expected_value(&mut cache, new_envelope);
        }

        result /= envelope.len();
        if envelope.len() == 1 {
            result += 1;
        }
    }

    cache.insert(envelope, result.clone());
    result
}

pub fn problem151() -> String {
    // A printing shop runs 16 batches (jobs) every week and each batch requires a sheet of special colour-proofing
    // paper of size A5.
    //
    // Every Monday morning, the foreman opens a new envelope, containing a large sheet of the special paper with size
    // A1.
    //
    // He proceeds to cut it in half, thus getting two sheets of size A2. Then he cuts one of them in half to get two
    // sheets of size A3 and so on until he obtains the A5-size sheet needed for the first batch of the week.
    //
    // All the unused sheets are placed back in the envelope.
    //
    // At the beginning of each subsequent batch, he takes from the envelope one sheet of paper at random. If it is of
    // size A5, he uses it. If it is larger, he repeats the 'cut-in-half' procedure until he has what he needs and any
    // remaining sheets are always placed back in the envelope.
    //
    // Excluding the first and last batch of the week, find the expected number of times (during each week) that the
    // foreman finds a single sheet of paper in the envelope.
    //
    // Give your answer rounded to six decimal places using the format x.xxxxxx .
    let mut cache = HashMap::new();
    let f = expected_value(&mut cache, vec![1]) - 2;

    format!("{:.6}", f.get_f())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem151() {
        let result = problem151();
        assert_eq!(result, "0.464399");
    }
}
