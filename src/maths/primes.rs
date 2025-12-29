use crate::maths::power::Power;
use crate::utils::timer::ScopeTimer;
use bit_set::BitSet;
use num_traits::PrimInt;
use rayon::join;

#[inline]
fn split_mix_64(x: &mut u64) -> u64 {
    *x = x.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *x;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

pub trait Primes {
    fn miller_rabin(self, reps: u16) -> bool;

    fn next_prime(self, reps: u16) -> Self;
}

trait InternalPrimes: Power {
    fn internal_miller_rabin(self, a: Self, q: Self, k: usize) -> bool;
}

macro_rules! impl_primes {
    ($($t:ty),*) => {
        $(
            impl InternalPrimes for $t {
                fn internal_miller_rabin(self, a: Self, q: Self, mut k: usize) -> bool {
                    let mut x = Self::power_mod(a, q, self);
                    if x == 1 || x + 1 == self {
                        return false;
                    }

                    while k > 1 {
                        x = Self::power_mod(x, 2, self);
                        if x + 1 == self {
                            return false;
                        }
                        k -= 1;
                    }

                    true
                }
            }

            impl Primes for $t {

                fn miller_rabin(self, reps: u16) -> bool {
                    if self == 2 {
                        return true;
                    }

                    if self % 2 == 0 {
                        return false;
                    }

                    if self < 100 {
                        return matches!(self,
                        2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 | 23 | 29 |
                        31 | 37 | 41 | 43 | 47 | 53 | 59 | 61 |
                        67 | 71 | 73 | 79 | 83 | 89 | 97);
                    }

                    let nm1 = self - 1;

                    // Test de primalité de Fermet avec 210
                    // https://fr.wikipedia.org/wiki/Test_de_primalit%C3%A9_de_Fermat
                    let y = Self::power_mod(210, nm1, self);
                    if y != 1 {
                        return false;
                    }

                    let mut q = nm1;
                    let mut k = 0;
                    while q % 2 == 0 {
                        k += 1;
                        q /= 2;
                    }

                    let mut seed = self as u64 ^ 0xDEADBEEFCAFEBABE;
                    let n = self as u64;

                    for _ in 0..reps {
                        let r = split_mix_64(&mut seed);
                        let x = (r % (n - 3) + 2) as Self;
                        // let x = rng.random_range(2..self - 1);
                        if self.internal_miller_rabin(x, q, k) {
                            return false;
                        }
                    }

                    return true;
                }

                fn next_prime(self, reps: u16) -> Self {
                    const NEXTS:[$t; 210] = [
                        2, 2, 3, 5, 5, 7, 7, 11, 11, 11, 11, 13, 13, 17, 17, 17, 17, 19, 19, 23, 23,
                        23, 23, 29, 29, 29, 29, 29, 29, 31, 31, 37, 37, 37, 37, 37, 37, 41, 41, 41,
                        41, 43, 43, 47, 47, 47, 47, 53, 53, 53, 53, 53, 53, 59, 59, 59, 59, 59, 59,
                        61, 61, 67, 67, 67, 67, 67, 67, 71, 71, 71, 71, 73, 73, 79, 79, 79, 79, 79,
                        79, 83, 83, 83, 83, 89, 89, 89, 89, 89, 89, 97, 97, 97, 97, 97, 97, 97, 97,
                        101, 101, 101, 101, 103, 103, 107, 107, 107, 107, 109, 109, 113, 113, 113,
                        113, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127,
                        131, 131, 131, 131, 137, 137, 137, 137, 137, 137, 139, 139, 149, 149, 149,
                        149, 149, 149, 149, 149, 149, 149, 151, 151, 157, 157, 157, 157, 157, 157,
                        163, 163, 163, 163, 163, 163, 167, 167, 167, 167, 173, 173, 173, 173, 173,
                        173, 179, 179, 179, 179, 179, 179, 181, 181, 191, 191, 191, 191, 191, 191,
                        191, 191, 191, 191, 193, 193, 197, 197, 197, 197, 199, 199, 211, 211, 211,
                        211, 211, 211, 211, 211, 211, 211, 211];

                    // pre-calced sieve of eratosthenes for n = 2, 3, 5, 7
                    const INDICES: [$t; 48] = [
                        1, 11, 13, 17, 19, 23, 29, 31, 37, 41,
                        43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
                        89, 97, 101, 103, 107, 109, 113, 121, 127, 131,
                        137, 139, 143, 149, 151, 157, 163, 167, 169, 173,
                        179, 181, 187, 191, 193, 197, 199, 209
                    ];

                    // distances between sieve values
                    const OFFSETS: [$t; 48] = [
                        10, 2, 4, 2, 4, 6, 2, 6, 4, 2, 4, 6,
                        6, 2, 6, 4, 2, 6, 4, 6, 8, 4, 2, 4,
                        2, 4, 8, 6, 4, 6, 2, 4, 6, 2, 6, 6,
                        4, 2, 4, 6, 2, 6, 4, 2, 4, 2, 10, 2
                    ];

                    if self < 2 {
                        return 2;
                    }

                    if self < 210 {
                        return NEXTS[self as usize];
                    }

                    let nn = (self + 1) | 1;

                    let x: Self = nn % 210;
                    let mut s: usize = 0;
                    let mut e: usize = 47;
                    let mut m: usize = 24;

                    while m != e {
                        if INDICES[m] < x {
                            s = m;
                            m = (s + e + 1) >> 1;
                        } else {
                            e = m;
                            m = (s + e) >> 1;
                        }
                    }

                    let mut i = nn + INDICES[m] - x;

                    for mm in m.. {
                        if i.miller_rabin(reps) {
                            return i;
                        }

                        i += OFFSETS[mm % OFFSETS.len()];
                    }

                    panic!()
                }
            }
        )*
    }
}

impl_primes!(u16, u32, u64, u128, usize);

pub(crate) fn crible2<T, F>(taille: usize, mut sortie: F)
where
    T: PrimInt + std::ops::DivAssign,
    F: FnMut(T),
{
    let label = format!("Crible 2 with taille = {}", taille);
    let _timer = ScopeTimer::new(label.as_str(), false);

    let taille_crible = taille / 2;
    let mut test: BitSet<u32> = BitSet::from_iter(1..taille_crible);
    let mut p = 1;
    while p * p < taille_crible / 2 {
        if test.contains(p) {
            ((2 * (p * p + p))..taille_crible)
                .step_by(2 * p + 1)
                .for_each(|n| {
                    test.remove(n);
                });
        }

        p += 1;
    }

    sortie(T::from(2).unwrap());
    for p in test.iter() {
        sortie(T::from(2 * p + 1).unwrap());
    }
}

pub(crate) fn crible23<T, F>(taille: usize, mut sortie: F)
where
    T: PrimInt + std::ops::DivAssign,
    F: FnMut(T),
{
    let label = format!("Crible 2-3 with taille = {}", taille);
    let _timer = ScopeTimer::new(label.as_str(), false);

    let taille_crible = taille / 6 + 1;
    let mut test1: BitSet<u32> = BitSet::from_iter(1..taille_crible);
    let mut test5: BitSet<u32> = BitSet::from_iter(0..taille_crible);

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();
    pool.install(|| internal_crible23(taille_crible, &mut test1, &mut test5));

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

fn internal_crible23(taille_crible: usize, test1: &mut BitSet<u32>, test5: &mut BitSet<u32>) {
    let mut k = 0;
    while 6 * k * k < taille_crible {
        if test1.contains(k) {
            let p = 6 * k + 1;
            join(
                || {
                    (6 * k * k + 2 * k..taille_crible).step_by(p).for_each(|n| {
                        test1.remove(n);
                    })
                },
                || {
                    (6 * k * k + 6 * k..taille_crible).step_by(p).for_each(|n| {
                        test5.remove(n);
                    })
                },
            );
        }
        if test5.contains(k) {
            let p = 6 * k + 5;
            join(
                || {
                    (6 * k * k + 10 * k + 4..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test1.remove(n);
                        })
                },
                || {
                    (6 * k * k + 12 * k + 5..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test5.remove(n);
                        })
                },
            );
        }
        k += 1;
    }
}

