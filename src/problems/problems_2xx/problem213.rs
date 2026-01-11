use crate::register_problem;
use ndarray::Array2;

register_problem!(213, "Flea Circus", problem213);

fn power_matrix(a: &Array2<f64>, mut exponent: usize) -> Array2<f64> {
    let mut base = a.clone();
    let size = a.shape()[0];
    let mut result = Array2::<f64>::eye(size);
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = result.dot(&base);
        }
        exponent /= 2;
        base = base.dot(&base);
    }
    result
}

pub fn problem213() -> String {
    // A 30×30 grid of squares contains 900 fleas, initially one flea per square.
    // When a bell is rung, each flea jumps to an adjacent square at random (usually 4 possibilities,
    // except for fleas on the edge of the grid or at the corners).
    //
    // What is the expected number of unoccupied squares after 50 rings of the bell? Give your answer
    // rounded to six decimal places.
    let taille = 30;

    let mut matrix_a = Array2::<f64>::zeros((taille * taille, taille * taille));
    for i in 0..taille {
        for j in 0..taille {
            let ij = i * taille + j;
            let mut adjacent = Vec::new();
            if i > 0 {
                adjacent.push((i - 1) * taille + j);
            }
            if j > 0 {
                adjacent.push(i * taille + j - 1);
            }
            if i < taille - 1 {
                adjacent.push((i + 1) * taille + j);
            }
            if j < taille - 1 {
                adjacent.push(i * taille + j + 1);
            }

            let value = 1. / adjacent.len() as f64;
            for a in adjacent {
                matrix_a[[ij, a]] = value;
            }
        }
    }

    let mut result = 0.;
    let an = power_matrix(&matrix_a, 50);
    for j in 0..taille * taille {
        let mut e = 1.;
        for i in 0..taille * taille {
            e *= 1. - an[[i, j]];
        }
        result += e;
    }

    format!("{:.6}", result)
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem213::problem213;

    #[test]
    fn test_problem213() {
        let result = problem213();
        assert_eq!(result, "330.721154");
    }
}
