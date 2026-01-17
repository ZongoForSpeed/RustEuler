use crate::register_problem;
use std::collections::HashMap;

register_problem!(194, "Coloured Configurations", problem194);

type Key = (u128, u128, u128);
type Cache = HashMap<Key, u128>;

fn combination_b(color: u128) -> u128 {
    let c = color - 2;
    let mut result = 0;

    result += c * c * c + c;
    result += c * (c * c * c + c * c + c - 1);
    result += (c - 1) * c * (c * c * c + c - 2);
    result += 2 * c * (c * c * c + c - 1);
    result %= 100000000;

    result
}

fn combination_a(color: u128) -> u128 {
    let c = color - 2;
    let mut result = 0;
    result += 1 * (c * c * c + c);
    result += (c - 1) * c * (c * c * c + c - 2);
    result += (2 * c) * (c * c * c + c - 1);
    result %= 100000000;
    result
}

fn n(mut cache: &mut Cache, a: u128, b: u128, c: u128) -> u128 {
    if a == 0 && b == 0 {
        return c * (c - 1) % 100000000;
    }

    let key = (a, b, c);
    if let Some(&value) = cache.get(&key) {
        return value;
    }

    let mut result = 0;
    if b > 0 {
        result += combination_b(c) * n(&mut cache, a, b - 1, c);
    }

    if a > 0 {
        result += combination_a(c) * n(&mut cache, a - 1, b, c);
    }

    result %= 100000000;

    cache.insert(key, result);
    result
}

pub fn problem194() -> String {
    // Consider graphs built with the units A:  and B: , where the units are glued along the
    // vertical edges as in the graph .
    //
    // A configuration of type (a,b,c) is a graph thus built of a units A and b units B, where the
    // graph's vertices are coloured using up to c colours, so that no two adjacent vertices have
    // the same colour.
    // The compound graph above is an example of a configuration of type (2,2,6), in fact of type
    // (2,2,c) for all c ≥ 4.
    //
    // Let N(a,b,c) be the number of configurations of type (a,b,c).
    // For example, N(1,0,3) = 24, N(0,2,4) = 92928 and N(2,2,3) = 20736.
    //
    // Find the last 8 digits of N(25,75,1984).
    let mut cache = Cache::new();

    println!("N(1,0,3) = {}", n(&mut cache, 1, 0, 3));
    println!("N(0,2,4) = {}", n(&mut cache, 0, 2, 4));
    println!("N(2,2,3) = {}", n(&mut cache, 2, 2, 3));

    let result = n(&mut cache, 25, 75, 1984);
    println!("N(2,2,3) = {}", result);

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem194() {
        let result = problem194();
        assert_eq!(result, "61190912");
    }
}
