use crate::maths::arithmetic::Arithmetic;
use crate::maths::power::Power;
use crate::register_problem;

register_problem!(
    176,
    "Right-angled triangles that share a cathetus",
    problem176
);

pub fn problem176() -> String {
    // The four right-angled triangles with sides (9,12,15), (12,16,20), (5,12,13) and (12,35,37)
    // all have one of the shorter sides (catheti) equal to 12. It can be shown that no other
    // integer sided right-angled triangle exists with one of the catheti equal to 12.
    //
    // Find the smallest integer that can be the length of a cathetus of exactly 47547 different
    // integer sided right-angled triangles.
    let primes: Vec<u64> = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
    ];
    // https://oeis.org/A046079
    // Let n = (2^a0)*(p1^a1)*...*(pk^ak).
    // Then a(n) = [(2*a0 - 1)*(2*a1 + 1)*(2*a2 + 1)*(2*a3 + 1)*...*(2*ak + 1) - 1]/2.
    // Note that if there is no a0 term, i.e. if n is odd, then the first term is simply omitted.
    let objective = 47547;

    let mut factors = Vec::new();
    (objective * 2 + 1).factorization(&primes, |p, _| factors.push(p / 2));

    factors.sort_by(|a, b| b.cmp(a));
    factors[0] += 1;

    let mut result = 1;
    for (n, f) in factors.iter().enumerate() {
        result *= u64::power(primes[n], *f);
    }

    result.to_string()
}
