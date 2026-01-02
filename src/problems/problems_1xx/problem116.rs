use crate::register_problem;

register_problem!(116, "Red, green or blue tiles", problem116);

pub fn problem116() -> String {
    // A row of five black square tiles is to have a number of its tiles replaced with coloured oblong tiles chosen from
    // red (length two), green (length three), or blue (length four).
    //
    // If red tiles are chosen there are exactly seven ways this can be done.
    // If green tiles are chosen there are three ways.
    // And if blue tiles are chosen there are two ways.
    //
    // Assuming that colours cannot be mixed there are 7 + 3 + 2 = 12 ways of replacing the black tiles in a row
    // measuring five units in length.
    //
    // How many different ways can the black tiles in a row measuring fifty units in length be replaced if colours
    // cannot be mixed and at least one coloured tile must be used?
    //
    // NOTE: This is related to Problem 117.
    let longueur = 50;
    let mut resultat: u64 = 0;
    for color in 2..5 {
        let mut values = vec![0; color];

        for (j, base) in (color..=longueur).zip(1..) {
            values.push(values[j - 1] + base + values[j - color]);
        }
        resultat += values[longueur] - values[longueur - 1];
    }

    resultat.to_string()
}
