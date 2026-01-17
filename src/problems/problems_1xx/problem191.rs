use crate::register_problem;
use std::collections::HashMap;

register_problem!(191, "Prize Strings", problem191);

type Key = (u64, u64, bool);
type Cache = HashMap<Key, u64>;

fn sequence(mut cache: &mut Cache, n: u64, absent: u64, late: bool) -> u64 {
    if n == 0 {
        return 1;
    }
    let key = (n, absent, late);
    if let Some(&value) = cache.get(&key) {
        return value;
    }

    let mut result = sequence(&mut cache, n - 1, 0, late);
    if !late {
        result += sequence(&mut cache, n - 1, 0, true);
    }

    if absent < 2 {
        result += sequence(&mut cache, n - 1, absent + 1, late);
    }

    cache.insert(key, result);
    result
}

pub fn problem191() -> String {
    // A particular school offers cash rewards to children with good attendance and punctuality.
    // If they are absent for three consecutive days or late on more than one occasion then they
    // forfeit their prize.
    //
    // During an n-day period a trinary string is formed for each child consisting of L's (late),
    // O's (on time), and A's (absent).
    //
    // Although there are eighty-one trinary strings for a 4-day period that can be formed, exactly
    // forty-three strings would lead to a prize:
    //
    //       OOOO OOOA OOOL OOAO OOAA OOAL OOLO OOLA OAOO OAOA
    //       OAOL OAAO OAAL OALO OALA OLOO OLOA OLAO OLAA AOOO
    //       AOOA AOOL AOAO AOAA AOAL AOLO AOLA AAOO AAOA AAOL
    //       AALO AALA ALOO ALOA ALAO ALAA LOOO LOOA LOAO LOAA
    //       LAOO LAOA LAAO
    //
    // How many "prize" strings exist over a 30-day period?
    let mut cache = Cache::new();
    let result = sequence(&mut cache, 30, 0, false);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem191() {
        let result = problem191();
        assert_eq!(result, "1918080160");
    }
}
