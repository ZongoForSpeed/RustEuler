use crate::register_problem;
use fraction::Fraction;
use std::collections::HashSet;

register_problem!(155, "Counting Capacitor Circuits", problem155);

pub fn problem155() -> String {
    // An electric circuit uses exclusively identical capacitors of the same value C.
    //
    // The capacitors can be connected in series or in parallel to form sub-units, which can then be connected in series
    // or in parallel with other capacitors or other sub-units to form larger sub-units, and so on up to a final circuit.
    //
    // Using this simple procedure and up to n identical capacitors, we can make circuits having a range of different
    // total capacitances. For example, using up to n=3 capacitors of 60 F each, we can obtain the following 7 distinct
    // total capacitance values:
    //
    // If we denote by D(n) the number of distinct total capacitance values we can obtain when using up to n
    // equal-valued capacitors and the simple procedure described above, we have: D(1)=1,
    // D(2)=3, D(3)=7 ...
    //
    // Find D(18).
    //
    // Reminder : When connecting capacitors C1, C2 etc in parallel, the total capacitance is
    //                      CT = C1 + C2 +...,
    // whereas when connecting them in series, the overall capacitance is given by:
    //                      1/CT = 1/C1 + 1/C2 +...
    let mut capacitors: Vec<HashSet<Fraction>> = vec![HashSet::new(); 20];
    capacitors[1].insert(1.into());

    let mut result = HashSet::new();
    result.insert(1.into());

    for n in 2..19 {
        let mut cn = HashSet::new();
        for i in 1..=n / 2 {
            let j = n - i;
            let ci = &capacitors[i];
            let cj = &capacitors[j];
            for vi in ci {
                let ai = vi.numer().unwrap();
                let bi = vi.denom().unwrap();
                for vj in cj {
                    let aj = vj.numer().unwrap();
                    let bj = vj.denom().unwrap();

                    let parallel = vi + vj;
                    if result.insert(parallel) {
                        cn.insert(parallel);
                    }

                    let series = Fraction::new(ai * aj, ai * bj + aj * bi);
                    if result.insert(series) {
                        cn.insert(series);
                    }
                }
            }
        }

        println!("D({n}) = {}", result.len());
        capacitors[n] = cn;
    }

    result.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem155() {
        let result = problem155();
        assert_eq!(result, "3857447");
    }
}
