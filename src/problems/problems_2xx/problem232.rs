use crate::register_problem;
use std::collections::HashMap;

register_problem!(232, "The Race", problem232);

fn the_game(mut cache: &mut HashMap<(i64, i64), f64>, n: i64, k: i64) -> f64 {
    if k <= 0 {
        return 1.;
    } else if k > 0 && n <= 0 {
        return 0.;
    }

    if let Some(&value) = cache.get(&(n, k)) {
        return value;
    }

    let mut strategies: Vec<i64> = vec![2];
    while let Some(&back) = strategies.last()
        && back < 2 * k
    {
        strategies.push(2 * back);
    }

    let mut probability = 0.;
    for t in strategies {
        let p1_win = 1. / 2.;
        let p2_win = 1. / t as f64;
        let p1_lose = 1. - p1_win;
        let p2_lose = 1. - p2_win;

        let win = t / 2;

        let p = (p1_win * p2_win * the_game(&mut cache, n - 1, k - win)
            + p1_win * p2_lose * the_game(&mut cache, n - 1, k)
            + p1_lose * p2_win * the_game(&mut cache, n, k - win))
            / (1. - p1_lose * p2_lose);

        if p > probability {
            probability = p;
        }
    }

    cache.insert((n, k), probability);
    probability
}

pub fn problem232() -> String {
    // Two players share an unbiased coin and take it in turns to play "The Race". On Player 1's turn, he tosses the
    // coin once: if it comes up Heads, he scores one point; if it comes up Tails, he scores nothing. On Player 2's
    // turn, she chooses a positive integer T and tosses the coin T times: if it comes up all Heads, she scores 2T-1
    // points; otherwise, she scores nothing. Player 1 goes first. The winner is the first to 100 or more points.
    //
    // On each turn Player 2 selects the number, T, of coin tosses that maximises the probability of her winning.
    //
    // What is the probability that Player 2 wins?
    //
    // Give your answer rounded to eight decimal places in the form 0.abcdefgh .
    let mut cache = HashMap::new();
    let result = 0.5 * the_game(&mut cache, 100, 100) + 0.5 * the_game(&mut cache, 99, 100);

    format!("{:.8}", result)
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem232::problem232;

    #[test]
    fn test_problem232() {
        let result = problem232();
        assert_eq!(result, "0.83648556");
    }
}
