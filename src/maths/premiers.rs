use bit_set::BitSet;
use num_traits::PrimInt;

pub(crate) fn crible2<T, F>(taille: usize, mut sortie: F)
where
    T: PrimInt + std::ops::DivAssign,
    F: FnMut(T),
{
    let taille_crible = taille / 2;
    let mut test: BitSet<usize> = BitSet::from_iter(1..taille_crible);
    let mut p = 1;
    while p * p < taille_crible / 2 {
        if test.contains(p) {
            let start = 2 * (p * p + p);
            set_test(&mut test, start, taille_crible, 2 * p + 1);
        }

        p += 1;
    }

    sortie(T::from(2).unwrap());
    for p in test.iter() {
        sortie(T::from(2 * p + 1).unwrap());
    }
}

fn set_test(test: &mut BitSet<usize>, start: usize, end: usize, step: usize) {
    for n in (start..end).step_by(step) {
        test.remove(n);
    }
}

pub(crate) fn crible23<T, F>(taille: usize, mut sortie: F)
where
    T: PrimInt + std::ops::DivAssign,
    F: FnMut(T),
{
    let taille_crible = taille / 6 + 1;
    let mut test1: BitSet<usize> = BitSet::from_iter(1..taille_crible);
    let mut test5: BitSet<usize> = BitSet::from_iter(0..taille_crible);
    let mut k = 0;
    while 6 * k * k < taille_crible {
        if test1.contains(k) {
            let p = 6 * k + 1;
            set_test(&mut test1, 6 * k * k + 2 * k, taille_crible, p);
            set_test(&mut test5, 6 * k * k + 6 * k, taille_crible, p);
        }
        if test5.contains(k) {
            let p = 6 * k + 5;
            set_test(&mut test1, 6 * k * k + 10 * k + 4, taille_crible, p);
            set_test(&mut test5, 6 * k * k + 12 * k + 5, taille_crible, p);
        }
        k += 1;
    }

    sortie(T::from(2).unwrap());
    sortie(T::from(3).unwrap());
    for p in 0..taille_crible {
        if test1.contains(p) {
            sortie(T::from(6 * p + 1).unwrap());
        }
        if test5.contains(p) {
            sortie(T::from(6 * p + 5).unwrap());
        }
    }
}

