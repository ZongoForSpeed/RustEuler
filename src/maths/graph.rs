use std::cmp::Reverse;
use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use num_traits::PrimInt;

pub fn a_star<T, V, G, D, H>(start: T, end: T, graph: G, distance: D, heuristic: H) -> V
where
    T: Hash + Eq + Clone,
    V: PrimInt + Hash + Eq,
    G: Fn(&T) -> Vec<T>,
    D: Fn(&T, &T) -> V,
    H: Fn(&T, &T) -> V,
{
    let mut closed_list: HashSet<T> = HashSet::new();
    let mut queue:PriorityQueue<(T, V), Reverse<V>> = PriorityQueue::new();

    queue.push((start, V::zero()), Reverse(V::zero()));

    while !queue.is_empty() {
        if let Some(((node, cost), _)) = queue.pop() {
            if node == end {
                return cost;
            }

            if closed_list.insert(node.clone()) {
                let moves = graph(&node);
                for mov in moves {
                    if !closed_list.contains(&mov) {
                        let d1 = distance(&node, &mov);
                        let d2 = heuristic(&mov, &end);

                        queue.push((mov, cost + d1), Reverse(cost + d1 + d2));
                    }
                }
            }
        }
    }

    V::zero()
}

pub struct Dijkstra {
    graph: HashMap<usize, Vec<(usize, u32)>>,
    start: usize,
    end: usize,
}

impl Dijkstra {
    pub fn new(graph: HashMap<usize, Vec<(usize, u32)>>, start: usize, end: usize) -> Dijkstra {
        Dijkstra { graph, start, end }
    }

    pub fn algorithm(self) -> u32 {
        let length = self.graph.len();

        let mut paths = vec![u32::MAX; length];
        // let mut previous = vec![self.start; length];
        paths[self.start] = 0;

        let mut nodes: HashSet<usize> = self.graph.keys().copied().collect();

        while !nodes.is_empty() {
            let mut suivant = 0;
            let mut minimum = u32::MAX;
            for n in &nodes {
                if paths[*n] < minimum {
                    suivant = *n;
                    minimum = paths[*n];
                }
            }

            nodes.remove(&suivant);
            if let Some(v) = self.graph.get(&suivant) {
                for (key, value) in v {
                    if paths[*key] > paths[suivant] + value {
                        paths[*key] = paths[suivant] + value;
                        // previous[*key] = suivant;
                    }
                }
            }
        }

        paths[self.end]
    }
}
