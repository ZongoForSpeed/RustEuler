use crate::register_problem;

register_problem!(227, "The Chase", problem227);

pub fn problem227() -> String {
    // "The Chase" is a game played with two dice and an even number of players.
    //
    // The players sit around a table; the game begins with two opposite players having one die each.
    // On each turn, the two players with a die roll it.
    // If a player rolls a 1, he passes the die to his neighbour on the left; if he rolls a 6, he passes the die to his
    // neighbour on the right; otherwise, he keeps the die for the next turn. The game ends when one player has both
    // dice after they have been rolled and passed; that player has then lost.
    //
    // In a game with 100 players, what is the expected number of turns the game lasts?
    //
    // Give your answer rounded to ten significant digits.
    let mut chase = vec![0.; 51];
    chase[50] = 1.;

    let mut turns = 0;
    let mut ending = 0.;
    let mut expected = 0.;

    let error = f64::powi(0.1, 12);

    while ending < 1. - error {
        chase[0] = 0.;
        turns += 1;
        let mut nouveau = vec![0.; 51];

        for (n, c) in chase.iter().enumerate() {
            match n {
                0 => {}
                1 => {
                    nouveau[0] += c * 2. / 9.;
                    nouveau[1] += c * 19. / 36.;
                    nouveau[2] += c * 2. / 9.;
                    nouveau[3] += c / 36.;
                }
                49 => {
                    nouveau[47] += c / 36.;
                    nouveau[48] += c * 2. / 9.;
                    nouveau[49] += c * 19. / 36.;
                    nouveau[50] += c * 2. / 9.;
                }
                50 => {
                    nouveau[48] += c / 18.;
                    nouveau[49] += c * 4. / 9.;
                    nouveau[50] += c / 2.;
                }
                _ => {
                    nouveau[n - 2] += c / 36.;
                    nouveau[n - 1] += c * 2. / 9.;
                    nouveau[n] += c / 2.;
                    nouveau[n + 1] += c * 2. / 9.;
                    nouveau[n + 2] += c / 36.;
                }
            }
        }

        chase = nouveau;

        ending += chase[0];
        expected += turns as f64 * chase[0];
    }

    format!("{:.6}", expected)
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem227::problem227;

    #[test]
    fn test_problem227() {
        let result = problem227();
        assert_eq!(result, "3780.618622");
    }
}
