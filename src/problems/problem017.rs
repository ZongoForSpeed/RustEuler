use crate::maths::timer::ScopeTimer;

pub fn problem017() -> u64 {
    let _timer = ScopeTimer::new("Problem 17 Number letter counts", false);
    // If the numbers 1 to 5 are written out in words: one, two, three, four, five,
    // then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
    //
    // If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words,
    // how many letters would be used?
    //
    // NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two)
    // contains 23 letters and 115 (one hundred and fifteen) contains 20 letters.
    // The use of},and" when writing out numbers is in compliance with British usage.
    let unite = vec![0, 3, 3, 5, 4, 4, 3, 5, 5, 4, 3, 6, 6, 8, 8, 7, 7, 9, 8, 8];
    let dizaine = vec![0, 3, 6, 6, 5, 5, 5, 7, 6, 6];
    let centaine = 7; // hundred
    let et = 3; // and

    let mut resultat = 11; // onethousand
    for n in 1..1000 {
        if n % 100 != 0 {
            let c = n % 100;
            if c < 20 {
                resultat += unite[c];
            }            else {
                resultat += dizaine[c / 10] + unite[c % 10];
            }
        }
        if n >= 100 {
            resultat += unite[n / 100] + centaine;
            if n % 100 != 0 {
                resultat += et;
            }
        }
    }
    resultat
}
