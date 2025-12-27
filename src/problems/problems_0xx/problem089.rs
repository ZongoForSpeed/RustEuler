use crate::register_problem;
use std::collections::HashMap;
use std::path::Path;

fn read(values: &HashMap<char, i32>, line: &String) -> usize {
    let chars = line.chars().collect::<Vec<char>>();
    let mut n: i32 = 0;
    let len = chars.len();
    for i in 0..len {
        let c = chars[i];
        if let Some(r) = values.get(&c) {
            if (i + 1..len).map(|n| chars[n]).any(|c| {
                return match values.get(&c) {
                    None => false,
                    Some(v) => v > r,
                };
            }) {
                n -= r;
            } else {
                n += r;
            }
        }
    }
    n as usize
}

register_problem!(89, "Roman numerals", problem089);

pub fn problem089() -> String {
    // For a number written in Roman numerals to be considered valid there are basic rules which must be followed. Even
    // though the rules allow some numbers to be expressed in more than one way there is always a "best" way of writing
    // a particular number.
    //
    // For example, it would appear that there are at least six ways of writing the number sixteen:
    //
    //      IIIIIIIIIIIIIIII
    //      VIIIIIIIIIII
    //      VVIIIIII
    //      XIIIIII
    //      VVVI
    //      XVI
    //
    // However, according to the rules only XIIIIII and XVI are valid, and the last example is considered to be the most
    // efficient, as it uses the least number of numerals.
    //
    // The 11K text file, roman.txt (right click and 'Save Link/Target As...'), contains one thousand numbers written in
    // valid, but not necessarily minimal, Roman numerals; see About... Roman Numerals for the definitive rules for this
    // problem.
    //
    // Find the number of characters saved by writing each of these in their minimal form.
    //
    // Note: You can assume that all the Roman numerals in the file contain no more than four consecutive identical
    // units.
    let values: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    let units = vec!["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
    let tens = vec!["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    let hundreds = vec!["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
    let thousands = vec![
        "",
        "M",
        "MM",
        "MMM",
        "MMMM",
        "MMMMM",
        "MMMMMM",
        "MMMMMMM",
        "MMMMMMMM",
        "MMMMMMMMM",
    ];

    let path = Path::new("data/p089_roman.txt");
    let lines: Vec<String> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect();

    let mut result = 0;
    for line in lines {
        let n = read(&values, &line);

        let mut m = n;
        // println!("{line} => {n}");

        let mut size = 0;
        size += thousands[m / 1000].len();
        m %= 1000;
        size += hundreds[m / 100].len();
        m %= 100;
        size += tens[m / 10].len();
        m %= 10;
        size += units[m].len();

        result += line.len() - size;
    }

    result.to_string()
}
