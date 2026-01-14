use crate::register_problem;
use std::collections::VecDeque;

register_problem!(223, "Almost right-angled triangles I", problem223);

type Triplet = (i64, i64, i64);

pub fn problem223() -> String {
    // Let us call an integer sided triangle with sides a ≤ b ≤ c barely acute if the sides satisfy a² + b² = c² + 1.
    //
    // How many barely acute triangles are there with perimeter ≤ 25,000,000?
    let limit = 25000000;
    let mut result = 0;

    let mut queue: VecDeque<Triplet> = VecDeque::new();
    queue.push_back((1, 1, 1));
    queue.push_back((1, 2, 2));
    result += 2;

    while let Some((a, b, c)) = queue.pop_front() {
        let (a2, b2, c2) = (2 * c + b - 2 * a, 2 * c + 2 * b - a, 3 * c + 2 * b - 2 * a);
        if a2 + b2 + c2 <= limit {
            queue.push_back((a2, b2, c2));
            result += 1;
        }

        let (a2, b2, c2) = (2 * c + b + 2 * a, 2 * c + 2 * b + a, 3 * c + 2 * b + 2 * a);
        if a2 + b2 + c2 <= limit {
            queue.push_back((a2, b2, c2));
            result += 1;
        }

        if a != b {
            let (a2, b2, c2) = (2 * c - 2 * b + a, 2 * c - b + 2 * a, 3 * c - 2 * b + 2 * a);
            if a2 + b2 + c2 <= limit {
                queue.push_back((a2, b2, c2));
                result += 1;
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem223::problem223;

    #[test]
    fn test_problem223() {
        let result = problem223();
        assert_eq!(result, "61614848");
    }
}
