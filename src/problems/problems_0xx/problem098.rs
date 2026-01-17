use crate::maths::digits::conversion;
use crate::register_problem;
use crate::utils::permutations::permutations;
use std::collections::{HashMap, HashSet};
use std::ops::AddAssign;
use std::path::Path;
use crate::maths::polygonal::Polygonal;

fn are_anagrams(a: &String, b: &String) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut count = HashMap::new();

    for c in a.chars() {
        count.entry(c).or_insert(0).add_assign(1);
    }

    for c in b.chars() {
        match count.get_mut(&c) {
            Some(v) => {
                *v -= 1;
                if *v < 0 {
                    return false;
                }
            }
            None => return false,
        }
    }

    count.values().all(|&v| v == 0)
}


register_problem!(98, "Anagramic squares", problem098);

pub fn problem098() -> String {
    // By replacing each of the letters in the word CARE with 1, 2, 9, and 6 respectively, we form a square number:
    // 1296 = 36². What is remarkable is that, by using the same digital substitutions, the anagram, RACE, also forms a
    // square number: 9216 = 96². We shall call CARE (and RACE) a square anagram word pair and specify further that
    // leading zeroes are not permitted, neither may a different letter have the same digital value as another letter.
    //
    // Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly two-thousand common
    // English words, find all the square anagram word pairs (a palindromic word is NOT considered to be an anagram of
    // itself).
    //
    // What is the largest square number formed by any member of such a pair?
    //
    // NOTE: All anagrams formed must be contained in the given text file.
    let mut anagrams: HashMap<usize, HashSet<Vec<usize>>> = HashMap::new();
    let digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for permute in permutations(digits) {
        
        if permute[0] != 0 {
            for i in 1..10 {
                let permuation = permute[..i].to_vec();
                let n = conversion(&permuation, 10);
                if n.is_square() {
                    anagrams.entry(permuation.len()).or_insert_with(HashSet::new).insert(permuation);
                }
            }
            let n = conversion(&permute, 10);
            if n.is_square() {
                anagrams.entry(permute.len()).or_insert_with(HashSet::new).insert(permute.clone());
            }
        }
    }

    let path = Path::new("data/p098_words.txt");
    let words: Vec<String> = std::fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(|s| s[1..s.len() - 1].to_string())
        .collect::<Vec<String>>();

    let mut result = 0;
    let len = words.len();
    for i in 0..len {
        let word1 = words[i].clone();
        for j in i + 1..len {
            let word2 = words[j].clone();
            if are_anagrams(&word1, &word2) {
                if let Some(v) = anagrams.get(&word1.len()) {
                    for anagrame in v {
                        let mut decode: HashMap<char, usize> = HashMap::new();
                        for (n, c) in word1.chars().enumerate() {
                            decode.insert(c, anagrame[n]);
                        }

                        let mut v = Vec::new();
                        for c in word2.chars() {
                            if let Some(d) = decode.get(&c) {
                                v.push(*d)
                            }
                        }

                        if v[0] != 0 {
                            let n1 = conversion(&v, 10);
                            let n2 = conversion(anagrame, 10);
                            if n1.is_square() {
                                println!("({word1}, {word2}) ==> {n1} {n2}");
                                result = std::cmp::max(result, n1);
                                result = std::cmp::max(result, n2);
                            }
                        }
                    }
                }
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem098() {
        let result = problem098();
        assert_eq!(result, "18769");
    }
}
