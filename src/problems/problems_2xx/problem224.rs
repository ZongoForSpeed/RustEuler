use crate::register_problem;
use std::collections::VecDeque;

register_problem!(224, "Almost right-angled triangles II", problem224);

type Triplet = (i64, i64, i64);

pub fn problem224() -> String {
    // Let us call an integer sided triangle with sides a ≤ b ≤ c barely obtuse if the sides satisfy
    // a² + b² = c² - 1.
    //
    // How many barely obtuse triangles are there with perimeter ≤ 75,000,000?
    let limit = 75000000;
    let mut result = 0;

    let mut queue: VecDeque<Triplet> = VecDeque::new();
    queue.push_back((2, 2, 3));
    result += 1;

    while let Some((a, b, c)) = queue.pop_front() {
        let (a2, b2, c2) = (a - 2 * b + 2 * c, 2 * a - b + 2 * c, 2 * a - 2 * b + 3 * c);
        if a2 + b2 + c2 <= limit {
            queue.push_back((a2, b2, c2));
            result += 1;
        }

        let (a2, b2, c2) = (a + 2 * b + 2 * c, 2 * a + b + 2 * c, 2 * a + 2 * b + 3 * c);
        if a2 + b2 + c2 <= limit {
            queue.push_back((a2, b2, c2));
            result += 1;
        }

        if a != b {
            let (a2, b2, c2) = (2 * b + 2 * c - a, b + 2 * c - 2 * a, 2 * b + 3 * c - 2 * a);
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
    use crate::problems::problems_2xx::problem224::problem224;

    #[test]
    fn test_problem224() {
        let result = problem224();
        assert_eq!(result, "4137330");
    }
}
