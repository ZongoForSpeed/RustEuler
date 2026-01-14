use crate::register_problem;
use itertools::Itertools;

use rand::seq::SliceRandom;

register_problem!(222, "Sphere Packing", problem222);

fn height_sphere(r1: f64, r2: f64) -> f64 {
    2. * (r1 + r2 - 1.).sqrt()
}

fn height_stack(stack: &Vec<f64>) -> f64 {
    let mut result = stack[0] + stack[stack.len() - 1];
    for (p, n) in stack.iter().tuple_windows() {
        result += height_sphere(*p, *n);
    }
    result
}

pub fn problem222() -> String {
    // What is the length of the shortest pipe, of internal radius 50mm, that can fully contain 21 balls of radii 30mm,
    // 31mm, ..., 50mm?
    //
    // Give your answer in micrometres (10^-6 m) rounded to the nearest integer.
    let mut generator = rand::rng();
    const R: f64 = 50.;

    let size = 21;

    let mut radius = Vec::new();
    for n in 0..size {
        radius.push((30. + n as f64) / R);
    }

    radius.shuffle(&mut generator);

    let mut optimum = height_stack(&radius);
    let mut optimum_local = optimum;

    for _ in 0..100000 {
        let mut minimum = optimum_local;
        let mut m_i = size;
        let mut m_j = size;
        for i in 0..size - 1 {
            for j in i + 1..size {
                radius.swap(i, j);
                let h = height_stack(&radius);
                if h < minimum {
                    minimum = h;
                    m_i = i;
                    m_j = j;
                }
                radius.swap(i, j);
            }
        }

        if minimum < optimum_local {
            radius.swap(m_i, m_j);
            optimum_local = minimum;
        } else {
            optimum = if optimum_local < optimum {
                optimum_local
            } else {
                optimum
            };
            radius.shuffle(&mut generator);
            optimum_local = height_stack(&radius);
        }
    }

    let result = (optimum * R * 1000.).round() as i32;
    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem222::problem222;

    #[test]
    fn test_problem222() {
        let result = problem222();
        assert_eq!(result, "1590933");
    }
}