pub(crate) fn crible235<T, F>(taille: usize, mut sortie: F)
where
    T: PrimInt + std::ops::DivAssign,
    F: FnMut(T),
{
    let label = format!("Crible 2-3-5 with taille = {}", taille);
    let _timer = ScopeTimer::new(label.as_str(), false);

    let taille_crible = taille / 30 + 1;
    let mut test1: BitSet<u32> = BitSet::from_iter(1..taille_crible);
    let mut test7: BitSet<u32> = BitSet::from_iter(0..taille_crible);
    let mut test11: BitSet<u32> = BitSet::from_iter(0..taille_crible);
    let mut test13: BitSet<u32> = BitSet::from_iter(0..taille_crible);
    let mut test17: BitSet<u32> = BitSet::from_iter(0..taille_crible);
    let mut test19: BitSet<u32> = BitSet::from_iter(0..taille_crible);
    let mut test23: BitSet<u32> = BitSet::from_iter(0..taille_crible);
    let mut test29: BitSet<u32> = BitSet::from_iter(0..taille_crible);

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();
    pool.install(|| {
        internal_crible235(
            taille_crible,
            &mut test1,
            &mut test7,
            &mut test11,
            &mut test13,
            &mut test17,
            &mut test19,
            &mut test23,
            &mut test29,
        )
    });

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

fn internal_crible235(
    taille_crible: usize,
    test1: &mut BitSet<u32>,
    test7: &mut BitSet<u32>,
    test11: &mut BitSet<u32>,
    test13: &mut BitSet<u32>,
    test17: &mut BitSet<u32>,
    test19: &mut BitSet<u32>,
    test23: &mut BitSet<u32>,
    test29: &mut BitSet<u32>,
) {
    let mut k = 0;
    while 30 * k * k < taille_crible {
        if test1.contains(k) {
            let p = 30 * k + 1;
            rayon::scope(|s| {
                s.spawn(|_| {
                    (30 * k * k + 2 * k + 0..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test1.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 8 * k + 0..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test7.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 12 * k + 0..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test11.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 14 * k + 0..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test13.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 18 * k + 0..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test17.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 20 * k + 0..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test19.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 24 * k + 0..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test23.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 30 * k + 0..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test29.remove(n);
                        })
                });
            });
        }
        if test7.contains(k) {
            let p = 30 * k + 7;
            rayon::scope(|s| {
                s.spawn(|_| {
                    (30 * k * k + 14 * k + 1..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test19.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 18 * k + 2..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test17.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 20 * k + 3..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test1.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 24 * k + 3..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test29.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 26 * k + 4..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test13.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 30 * k + 5..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test11.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 36 * k + 6..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test23.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 38 * k + 7..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test7.remove(n);
                        })
                });
            });
        }
        if test11.contains(k) {
            let p = 30 * k + 11;
            rayon::scope(|s| {
                s.spawn(|_| {
                    (30 * k * k + 22 * k + 4..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test1.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 24 * k + 4..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test23.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 28 * k + 6..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test7.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 30 * k + 6..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test29.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 34 * k + 8..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test13.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 40 * k + 10..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test19.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 42 * k + 11..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test11.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 48 * k + 13..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test17.remove(n);
                        })
                });
            });
        }
        if test13.contains(k) {
            let p = 30 * k + 13;
            rayon::scope(|s| {
                s.spawn(|_| {
                    (30 * k * k + 26 * k + 5..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test19.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 30 * k + 7..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test11.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 32 * k + 8..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test7.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 36 * k + 9..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test29.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 42 * k + 12..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test17.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 44 * k + 13..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test13.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 50 * k + 16..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test1.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 54 * k + 17..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test23.remove(n);
                        })
                });
            });
        }
        if test17.contains(k) {
            let p = 30 * k + 17;
            rayon::scope(|s| {
                s.spawn(|_| {
                    (30 * k * k + 34 * k + 9..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test19.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 36 * k + 10..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test23.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 40 * k + 13..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test1.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 46 * k + 16..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test13.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 48 * k + 17..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test17.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 54 * k + 20..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test29.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 58 * k + 23..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test7.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 60 * k + 24..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test11.remove(n);
                        })
                });
            });
        }
        if test19.contains(k) {
            let p = 30 * k + 19;
            rayon::scope(|s| {
                s.spawn(|_| {
                    (30 * k * k + 38 * k + 12..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test1.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 42 * k + 14..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test17.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 48 * k + 18..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test11.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 50 * k + 19..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test19.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 56 * k + 23..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test13.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 60 * k + 25..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test29.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 62 * k + 27..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test7.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 66 * k + 29..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test23.remove(n);
                        })
                });
            });
        }
        if test23.contains(k) {
            let p = 30 * k + 23;
            rayon::scope(|s| {
                s.spawn(|_| {
                    (30 * k * k + 46 * k + 17..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test19.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 52 * k + 22..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test7.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 54 * k + 23..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test23.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 60 * k + 28..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test11.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 64 * k + 31..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test13.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 66 * k + 32..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test29.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 70 * k + 36..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test1.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 72 * k + 37..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test17.remove(n);
                        })
                });
            });
        }
        if test29.contains(k) {
            let p = 30 * k + 29;
            rayon::scope(|s| {
                s.spawn(|_| {
                    (30 * k * k + 58 * k + 28..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test1.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 60 * k + 29..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test29.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 66 * k + 35..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test23.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 70 * k + 39..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test19.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 72 * k + 41..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test17.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 76 * k + 45..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test13.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 78 * k + 47..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test11.remove(n);
                        })
                });
                s.spawn(|_| {
                    (30 * k * k + 82 * k + 51..taille_crible)
                        .step_by(p)
                        .for_each(|n| {
                            test7.remove(n);
                        })
                });
            });
            (30 * k * k + 88 * k + 57..taille_crible)
                .step_by(p)
                .for_each(|n| {
                    test1.remove(n);
                });
        }
        k += 1;
    }
}

