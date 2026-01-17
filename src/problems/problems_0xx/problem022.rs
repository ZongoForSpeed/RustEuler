use crate::register_problem;
use std::path::Path;

fn to_usize(c: u8) -> usize {
    if c == '"' as u8 {
        0
    } else {
        let d = c - 'A' as u8;
        (d + 1) as usize
    }
}

fn conversion(s: &String) -> usize {
    s.bytes().map(to_usize).sum()
}

register_problem!(22, "Names scores", problem022);

pub fn problem022() -> String {
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
        .split(',')
        .map(String::from)
        .collect::<Vec<String>>();

    lines.sort();
    let mut total = 0;
    for (n, word) in lines.iter().enumerate() {
        total += (n + 1) * conversion(&word);
    }
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem022() {
        let result = problem022();
        assert_eq!(result, "871198282");
    }
}