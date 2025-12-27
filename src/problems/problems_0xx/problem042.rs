use std::collections::HashSet;
use std::path::Path;
use crate::register_problem;


fn conversion(s: &String) -> usize {
    s.bytes().map(|c| {
        if c == '"' as u8 {
            return 0;
        } else {
            let d = c - 'A' as u8;
            (d + 1) as usize
        }
    }).sum()
}

register_problem!(42, "Coded triangle numbers", problem042);

pub fn problem042() -> String {
    // The nth term of the sequence of triangle numbers is given by, tn = ½n(n+1); so the first ten
    // triangle numbers are:
    //
    //                                      1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
    //
    // By converting each letter in a word to a number corresponding to its alphabetical position
    // and adding these values we form a word value. For example, the word value for SKY is
    // 19 + 11 + 25 = 55 = t10. If the word value is a triangle number then we shall call the word
    // a triangle word.
    //
    // Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly
    // two-thousand common English words, how many are triangle words?
    let path = Path::new("data/p042_words.txt");
    let words: Vec<String> = std::fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(String::from)
        .collect();

    let mut triangles= HashSet::new();
    for n in 1..50 {
        triangles.insert(n * (n + 1) / 2);
    }

    let mut total = 0;
    for word in words {
        let c = conversion(&word);
        if triangles.contains(&c) {
            println!("{} -> {}", word, c);
            total += 1;
        }
    }
    total.to_string()
}