pub(crate) fn crible235<T, F>(taille: usize, mut sortie: F)
where
    T: PrimInt + std::ops::DivAssign,
    F: FnMut(T),
{
    let taille_crible = taille / 30 + 1;
    let mut test1: BitSet<usize> = BitSet::from_iter(1..taille_crible);
    let mut test7: BitSet<usize> = BitSet::from_iter(0..taille_crible);
    let mut test11: BitSet<usize> = BitSet::from_iter(0..taille_crible);
    let mut test13: BitSet<usize> = BitSet::from_iter(0..taille_crible);
    let mut test17: BitSet<usize> = BitSet::from_iter(0..taille_crible);
    let mut test19: BitSet<usize> = BitSet::from_iter(0..taille_crible);
    let mut test23: BitSet<usize> = BitSet::from_iter(0..taille_crible);
    let mut test29: BitSet<usize> = BitSet::from_iter(0..taille_crible);

    let mut k = 0;
    while 30 * k * k < taille_crible {
        if test1.contains(k) {
            let p = 30 * k + 1;
            set_test(&mut test1, 30 * k * k + 2 * k + 0, taille, p);
            set_test(&mut test7, 30 * k * k + 8 * k + 0, taille, p);
            set_test(&mut test11, 30 * k * k + 12 * k + 0, taille, p);
            set_test(&mut test13, 30 * k * k + 14 * k + 0, taille, p);
            set_test(&mut test17, 30 * k * k + 18 * k + 0, taille, p);
            set_test(&mut test19, 30 * k * k + 20 * k + 0, taille, p);
            set_test(&mut test23, 30 * k * k + 24 * k + 0, taille, p);
            set_test(&mut test29, 30 * k * k + 30 * k + 0, taille, p);
        }
        if test7.contains(k) {
            let p = 30 * k + 7;

            set_test(&mut test19, 30 * k * k + 14 * k + 1, taille, p);
            set_test(&mut test17, 30 * k * k + 18 * k + 2, taille, p);
            set_test(&mut test1, 30 * k * k + 20 * k + 3, taille, p);
            set_test(&mut test29, 30 * k * k + 24 * k + 3, taille, p);
            set_test(&mut test13, 30 * k * k + 26 * k + 4, taille, p);
            set_test(&mut test11, 30 * k * k + 30 * k + 5, taille, p);
            set_test(&mut test23, 30 * k * k + 36 * k + 6, taille, p);
            set_test(&mut test7, 30 * k * k + 38 * k + 7, taille, p);
        }
        if test11.contains(k) {
            let p = 30 * k + 11;

            set_test(&mut test1, 30 * k * k + 22 * k + 4, taille, p);
            set_test(&mut test23, 30 * k * k + 24 * k + 4, taille, p);
            set_test(&mut test7, 30 * k * k + 28 * k + 6, taille, p);
            set_test(&mut test29, 30 * k * k + 30 * k + 6, taille, p);
            set_test(&mut test13, 30 * k * k + 34 * k + 8, taille, p);
            set_test(&mut test19, 30 * k * k + 40 * k + 10, taille, p);
            set_test(&mut test11, 30 * k * k + 42 * k + 11, taille, p);
            set_test(&mut test17, 30 * k * k + 48 * k + 13, taille, p);
        }
        if test13.contains(k) {
            let p = 30 * k + 13;

            set_test(&mut test19, 30 * k * k + 26 * k + 5, taille, p);
            set_test(&mut test11, 30 * k * k + 30 * k + 7, taille, p);
            set_test(&mut test7, 30 * k * k + 32 * k + 8, taille, p);
            set_test(&mut test29, 30 * k * k + 36 * k + 9, taille, p);
            set_test(&mut test17, 30 * k * k + 42 * k + 12, taille, p);
            set_test(&mut test13, 30 * k * k + 44 * k + 13, taille, p);
            set_test(&mut test1, 30 * k * k + 50 * k + 16, taille, p);
            set_test(&mut test23, 30 * k * k + 54 * k + 17, taille, p);
        }
        if test17.contains(k) {
            let p = 30 * k + 17;

            set_test(&mut test19, 30 * k * k + 34 * k + 9, taille, p);
            set_test(&mut test23, 30 * k * k + 36 * k + 10, taille, p);
            set_test(&mut test1, 30 * k * k + 40 * k + 13, taille, p);
            set_test(&mut test13, 30 * k * k + 46 * k + 16, taille, p);
            set_test(&mut test17, 30 * k * k + 48 * k + 17, taille, p);
            set_test(&mut test29, 30 * k * k + 54 * k + 20, taille, p);
            set_test(&mut test7, 30 * k * k + 58 * k + 23, taille, p);
            set_test(&mut test11, 30 * k * k + 60 * k + 24, taille, p);
        }
        if test19.contains(k) {
            let p = 30 * k + 19;

            set_test(&mut test1, 30 * k * k + 38 * k + 12, taille, p);
            set_test(&mut test17, 30 * k * k + 42 * k + 14, taille, p);
            set_test(&mut test11, 30 * k * k + 48 * k + 18, taille, p);
            set_test(&mut test19, 30 * k * k + 50 * k + 19, taille, p);
            set_test(&mut test13, 30 * k * k + 56 * k + 23, taille, p);
            set_test(&mut test29, 30 * k * k + 60 * k + 25, taille, p);
            set_test(&mut test7, 30 * k * k + 62 * k + 27, taille, p);
            set_test(&mut test23, 30 * k * k + 66 * k + 29, taille, p);
        }
        if test23.contains(k) {
            let p = 30 * k + 23;

            set_test(&mut test19, 30 * k * k + 46 * k + 17, taille, p);
            set_test(&mut test7, 30 * k * k + 52 * k + 22, taille, p);
            set_test(&mut test23, 30 * k * k + 54 * k + 23, taille, p);
            set_test(&mut test11, 30 * k * k + 60 * k + 28, taille, p);
            set_test(&mut test13, 30 * k * k + 64 * k + 31, taille, p);
            set_test(&mut test29, 30 * k * k + 66 * k + 32, taille, p);
            set_test(&mut test1, 30 * k * k + 70 * k + 36, taille, p);
            set_test(&mut test17, 30 * k * k + 72 * k + 37, taille, p);
        }
        if test29.contains(k) {
            let p = 30 * k + 29;

            set_test(&mut test1, 30 * k * k + 58 * k + 28, taille, p);
            set_test(&mut test29, 30 * k * k + 60 * k + 29, taille, p);
            set_test(&mut test23, 30 * k * k + 66 * k + 35, taille, p);
            set_test(&mut test19, 30 * k * k + 70 * k + 39, taille, p);
            set_test(&mut test17, 30 * k * k + 72 * k + 41, taille, p);
            set_test(&mut test13, 30 * k * k + 76 * k + 45, taille, p);
            set_test(&mut test11, 30 * k * k + 78 * k + 47, taille, p);
            set_test(&mut test7, 30 * k * k + 82 * k + 51, taille, p);
            set_test(&mut test1, 30 * k * k + 88 * k + 57, taille, p);
        }
        k += 1;
    }

    sortie(T::from(2).unwrap());
    sortie(T::from(3).unwrap());
    sortie(T::from(5).unwrap());
    for p in 0..taille_crible {
        if test1.contains(p) {
            sortie(T::from(30 * p + 1).unwrap());
        }
        if test7.contains(p) {
            sortie(T::from(30 * p + 7).unwrap());
        }
        if test11.contains(p) {
            sortie(T::from(30 * p + 11).unwrap());
        }
        if test13.contains(p) {
            sortie(T::from(30 * p + 13).unwrap());
        }
        if test17.contains(p) {
            sortie(T::from(30 * p + 17).unwrap());
        }
        if test19.contains(p) {
            sortie(T::from(30 * p + 19).unwrap());
        }
        if test23.contains(p) {
            sortie(T::from(30 * p + 23).unwrap());
        }
        if test29.contains(p) {
            sortie(T::from(30 * p + 29).unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crible2() {
        let expected = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];

        let mut result: Vec<u32> = Vec::new();
        crible2(100, |p| {
            result.push(p);
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_crible23_small() {
        let expected = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199,
        ];

        let mut result: Vec<u32> = Vec::new();
        crible23(200, |p| {
            result.push(p);
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_crible23_large() {
        let mut result: Vec<u32> = Vec::new();
        crible23(10000, |p| {
            result.push(p);
        });

        assert_eq!(result[1], 3);
        assert_eq!(result[2], 5);
        assert_eq!(result[100], 547);
        assert_eq!(result[1000], 7927);
    }

    #[test]
    fn test_crible235() {
        let expected = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
            487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599,
            601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701,
            709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823,
            827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941,
            947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019,
        ];

        let mut result: Vec<u32> = Vec::new();
        crible235(1000, |p| {
            result.push(p);
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_crible235_large() {
        let mut result: Vec<u32> = Vec::new();
        crible23(2000000, |p| {
            result.push(p);
        });

        assert_eq!(result[1], 3);
        assert_eq!(result[2], 5);
        assert_eq!(result[100], 547);
        assert_eq!(result[1000], 7927);
        assert_eq!(result[10000], 104743);
        assert_eq!(result[50000], 611957);
        assert_eq!(result[100000], 1299721);
    }
}
