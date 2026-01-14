use crate::register_problem;
use bit_set::BitSet;

register_problem!(229, "Four Representations using Squares", problem229);

pub fn problem229() -> String {
    // Consider the number 3600. It is very special, because
    //
    // 				3600 = 48² +   36²
    //				3600 = 20² + 2×40²
    //				3600 = 30² + 3×30²
    //				3600 = 45² + 7×15²
    //
    // Similarly, we find that 88201 = 99² + 280² = 287² + 2×54² = 283² + 3×52² = 1972 + 7×84².
    //
    // In 1747, Euler proved which numbers are representable as a sum of two squares. We are interested in the numbers n
    // which admit representations of all of the following four types:
    //
    //				n = a1² +   b1²
    //				n = a2² + 2 b2²
    //				n = a3² + 3 b3²
    //				n = a7² + 7 b7²,
    //
    // where the ak and bk are positive integers.
    //
    // There are 75373 such numbers that do not exceed 10^7.
    // How many such numbers are there that do not exceed 2×10^9?
    let limit = 2000000000;
    let mut type1 = BitSet::with_capacity(limit + 1);
    let mut type2 = BitSet::with_capacity(limit + 1);
    let mut type3 = BitSet::with_capacity(limit + 1);
    let mut type7 = BitSet::with_capacity(limit + 1);

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.install(|| fill(limit, &mut type1, &mut type2, &mut type3, &mut type7));

    type1.intersect_with(&type2);
    type1.intersect_with(&type3);
    type1.intersect_with(&type7);

    type1.len().to_string()
}

fn fill(
    taille: usize,
    mut type1: &mut BitSet,
    mut type2: &mut BitSet,
    mut type3: &mut BitSet,
    mut type7: &mut BitSet,
) {
    rayon::scope(|s| {
        s.spawn(|_| fill_type(taille, &mut type1, 1));
        s.spawn(|_| fill_type(taille, &mut type2, 2));
        s.spawn(|_| fill_type(taille, &mut type3, 3));
        s.spawn(|_| fill_type(taille, &mut type7, 7));
    });
}

fn fill_type(limit: usize, shape: &mut BitSet, k: usize) {
    for x in 1.. {
        if x * x > limit {
            break;
        }
        for y in 1.. {
            let n = x * x + k * y * y;
            if n > limit {
                break;
            }
            shape.insert(n);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem229::problem229;

    #[test]
    fn test_problem229() {
        let result = problem229();
        assert_eq!(result, "11325263");
    }
}
