use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::AddAssign;
use crate::register_problem;

use num_traits::FloatConst;

register_problem!(199, "Iterative Circle Packing", problem199);

#[derive(Debug, Copy, Clone)]
struct Triplet(pub f64, pub f64, pub f64);

type Apollonios = BTreeMap<Triplet, u64>;

impl PartialEq for Triplet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Eq for Triplet {}

impl PartialOrd for Triplet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Triplet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .total_cmp(&other.0)
            .then_with(|| self.1.total_cmp(&other.1))
            .then_with(|| self.2.total_cmp(&other.2))
    }
}


fn area(k: f64) -> f64 {
    let r = f64::abs(1. / k);
    r * r * f64::PI()
}

fn iteration(apollonios: &Apollonios) -> (Apollonios, f64) {
    let mut a = 0.;
    let mut result = Apollonios::new();
    for (&Triplet(k1, k2, k3), second) in apollonios {
        let k4 = k1 + k2 + k3 + 2. * f64::sqrt(k1 * k2 + k2 * k3 + k1 * k3);
        a += (*second as f64) * area(k4);
        result.entry(Triplet(k4, k1, k2)).or_insert(0).add_assign(second);
        result.entry(Triplet(k4, k1, k3)).or_insert(0).add_assign(second);
        result.entry(Triplet(k4, k2, k3)).or_insert(0).add_assign(second);
    }

    (result, a)
}

pub fn problem199() -> String {
    // Three circles of equal radius are placed inside a larger circle such that each pair of circles is tangent to one
    // another and the inner circles do not overlap. There are four uncovered "gaps" which are to be filled iteratively
    // with more tangent circles.
    //
    // At each iteration, a maximally sized circle is placed in each gap, which creates more gaps for the next iteration.
    // After 3 iterations (pictured), there are 108 gaps and the fraction of the area which is not covered by circles is
    // 0.06790342, rounded to eight decimal places.
    //
    // What fraction of the area is not covered by circles after 10 iterations?
    // Give your answer rounded to eight decimal places using the format x.xxxxxxxx.
    let mut apollonios = Apollonios::new();

    // Théorème de Descartes : (k1 + k2 + k3 + k4)² = sqrt(k1² + k2² + k3² + k4²)
    // k4 = k1 + k2 + k3 +/- 2*sqrt(k1.k2 + k2.k3 + k3.k1)
    let k0 = -1.;
    let k1 = k0 / (3. - 2. * f64::sqrt(3.)); // k1 = k2 = k3

    apollonios.insert(Triplet(k1, k1, k1), 1);
    apollonios.insert(Triplet(k1, k1, k0), 3);

    let a0 = area(k0);
    let mut a = 3. * area(k1);

    println!("Iteration 0: {:.8}", 1. - a / a0);

    for n in 1..=10 {
        let (new_apollonios, new_a) = iteration(&apollonios);
        a += new_a;
        apollonios = new_apollonios;
        println!("Iteration {n}: {:.8}", 1. - a / a0);
    }

    let result = 1. - a / a0;
    format!("{:.8}", result)
}
