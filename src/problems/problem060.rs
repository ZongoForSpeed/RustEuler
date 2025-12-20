use crate::maths::digits::concat_numbers;
use crate::maths::primes::crible235;
use crate::register_problem;
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet};

fn v_intersection(a: &Vec<u64>, b: &Vec<u64>) -> Vec<u64> {
    let set_a = HashSet::<u64>::from_iter(a.iter().cloned());
    let set_b = HashSet::<u64>::from_iter(b.iter().cloned());

    set_a.intersection(&set_b).cloned().sorted().collect()
}

fn next_group(g: &HashMap<Vec<u64>, Vec<u64>>, groupe: &HashMap<Vec<u64>, Vec<u64>>) -> HashMap<Vec<u64>, Vec<u64>> {
    let mut suivant = HashMap::new();
    for (p, arete) in g {
        for q in arete {
            let mut v = vec![q.clone()];
            if let Some(arete_q) = groupe.get(&v) {
                let intersection = v_intersection(arete, arete_q);
                if !intersection.is_empty() {
                    v.extend(p);
                    v.sort();
                    suivant.insert(v, intersection);
                }
            }
        }
    }
    suivant
}

register_problem!(60, "Prime pair sets", problem060);

pub fn problem060() -> String {
    // The primes 3, 7, 109, and 673, are quite remarkable. By taking any two primes and concatenating them in
    // any order the result will always be prime. For example, taking 7 and 109, both 7109 and 1097 are prime.
    // The sum of these four primes, 792, represents the lowest sum for a set of four primes with this property.
    //
    // Find the lowest sum for a set of five primes for which any two primes concatenate to produce another prime.
    let limit: u64 = 10000;
    let mut primes: BTreeSet<u64> = BTreeSet::new();
    crible235((limit as usize) * (limit as usize), |p| {
        primes.insert(p);
    });

    let mut graph: HashMap<Vec<u64>, Vec<u64>> = HashMap::new();

    for p in &primes {
        if *p > limit {
            break;
        }
        for q in &primes {
            if q >= p {
                break;
            }

            let pq = concat_numbers(*p, *q, 10);
            let qp = concat_numbers(*q, *p, 10);

            if primes.contains(&pq) && primes.contains(&qp) {
                let v = vec![q.clone()];

                graph.entry(v).or_default().push(*p);
            }
        }
    }
    println!("Step 1 : {:?}", graph.len());
    let next = next_group(&graph, &graph);
    println!("Step 2 : {:?}", next.len());
    let next = next_group(&next, &graph);
    println!("Step 3 : {:?}", next.len());
    let next = next_group(&next, &graph);
    println!("Step 4 : {:?}", next.len());

    let mut solution: Vec<u64> = Vec::new();
    if !next.is_empty() {
        // println!("{:?}", suivant);
        let entry = next.iter().next().unwrap();
        solution.extend(entry.0);
        solution.extend(entry.1);
        solution.sort();
    }


    let result = solution.iter().sum::<u64>();
    result.to_string()
}
