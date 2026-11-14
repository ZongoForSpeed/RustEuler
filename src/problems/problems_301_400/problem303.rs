use crate::register_problem;

register_problem!(303, "Multiples with small digits", problem303);

fn trinary(mut n: u64) -> bool {
    while n % 10 < 3 && n > 10 {
        n /= 10;
    }

    n % 10 < 3
}

/// Renvoie m tel que m·n est le plus petit multiple de n dont les chiffres sont tous ≤ 2.
fn f(n: u64) -> u64 {
    let mut base = 1;
    let mut v = vec![0];
    loop {
        let mut tmp = Vec::new();
        for i in 1..11 {
            for &fv in &v {
                let m = i * base + fv;
                let mn = m * n;
                if trinary(mn) {
                    return m;
                } else if trinary(mn % base) {
                    tmp.push(m);
                }
            }
        }

        base *= 10;
        tmp.sort();
        v = tmp;
    }
}

/// For a positive integer n, define f(n) as the least positive multiple of n that, written in base 10, uses only
/// digits ≤ 2.
///
/// Thus f(2)=2, f(3)=12, f(7)=21, f(42)=210, f(89)=1121222.
/// ∑
/// Also, ∑ n=1..100 f(n)/n = 11363107
///
/// Find ∑ n=1..10000 f(n)/n.
pub fn problem303() -> String {
    let limite: u64 = 10000;
    let mut resultat = 0;
    for n in 1..limite + 1 {
        resultat += f(n);
    }

    resultat.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem303() {
        assert_eq!(problem303(), "1111981904675169");
    }
}
