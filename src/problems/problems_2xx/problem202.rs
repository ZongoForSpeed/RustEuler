use crate::maths::arithmetic::Arithmetic;
use crate::register_problem;

register_problem!(202, "\"Laserbeam\"", problem202);

pub fn problem202() -> String {
    // Three mirrors are arranged in the shape of an equilateral triangle, with their reflective surfaces
    // pointing inwards. There is an infinitesimal gap at each vertex of the triangle through which
    // a laser beam may pass.
    //
    // Label the vertices A, B and C. There are 2 ways in which a laser beam may enter vertex C, bounce
    // off 11 surfaces, then exit through the same vertex: one way is shown below; the other is the
    // reverse of that.
    //
    // There are 80840 ways in which a laser beam may enter vertex C, bounce off 1000001 surfaces,
    // then exit through the same vertex.
    //
    // In how many ways can a laser beam enter at vertex C, bounce off 12017639147 surfaces, then exit
    // through the same vertex?
    let limit: u64 = 12017639147;

    let primes: Vec<u64> = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];

    let max = (limit + 3) / 2;
    let mut d = Vec::new();
    max.factorization(&primes, |p, _| d.push(p));

    println!("{:?}", d);

    let mut result = 0;
    for m in (2..max / 2 + 1).step_by(3) {
        // if d.iter().all(|&p| m%p!= 0) {
        if (m % 5 != 0)
            && (m % 11 != 0)
            && (m % 17 != 0)
            && (m % 23 != 0)
            && (m % 29 != 0)
            && (m % 41 != 0)
            && (m % 47 != 0)
        {
            result += 2;
        }
    }

    println!("{result}");
    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem202::problem202;

    #[test]
    fn test_problem202() {
        let result = problem202();
        assert_eq!(result, "1209002624");
    }
}
