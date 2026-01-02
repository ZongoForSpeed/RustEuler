use crate::register_problem;

register_problem!(126, "Cuboid layers", problem126);

pub fn problem126() -> String {
    // The minimum number of cubes to cover every visible face on a cuboid measuring 3 x 2 x 1 is twenty-two.
    //
    // If we then add a second layer to this solid it would require forty-six cubes to cover every visible face, the
    // third layer would require seventy-eight cubes, and the fourth layer would require one-hundred and eighteen cubes
    // to cover every visible face.
    //
    // However, the first layer on a cuboid measuring 5 x 1 x 1 also requires twenty-two cubes; similarly the first
    // layer on cuboids measuring 5 x 3 x 1, 7 x 2 x 1, and 11 x 1 x 1 all contain forty-six cubes.
    //
    // We shall define C(n) to represent the number of cuboids that contain n cubes in one of its layers. So C(22) = 2,
    // C(46) = 4, C(78) = 5, and C(118) = 8.
    //
    // It turns out that 154 is the least value of n for which C(n) = 10.
    //
    // Find the least value of n for which C(n) = 1000.
    let limit = 100000;
    let mut couche = vec![0; limit];

    for a in 1..limit / 4 {
        let limit_b = std::cmp::min(limit / (4 * a), a+1);
        for b in 1..limit_b {
            for c in 1..=b {
                let mut n = 1;
                let mut layer = 2 * (a * b + b * c + a * c);
                while layer < limit {
                    couche[layer] += 1;
                    n += 1;
                    layer += 4 * (a + b + c) + 8 * (n - 2);
                }
            }
        }
    }

    couche.iter().position(|&r| r == 1000).unwrap().to_string()
}
