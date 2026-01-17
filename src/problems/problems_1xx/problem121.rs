use crate::register_problem;
use crate::utils::permutations::permutations;
use fraction::Fraction;
use num_traits::{ConstOne, ConstZero};

register_problem!(121, "Disc game prize fund", problem121);

pub fn problem121() -> String {
    // A bag contains one red disc and one blue disc. In a game of chance a player takes a disc at
    // random and its colour is noted. After each turn the disc is returned to the bag, an extra red
    // disc is added, and another disc is taken at random.
    //
    // The player pays £1 to play and wins if they have taken more blue discs than red discs at the
    // end of the game.
    //
    // If the game is played for four turns, the probability of a player winning is exactly 11/120,
    // and so the maximum prize fund the banker should allocate for winning in this game would be £10
    // before they would expect to incur a loss. Note that any payout will be a whole number of pounds
    // and also includes the original £1 paid to play the game, so in the example given the player
    // actually wins £9.
    //
    // Find the maximum prize fund that should be allocated to a single game in which fifteen turns are
    // played.
    let limit = 15;
    let mut probabilities: Vec<Fraction> = Vec::new();
    for n in 1..=limit {
        probabilities.push(Fraction::new(1u64, (n + 1) as u64));
    }

    let mut probability = Fraction::ZERO;
    for blue in limit / 2 + 1..limit + 1 {
        let mut possibilities = vec![false; limit - blue];
        possibilities.extend(vec![true; blue]);
        for permute in permutations(possibilities) {
            let mut p = Fraction::ONE;
            for n in 0..limit {
                if permute[n] {
                    p *= probabilities[n];
                } else {
                    p *= Fraction::ONE - probabilities[n];
                }
            }
            probability += p;
        }
    }

    let result = probability.denom().unwrap() / probability.numer().unwrap();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem121() {
        let result = problem121();
        assert_eq!(result, "2269");
    }
}
