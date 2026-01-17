use crate::maths::digits::Digits;
use crate::register_problem;

register_problem!(238, "Infinite string tour", problem238);

pub fn problem238() -> String {
    // Create a sequence of numbers using the "Blum Blum Shub" pseudo-random number generator:
    //
    //              s0      =	14025256
    //              sn+1	=	sn² mod 20300713
    //
    // Concatenate these numbers  s0s1s2… to create a string w of infinite length.
    // Then, w = 14025256741014958470038053646…
    //
    // For a positive integer k, if no substring of w exists with a sum of digits equal to k, p(k)
    // is defined to be zero. If at least one substring of w exists with a sum of digits equal to k,
    // we define p(k) = z, where z is the starting position of the earliest such substring.
    //
    // For instance:
    //
    // The substrings 1, 14, 1402, …
    // with respective sums of digits equal to 1, 5, 7, …
    // start at position 1, hence p(1) = p(5) = p(7) = … = 1.
    //
    // The substrings 4, 402, 4025, …
    // with respective sums of digits equal to 4, 6, 11, …
    // start at position 2, hence p(4) = p(6) = p(11) = … = 2.
    //
    // The substrings 02, 0252, …
    // with respective sums of digits equal to 2, 9, …
    // start at position 3, hence p(2) = p(9) = … = 3.
    //
    // Note that substring 025 starting at position 3, has a sum of digits equal to 7, but there was
    // an earlier substring (starting at position 1) with a sum of digits equal to 7, so p(7) = 1,
    // not 3.
    //
    // We can verify that, for 0 < k ≤ 10^3, ∑ p(k) = 4742.
    //
    // Find ∑ p(k), for 0 < k ≤ 2·10^15.
    let s0: usize = 14_025_256;
    let modulo: usize = 20_300_713;
    let limite: usize = 2_000_000_000_000_000;

    let mut sn = s0;
    let mut w: Vec<usize> = Vec::new();
    loop {
        let digits = sn.extract_digits(10);
        for d in digits {
            w.push(d);
        }
        sn = (sn * sn) % modulo;
        if sn == s0 {
            break;
        }
    }

    let somme: usize = w.iter().sum();

    let mut p = vec![0; somme + 1];
    let mut compteur = 0;
    for i in 0..w.len() {
        if compteur > somme {
            break;
        }
        let mut r#as = 0;
        for j in 0..w.len() {
            r#as += w[(j + i) % w.len()];
            if compteur > somme {
                break;
            }
            if p[r#as] == 0 {
                p[r#as] = i + 1;
                compteur += 1;
            }

        }
    }

    let mut resultat = limite / somme;
    for x in 1..somme {
        resultat += p[x] * (1 + ((limite - x) / somme));
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem238::problem238;

    #[test]
    fn test_problem238() {
        let result = problem238();
        assert_eq!(result, "9922545104535661");
    }
}
