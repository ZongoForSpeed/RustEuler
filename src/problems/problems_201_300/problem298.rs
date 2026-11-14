use crate::register_problem;
use std::collections::{HashMap, VecDeque};

register_problem!(298, "Selective Amnesia", problem298);

const MEMOIRE: usize = 5;

#[derive(Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Etat {
    memoire_larry: VecDeque<u16>,
    memoire_robin: VecDeque<u16>,
}

impl Etat {
    /// Renvoie le score du tirage (+1 pour Larry, -1 pour Robin) et met à jour les mémoires.
    fn suivant(&mut self, tirage: u16) -> i16 {
        let mut score = 0;

        // Larry retire le nombre qui n'a pas été appelé depuis le plus longtemps.
        if let Some(position) = self.memoire_larry.iter().position(|&n| n == tirage) {
            score += 1;
            self.memoire_larry.remove(position);
            self.memoire_larry.push_front(tirage);
        } else {
            self.memoire_larry.push_front(tirage);
            if self.memoire_larry.len() > MEMOIRE {
                self.memoire_larry.pop_back();
            }
        }

        // Robin retire le nombre qui est en mémoire depuis le plus longtemps.
        if self.memoire_robin.contains(&tirage) {
            score -= 1;
        } else {
            self.memoire_robin.push_front(tirage);
            if self.memoire_robin.len() > MEMOIRE {
                self.memoire_robin.pop_back();
            }
        }

        // renumérote les nombres par ordre d'apparition, pour identifier les états équivalents
        let mut echange: HashMap<u16, u16> = HashMap::new();
        let mut compteur = 0;
        for n in self
            .memoire_robin
            .iter_mut()
            .chain(self.memoire_larry.iter_mut())
        {
            let nouveau = *echange.entry(*n).or_insert_with(|| {
                compteur += 1;
                compteur
            });
            *n = nouveau;
        }

        score
    }
}

/// Larry and Robin play a memory game involving of a sequence of random numbers between 1 and 10, inclusive, that
/// are called out one at a time.
/// Each player can remember up to 5 previous numbers. When the called number is in a player's memory, that player is
/// awarded a point. If it's not, the player adds the called number to his memory, removing another number if his
/// memory is full.
///
/// Both players start with empty memories. Both players always add new missed numbers to their memory but use a
/// different strategy in deciding which number to remove:
///     - Larry's strategy is to remove the number that hasn't been called in the longest time.
///     - Robin's strategy is to remove the number that's been in the memory the longest time.
///
/// Example game:
///
///     Turn	Called 	    Larry's Larry's     Robin's Robin's
///             number      memory  score       memory  score
///        1	     1	          1	      0	          1	      0
///        2	     2	        1,2	      0	        1,2	      0
///        3	     4	      1,2,4	      0	      1,2,4	      0
///        4	     6	    1,2,4,6	      0	    1,2,4,6	      0
///        5	     1	    1,2,4,6	      1	    1,2,4,6	      1
///        6	     8	  1,2,4,6,8	      1	  1,2,4,6,8	      1
///        7	    10	 1,4,6,8,10	      1	 2,4,6,8,10	      1
///        8	     2	 1,2,6,8,10	      1	 2,4,6,8,10	      2
///        9	     4	 1,2,4,8,10	      1	 2,4,6,8,10	      3
///       10	     1	 1,2,4,8,10	      2	 1,4,6,8,10	      3
///
/// Denoting Larry's score by L and Robin's score by R, what is the expected value of |L-R| after 50 turns? Give your
/// answer rounded to eight decimal places using the format x.xxxxxxxx.
pub fn problem298() -> String {
    // Construit le graphe des états : graphe[etat] = liste de (état suivant, score, probabilité)
    let mut etats: Vec<Etat> = vec![Etat::default()];
    let mut indices: HashMap<Etat, usize> = HashMap::from([(Etat::default(), 0)]);
    let mut graphe: Vec<Vec<(usize, i16, f64)>> = Vec::new();

    let mut i = 0;
    while i < etats.len() {
        let mut suivants: Vec<(usize, i16, f64)> = Vec::new();

        for tirage in 0..10 {
            let mut nouvel_etat = etats[i].clone();
            let score = nouvel_etat.suivant(tirage);

            let suivant = match indices.get(&nouvel_etat) {
                Some(&index) => index,
                None => {
                    let index = etats.len();
                    indices.insert(nouvel_etat.clone(), index);
                    etats.push(nouvel_etat);
                    index
                }
            };

            match suivants
                .iter_mut()
                .find(|(etat, s, _)| *etat == suivant && *s == score)
            {
                Some((_, _, probabilite)) => *probabilite += 0.1,
                None => suivants.push((suivant, score, 0.1)),
            }
        }

        graphe.push(suivants);
        i += 1;
    }

    // Programmation dynamique : dp[etat] = distribution des scores après chaque tour
    let mut dp: Vec<HashMap<i16, f64>> = vec![HashMap::new(); etats.len()];
    dp[0].insert(0, 1.0);

    for _ in 0..50 {
        let mut suivant: Vec<HashMap<i16, f64>> = vec![HashMap::new(); etats.len()];
        for etat in 0..etats.len() {
            for (&score, &probabilite) in &dp[etat] {
                for &(e, s, p) in &graphe[etat] {
                    *suivant[e].entry(score + s).or_insert(0.0) += p * probabilite;
                }
            }
        }
        dp = suivant;
    }

    let mut resultat = 0.0;
    for distribution in &dp {
        for (&score, &probabilite) in distribution {
            resultat += (score.abs() as f64) * probabilite;
        }
    }

    format!("{:.8}", resultat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem298() {
        assert_eq!(problem298(), "1.76882294");
    }
}
