use crate::register_problem;
use disjoint::DisjointSet;

register_problem!(186, "Connectedness of a network", problem186);

fn lagged_fibonacci_generator(cache: &mut Vec<usize>, n: usize) -> usize {
    if cache.is_empty() {
        cache.push(0);
        for k in 1..56 {
            cache.push((100003 + 300007 * k * k * k - 200003 * k) % 1000000);
        }
    }

    if cache.len() <= n {
        for k in cache.len()..n + 1000 {
            cache.push((cache[k - 24] + cache[k - 55]) % 1000000);
        }
    }

    cache[n]
}

fn caller(mut cache: &mut Vec<usize>, n: usize) -> usize {
    lagged_fibonacci_generator(&mut cache, 2 * n - 1)
}

fn called(mut cache: &mut Vec<usize>, n: usize) -> usize {
    lagged_fibonacci_generator(&mut cache, 2 * n)
}

pub fn problem186() -> String {
    // Here are the records from a busy telephone system with one million users:
    //
    //                  RecNr   Caller  Called
    //                      1	200007	100053
    //                      2	600183	500439
    //                      3	600863	701497
    //                    ...      ...     ...
    //
    // The telephone number of the caller and the called number in record n are Caller(n) = S2n-1
    // and Called(n) = S2n where S1,2,3,... come from the "Lagged Fibonacci Generator":
    //
    // For 1 ≤ k ≤ 55, Sk = [100003 - 200003k + 300007k3] (modulo 1000000)
    // For 56 ≤ k, Sk = [Sk-24 + Sk-55] (modulo 1000000)
    //
    // If Caller(n) = Called(n) then the user is assumed to have misdialled and the call fails;
    // otherwise the call is successful.
    //
    // From the start of the records, we say that any pair of users X and Y are friends if X calls Y
    // or vice-versa. Similarly, X is a friend of a friend of Z if X is a friend of Y and Y is a
    // friend of Z; and so on for longer chains.
    //
    // The Prime Minister's phone number is 524287. After how many successful calls, not counting
    // misdials, will 99% of the users (including the PM) be a friend, or a friend of a friend etc.,
    // of the Prime Minister?
    let prime_minister = 524287;

    let mut forest = DisjointSet::with_len(1000000);
    let mut group_size = vec![1; 1000000];

    for _ in 0..1000000 {
        forest.add_singleton();
    }

    let mut result = 0;
    let mut cache: Vec<usize> = Vec::new();
    let mut counter = 0;

    for n in 1.. {
        let i = called(&mut cache, n);
        let j = caller(&mut cache, n);
        if i == j {
            continue;
        }

        result += 1;
        if !forest.is_joined(i, j) {
            let size_i = group_size[forest.root_of(i)];
            let size_j = group_size[forest.root_of(j)];
            forest.join(i, j);
            group_size[forest.root_of(i)] = size_i + size_j;
            counter = group_size[forest.root_of(prime_minister)];
        }

        if counter >= 990000 {
            return result.to_string();
        }
    }

    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem186() {
        let result = problem186();
        assert_eq!(result, "2325629");
    }
}
