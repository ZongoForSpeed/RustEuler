use crate::maths::arithmetic::Arithmetic;
use crate::register_problem;
use num_integer::Roots;
use std::collections::{HashMap, HashSet};

register_problem!(143, "Investigating the Torricelli point of a triangle", problem143);

pub fn problem143() -> String {
    // Let ABC be a triangle with all interior angles being less than 120 degrees.
    // Let X be any point inside the triangle and let XA = p, XB = q, and XC = r.
    //
    // Fermat challenged Torricelli to find the position of X such that p + q + r was minimised.
    //
    // Torricelli was able to prove that if equilateral triangles AOB, BNC and AMC are constructed on each side of
    // triangle ABC, the circumscribed circles of AOB, BNC, and AMC will intersect at a single point, T, inside the
    // triangle.
    // Moreover he proved that T, called the Torricelli/Fermat point, minimises p + q + r.
    // Even more remarkable, it can be shown that when the sum is minimised, AN = BM = CO = p + q + r and that AN, BM
    // and CO also intersect at T.
    //
    // If the sum is minimised and a, b, c, p, q and r are all positive integers we shall call triangle ABC a
    // Torricelli triangle.
    // For example, a = 399, b = 455, c = 511 is an example of a Torricelli triangle, with p + q + r = 784.
    //
    // Find the sum of all distinct values of p + q + r ≤ 120000 for Torricelli triangles.
    let limit = 120000;

    let mut dict:HashMap<u64, HashSet<u64>> = HashMap::new();
    
    for m in 1..limit.sqrt() {
        for n in 1..m {
            if (m - n) % 3 == 0 {
                continue;
            }
            if u64::gcd(m, n) != 1 {
                continue;
            }

            let a = 2 * n * m + n * n;
            let b = m * m - n * n;

            for k in (1..).take_while(|k| k * (a + b) < limit) {
                dict.entry(a * k).or_insert_with(HashSet::new).insert(b* k);
                dict.entry(b * k).or_insert_with(HashSet::new).insert(a* k);
            }
        }
    }
    
    let mut triplets = HashSet::new();

    for (a, bs) in &dict {
        for b in bs {
            if let Some(found_b) = dict.get(b) {
                for c in bs.intersection(found_b) {
                    let abc = a + b + c;
                    if abc < limit {
                        triplets.insert(abc);
                    }
                }                
            }
        }
    }
    
    triplets.into_iter().sum::<u64>().to_string()
}