#[cfg(test)]
macro_rules! prime_tests_for_type {
    ($t:ident) => {
        mod $t {
            use super::Primes;

            #[test]
            fn small_primes() {
                let primes: &[$t] = &[
                    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
                ];
                for &p in primes {
                    assert!(p.miller_rabin(10), "{} should be prime", p);
                }
            }

            #[test]
            fn small_composites() {
                let composites: &[$t] = &[
                    0, 1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25, 26,
                ];
                for &n in composites {
                    assert!(!n.miller_rabin(10), "{} should be composite", n);
                }
            }

            #[test]
            fn even_numbers_gt_2() {
                for n in (4..200).step_by(2) {
                    let n = n as $t;
                    assert!(!n.miller_rabin(5), "{} should be composite", n);
                }
            }

            #[test]
            fn squares_are_composite() {
                for n in 2..50 {
                    let x = (n * n) as $t;
                    assert!(!x.miller_rabin(5), "{} = {}^2 should be composite", x, n);
                }
            }

            #[test]
            fn deterministic_reproducibility() {
                // même entrée → même résultat
                for n in 2..200 {
                    let n = n as $t;
                    let a = n.miller_rabin(7);
                    let b = n.miller_rabin(7);
                    assert_eq!(a, b, "non-deterministic result for {}", n);
                }
            }
        }
    };
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
        crible235(2000000, |p| {
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

    #[test]
    fn test_miller_rabin() {
        let n: u128 = 32416189721;
        assert_eq!(n.miller_rabin(25), true);
        assert_eq!((n + 44).miller_rabin(25), false);

        let m: u128 = 2305843009213693951;
        assert_eq!(m.miller_rabin(25), true);
        assert_eq!((m + 44).miller_rabin(25), false);

        assert_eq!((n * m).miller_rabin(25), false);
        assert_eq!((n * m + 44).miller_rabin(25), false);

        assert_eq!(9_074_200_321u128.miller_rabin(25), true);
        assert_eq!(7usize.miller_rabin(25), true);
    }

    #[test]
    fn test_next_prime() {
        let n = 32416189721u64;
        assert_eq!(n.next_prime(25), 32416189733);

        let m = 32416189877u64;
        assert_eq!(m.next_prime(25), 32416189909);

        let p = 2305843009213693951u64;
        assert_eq!(p.next_prime(25), 2305843009213693967);

        for n in p+1..2305843009213693967 {
            assert_eq!(n.miller_rabin(25), false);
        }

        let mut nexts: Vec<u32> = vec![
            4, 8, 15, 16, 23, 42, 69, 666, 8283, 98084, 730210, 2691418, 80314325, 620283078,
        ];
        for x in nexts.iter_mut() {
            *x = x.next_prime(25);
        }

        let expected: Vec<u32> = vec![
            5, 11, 17, 17, 29, 43, 71, 673, 8287, 98101, 730217, 2691421, 80314327, 620283089,
        ];
        assert_eq!(nexts, expected);

        let np = (n as u128) * (p as u128);
        let next_prime = np.next_prime(25);
        assert_eq!(next_prime, 74746644453512654146846077709);
    }

    //     BOOST_AUTO_TEST_CASE(suivant) {
    //
    //         unsigned long long p = 2305843009213693951ull;
    //         BOOST_CHECK_EQUAL(2305843009213693967, premiers::suivant(p));
    //         for (unsigned long long i = p + 2; i < 2305843009213693967; i += 2) {
    //             BOOST_CHECK_EQUAL(false, premiers::miller_rabin(i, 25));
    //         }
    //
    //         std::vector<size_t> suivants{4, 8, 15, 16, 23, 42, 69, 666, 8283, 98084, 730210, 2691418, 80314325, 620283078};
    //         for (auto &s: suivants) {
    //             s = premiers::suivant(s);
    //         }
    //
    //         std::vector<size_t> resultats{
    //             5, 11, 17, 17, 29, 43, 71, 673, 8287, 98101, 730217, 2691421, 80314327,
    //             620283089
    //         };
    //         BOOST_CHECK_EQUAL(resultats, suivants);
    //     }

    //prime_tests_for_type!(u8);
    prime_tests_for_type!(u16);
    prime_tests_for_type!(u32);
    prime_tests_for_type!(u64);
    prime_tests_for_type!(u128);
}
