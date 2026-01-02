use crate::register_problem;
use std::collections::HashMap;

register_problem!(161, "Triominoes", problem161);

type Matrix = [[bool; 12]; 9];

fn free(m: &Matrix) -> Option<(usize, usize)> {
    for (i, line) in m.iter().enumerate() {
        for (j, value) in line.iter().enumerate() {
            if *value {
                return Some((i, j));
            }
        }
    }

    None
}

fn test_shape_1(m: &Matrix, i: usize, j: usize) -> bool {
    if m[i].len() < j + 3 {
        return false;
    }

    m[i][j] && m[i][j + 1] && m[i][j + 2]
}

fn test_shape_2(m: &Matrix, i: usize, j: usize) -> bool {
    if m.len() < i + 3 {
        return false;
    }

    m[i][j] && m[i + 1][j] && m[i + 2][j]
}

fn test_shape_3(m: &Matrix, i: usize, j: usize) -> bool {
    if m.len() < i + 2 || m[0].len() < j + 2 {
        return false;
    }

    m[i][j] && m[i + 1][j] && m[i][j + 1]
}

fn test_shape_4(m: &Matrix, i: usize, j: usize) -> bool {
    if m.len() < i + 2 || m[0].len() < j + 2 {
        return false;
    }

    m[i][j] && m[i + 1][j] && m[i + 1][j + 1]
}

fn test_shape_5(m: &Matrix, i: usize, j: usize) -> bool {
    if m.len() < i + 2 || m[0].len() < j + 2 {
        return false;
    }

    m[i][j] && m[i][j + 1] && m[i + 1][j + 1]
}

fn test_shape_6(m: &Matrix, i: usize, j: usize) -> bool {
    if m.len() < i + 2 || j == 0 {
        return false;
    }

    m[i][j] && m[i + 1][j] && m[i + 1][j - 1]
}

fn combinaison(cache: &mut HashMap<Matrix, u128>, m: Matrix) -> u128 {
    if let Some(v) = cache.get(&m) {
        return *v;
    }

    let mut result = 0;
    if let Some((i, j)) = free(&m) {
        if test_shape_1(&m, i, j) {
            let mut mm = m.clone();
            mm[i][j] = false;
            mm[i][j + 1] = false;
            mm[i][j + 2] = false;
            result += combinaison(cache, mm);
        }

        if test_shape_2(&m, i, j) {
            let mut mm = m.clone();
            mm[i][j] = false;
            mm[i + 1][j] = false;
            mm[i + 2][j] = false;
            result += combinaison(cache, mm);
        }

        if test_shape_3(&m, i, j) {
            let mut mm = m.clone();
            mm[i][j] = false;
            mm[i + 1][j] = false;
            mm[i][j + 1] = false;
            result += combinaison(cache, mm);
        }

        if test_shape_4(&m, i, j) {
            let mut mm = m.clone();
            mm[i][j] = false;
            mm[i + 1][j] = false;
            mm[i + 1][j + 1] = false;
            result += combinaison(cache, mm);
        }

        if test_shape_5(&m, i, j) {
            let mut mm = m.clone();
            mm[i][j] = false;
            mm[i][j + 1] = false;
            mm[i + 1][j + 1] = false;
            result += combinaison(cache, mm);
        }

        if test_shape_6(&m, i, j) {
            let mut mm = m.clone();
            mm[i][j] = false;
            mm[i + 1][j] = false;
            mm[i + 1][j - 1] = false;
            result += combinaison(cache, mm);
        }
    } else {
        result = 1;
    }
    cache.insert(m, result);
    result
}

pub fn problem161() -> String {
    // A triomino is a shape consisting of three squares joined via the edges. There are two basic forms:
    //
    //              XXX     XX
    //                      X
    //
    // If all possible orientations are taken into account there are six:
    //
    //           X  X        X
    //      XXX  X  XX  XX  XX  XX
    //           X      X        X
    //
    // Any n by m grid for which nxm is divisible by 3 can be tiled with triominoes.
    // If we consider tilings that can be obtained by reflection or rotation from another tiling as different there are
    // 41 ways a 2 by 9 grid can be tiled with triominoes:
    //
    // In how many ways can a 9 by 12 grid be tiled in this way by triominoes?
    let m: Matrix = [[true; 12]; 9];
    let mut cache: HashMap<Matrix, u128> = HashMap::new();
    let result = combinaison(&mut cache, m);
    result.to_string()
}
