use crate::maths::timer::ScopeTimer;

pub fn problem031() -> u64 {
    let _timer = ScopeTimer::new("Problem 31 Coin sums", false);
    // In England the currency is made up of pound, £, and pence, p, and there are eight coins in general circulation:
    //
    // 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
    // It is possible to make £2 in the following way:
    //
    // 1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
    // How many different ways can £2 be made using any number of coins?
    let objectif = 200;
    let pieces = vec![1, 2, 5, 10, 20, 50, 100, 200];
    let mut dp: Vec<u64> = vec![0; objectif + 1];
    dp[0] = 1;
    for piece in pieces {
        for j in piece..=objectif {
            dp[j] += dp[j - piece];
        }
    }

    dp[objectif]
}
