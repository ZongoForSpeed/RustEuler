use crate::register_problem;

register_problem!(260, "Stone Game", problem260);


/// A game is played with three piles of stones and two players.
/// At her turn, a player removes one or more stones from the piles. However, if she takes stones from more than one
/// pile, she must remove the same number of stones from each of the selected piles.
///
/// In other words, the player chooses some N>0 and removes:
///
///      N stones from any single pile; or
///      N stones from each of any two piles (2N total); or
///      N stones from each of the three piles (3N total).
///
/// The player taking the last stone(s) wins the game.
/// A winning configuration is one where the first player can force a win.
/// For example, (0,0,13), (0,11,11) and (5,5,5) are winning configurations because the first player can immediately
/// remove all stones.
///
/// A losing configuration is one where the second player can force a win, no matter what the first player does.
/// For example, (0,1,2) and (1,3,3) are losing configurations: any legal move leaves a winning configuration for the
/// second player.
///
/// Consider all losing configurations (xi,yi,zi) where xi ≤ yi ≤ zi ≤ 100.
/// We can verify that Σ(xi+yi+zi) = 173895 for these.
///
/// Find Σ(xi+yi+zi) where (xi,yi,zi) ranges over the losing configurations
/// with xi ≤ yi ≤ zi ≤ 1000.

#[inline(always)]
fn sort3(mut a: usize, mut b: usize, mut c: usize) -> (usize, usize, usize) {
    if a > b { std::mem::swap(&mut a, &mut b); }
    if b > c { std::mem::swap(&mut b, &mut c); }
    if a > b { std::mem::swap(&mut a, &mut b); }
    (a, b, c)
}

pub fn problem260() -> String {
    let limit = 1000;
    let mut result = 0;

    let size = limit + 1;

    let mut victories = vec![0u64; (size * size * size + 63) / 64];

    for x in 0..=limit {
        let x_offset = x * size * size;
        for y in x..=limit {
            let xy_offset = x_offset + y * size;
            for z in y..=limit {
                let idx = xy_offset + z;
                if (victories[idx / 64] >> (idx % 64)) & 1 != 0 {
                    continue;
                }

                result += x + y + z;

                // 1 pile
                for i in 1..=limit - x {
                    let (nx, ny, nz) = sort3(x + i, y, z);
                    let nidx = nx * size * size + ny * size + nz;
                    victories[nidx / 64] |= 1 << (nidx % 64);
                }
                for i in 1..=limit - y {
                    let (nx, ny, nz) = sort3(x, y + i, z);
                    let nidx = nx * size * size + ny * size + nz;
                    victories[nidx / 64] |= 1 << (nidx % 64);
                }
                for i in 1..=limit - z {
                    let nidx = xy_offset + (z + i);
                    victories[nidx / 64] |= 1 << (nidx % 64);
                }

                // 2 piles
                for i in 1..=limit - y {
                    let (nx, ny, nz) = sort3(x + i, y + i, z);
                    let nidx = nx * size * size + ny * size + nz;
                    victories[nidx / 64] |= 1 << (nidx % 64);
                }
                for i in 1..=limit - z {
                    let nidx = x * size * size + (y + i) * size + (z + i);
                    victories[nidx / 64] |= 1 << (nidx % 64);
                }
                for i in 1..=limit - z {
                    let (nx, ny, nz) = sort3(x + i, y, z + i);
                    let nidx = nx * size * size + ny * size + nz;
                    victories[nidx / 64] |= 1 << (nidx % 64);
                }

                // 3 piles
                for i in 1..=limit - z {
                    let nidx = (x + i) * size * size + (y + i) * size + (z + i);
                    victories[nidx / 64] |= 1 << (nidx % 64);
                }
            }
        }
    }
    result.to_string()
}


#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem260::problem260;

    #[test]
    fn test_problem260() {
        let result = problem260();
        assert_eq!(result, "167542057");
    }
}
