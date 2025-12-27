use crate::maths::pythagorean::pythagorean_limit;
use crate::register_problem;

register_problem!(75, "Singular integer right triangles", problem075);

pub fn problem075() -> String {
    // It turns out that 12 cm is the smallest length of wire that can be bent to form an integer
    // sided right angletriangle in exactly one way, but there are many more examples.
    //
    //      12 cm: (3,4,5)
    //      24 cm: (6,8,10)
    //      30 cm: (5,12,13)
    //      36 cm: (9,12,15)
    //      40 cm: (8,15,17)
    //      48 cm: (12,16,20)
    //
    // In contrast, some lengths of wire, like 20 cm, cannot be bent to form an integer sided right
    // angle triangle, and other lengths allow more than one solution to be found; for example,
    // using 120 cm it is possible to form exactly three different integer sided right angle
    // triangles.
    //
    //      120 cm: (30,40,50), (20,48,52), (24,45,51)
    //
    // Given that L is the length of the wire, for how many values of L ≤ 1,500,000 can exactly one integer sided right
    // angle triangle be formed?

    let limit = 1500000;

    let mut solutions: Vec<u32> = vec![0; limit + 1];

    for (a, b, c) in pythagorean_limit(limit / 2) {
        let l = a + b + c;
        for k in (l..=limit).step_by(l) {
            solutions[k] += 1;
        }
    }

    solutions.into_iter().filter(|s| *s == 1).count().to_string()
}
