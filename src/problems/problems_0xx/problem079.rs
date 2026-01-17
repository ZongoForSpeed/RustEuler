use std::collections::HashMap;
use std::path::Path;
use string_builder::Builder;
use crate::register_problem;

register_problem!(79, "Passcode derivation", problem079);

fn is_empty(keys: &Vec<String>) -> bool {
    keys.iter().all(|key| key.is_empty())
}

fn minimum(keys: &Vec<String>) -> char {
    let mut maximum: HashMap<char, usize> = HashMap::new();

    for key in keys {
        for (n, u8) in key.as_bytes().iter().enumerate() {
            maximum.entry(*u8 as char).and_modify(|e| *e = std::cmp::max(*e, n)).or_insert(n);
        }
    }

    for (c, n) in maximum {
        if n == 0 {
            return c;
        }
    }

    ' '
}


pub fn problem079() -> String {
    // A common security method used for online banking is to ask the user for three random
    // characters from a passcode.
    // For example, if the passcode was 531278, they may ask for the 2nd, 3rd, and 5th characters;
    // the expected reply would be: 317.
    //
    // The text file, keylog.txt, contains fifty successful login attempts.
    //
    // Given that the three characters are always asked for in order, analyse the file so as to
    // determine the shortest possible secret passcode of unknown length.
    let path = Path::new("data/p079_keylog.txt");
    let mut keys = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect::<Vec<String>>();

    let mut password = Builder::default();
    while !is_empty(&keys) {
            let c = minimum(&keys);
        password.append(c);
        for i in 0..keys.len() {
            let key = &keys[i];
            if !key.is_empty() && key.starts_with(c){
                keys[i] = key[1..].to_string();
            }
        }
    }

    password.string().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem079() {
        let result = problem079();
        assert_eq!(result, "73162890");
    }
}
