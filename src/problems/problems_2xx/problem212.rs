use crate::register_problem;

register_problem!(212, "Combined Volume of Cuboids", problem212);

type Segment = (u64, u64);
type Cuboid = (Segment, Segment, Segment);

fn intersection_s(&(xa, dxa): &Segment, &(xb, dxb): &Segment) -> Option<Segment> {
    let x = std::cmp::max(xa, xb);
    let y = std::cmp::min(xa + dxa, xb + dxb);

    if x < y { Some((x, y - x)) } else { None }
}

fn intersection_c(&(xa, ya, za): &Cuboid, &(xb, yb, zb): &Cuboid) -> Option<Cuboid> {
    if let Some(x) = intersection_s(&xa, &xb)
        && let Some(y) = intersection_s(&ya, &yb)
        && let Some(z) = intersection_s(&za, &zb)
    {
        Some((x, y, z))
    } else {
        None
    }
}

fn volume(&(x, y, z): &Cuboid) -> u64 {
    let (_, dx) = x;
    let (_, dy) = y;
    let (_, dz) = z;
    dx * dy * dz
}

pub fn problem212() -> String {
    // An axis-aligned cuboid, specified by parameters { (x0,y0,z0), (dx,dy,dz) }, consists of all
    // points (X,Y,Z) such that x0 ≤ X ≤ x0+dx, y0 ≤ Y ≤ y0+dy and z0 ≤ Z ≤ z0+dz. The volume of the
    // cuboid is the product, dx × dy × dz.
    //
    // The combined volume of a collection of cuboids is the volume of their union and will be less
    // than the sum of the individual volumes if any cuboids overlap.
    //
    // Let C1,...,C50000 be a collection of 50000 axis-aligned cuboids such that Cn has parameters
    //
    // x0 = S6n-5 modulo 10000
    // y0 = S6n-4 modulo 10000
    // z0 = S6n-3 modulo 10000
    // dx = 1 + (S6n-2 modulo 399)
    // dy = 1 + (S6n-1 modulo 399)
    // dz = 1 + (S6n modulo 399)
    //
    // where S1,...,S300000 come from the "Lagged Fibonacci Generator":
    //
    // For 1 ≤ k ≤ 55, Sk = [100003 - 200003k + 300007k3]   (modulo 1000000)
    // For 56 ≤ k, Sk = [Sk-24 + Sk-55]   (modulo 1000000)
    //
    // Thus, C1 has parameters {(7,53,183),(94,369,56)}, C2 has parameters {(2383,3563,5079),(42,212,344)},
    // and so on.
    //
    // The combined volume of the first 100 cuboids, C1,...,C100, is 723581599.
    //
    // What is the combined volume of all 50000 cuboids, C1,...,C50000 ?
    let limite = 50000;
    let mut s: Vec<u64> = vec![0; 6 * limite + 1];

    for k in 1..56 {
        s[k] = ((300007 * k * k * k - 200003 * k + 100003) % 1000000) as u64;
    }

    for k in 56..6 * limite + 1 {
        s[k] = (s[k - 24] + s[k - 55] + 1000000) % 1000000;
    }

    let mut cuboids: Vec<Cuboid> = Vec::with_capacity(limite);

    for n in 1..=limite {
        let x = (s[6 * n - 5] % 10000, 1 + s[6 * n - 2] % 399);
        let y = (s[6 * n - 4] % 10000, 1 + s[6 * n - 1] % 399);
        let z = (s[6 * n - 3] % 10000, 1 + s[6 * n] % 399);
        cuboids.push((x, y, z));
    }

    let mut intersections: Vec<(Cuboid, bool)> = Vec::new();
    for c in &cuboids {
        let mut tmp = vec![(*c, true)];
        for (cube, mark) in &intersections {
            if let Some(p) = intersection_c(cube, c) {
                tmp.push((p, !mark));
            }
        }

        intersections.extend(tmp);
    }

    let mut result_pos = 0;
    let mut result_neg = 0;
    for (c, mark) in intersections {
        if mark {
            result_pos += volume(&c);
        } else {
            result_neg += volume(&c);
        }
    }

    let result = result_pos - result_neg;
    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem212::problem212;

    #[test]
    fn test_problem212() {
        let result = problem212();
        assert_eq!(result, "328968937309");
    }
}
