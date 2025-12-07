use crate::maths::timer::ScopeTimer;
use std::path::Path;

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

pub fn problem022() -> usize {
    let _timer = ScopeTimer::new("Problem 22 Names scores", false);
    // Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing
    // over five-thousand first names, begin by sorting it into alphabetical order.
    // Then working out the alphabetical value for each name, multiply this value by its alphabetical
    // position in the list to obtain a name score.
    //
    // For example, when the list is sorted into alphabetical order, COLIN, which is worth
    // 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a
    // score of 938 × 53 = 49714.
    //
    // What is the total of all the name scores in the file?
    let path = Path::new("data/p022_names.txt");
    let mut lines: Vec<String> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .map(|s| s.split(',').into_iter().map(String::from).collect::<Vec<String>>())
        .flatten()
        .collect();

    lines.sort();
    let mut total = 0;
    for (n, word) in lines.iter().enumerate() {
        total += (n + 1) * conversion(&word);
    }
    total
}
