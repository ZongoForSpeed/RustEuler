use std::collections::{HashMap, HashSet};
use crate::register_problem;

register_problem!(109, "Darts", problem109);

type Dart = (String, u32);
type Darts = Vec<Dart>;
type Combinations = HashMap<u32, HashSet<Darts>>;

fn build(score: &Darts, combinaisons: &Combinations) -> Combinations {
    let mut result = combinaisons.clone();
    for s in score {
        for (first, second) in combinaisons {
            for f in second {
                let mut copy_f = f.clone();
                copy_f.push(s.clone());
                copy_f.sort();
                result.entry(s.1 + first).or_insert_with(HashSet::new).insert(copy_f);
            }
        }
    }
    result
}

fn dart(t: char, score: u32, value: u32) -> Dart {
    (format!("{t}{score}"), value)
}

pub fn problem109() -> String {
    // In the game of darts a player throws three darts at a target board which is split into twenty equal sized
    // sections numbered one to twenty.
    //
    // The score of a dart is determined by the number of the region that the dart lands in. A dart landing outside the
    // red/green outer ring scores zero. The black and cream regions inside this
    // ring represent single scores. However, the red/green outer ring and middle ring score double and treble scores
    // respectively.
    //
    // At the centre of the board are two concentric circles called the bull region, or bulls-eye.
    // The outer bull is worth 25 points and the inner bull is a double, worth 50 points.
    //
    // There are many variations of rules but in the most popular game the players will begin with a score 301 or 501
    // and the first player to reduce their running total to zero is a winner.
    // However, it is normal to play a "doubles out" system, which means that the player must land a double (including
    // the double bulls-eye at the centre of the board) on their final dart to win;
    // any other dart that would reduce their running total to one or lower means the score for that set of three darts
    // is "bust".
    //
    // When a player is able to finish on their current score it is called a "checkout" and the highest checkout is 170:
    // T20 T20 D25 (two treble 20s and double bull).
    //
    // There are exactly eleven distinct ways to checkout on a score of 6:
    //
    //      D3
    //      D1	D2
    //      S2	D2
    //      D2	D1
    //      S4	D1
    //      S1	S1	D2
    //      S1	T1	D1
    //      S1	S3	D1
    //      D1	D1	D1
    //      D1	S2	D1
    //      S2	S2	D1
    //
    // Note that D1 D2 is considered different to D2 D1 as they finish on different doubles. However, the combination
    // S1 T1 D1 is considered the same as T1 S1 D1.
    //
    // In addition we shall not include misses in considering combinations; for example, D3 is the same as 0 D3 and
    // 0 0 D3.
    //
    // Incredibly there are 42336 distinct ways of checking out in total.
    //
    // How many distinct ways can a player checkout with a score less than 100?
    let mut score:Darts = Darts::new();
    let mut score_double:Darts = Darts::new();

    for n in 1..=20 {
        score.push(dart('S', n, n));
        score.push(dart('D', n, 2*n));
        score.push(dart('T', n, 3*n));

        score_double.push(dart('D', n, 2*n));
    }

    score.push(dart('S', 25, 25));
    score.push(dart('D', 25, 2*25));

    score_double.push(dart('D', 25, 2*25));

    let zero: Darts = vec![("0".into(), 0)];

    let mut c: Combinations = Combinations::new();
    c.entry(0).or_insert_with(HashSet::new).insert(zero);

    let c = build(&score, &c);
    println!("c = {:?}", c.len());
    let c = build(&score, &c);
    println!("c = {:?}", c.len());
    let mut solution = Combinations::new();
    for s in score_double {
        for (first, second) in &c {
            for f in second {
                let mut copy_f = f.clone();
                copy_f.push(s.clone());
                solution.entry(s.1 + first).or_insert_with(HashSet::new).insert(copy_f);
            }
        }
    }
    println!("solution = {:?}", solution.len());

    let mut result = 0;
    for (first, second) in solution {
        if first < 100 {
            result += second.len();
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem109() {
        let result = problem109();
        assert_eq!(result, "38182");
    }
}
