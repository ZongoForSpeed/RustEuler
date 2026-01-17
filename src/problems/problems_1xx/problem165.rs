use crate::register_problem;
use fraction::Fraction;
use num_traits::{ConstOne, ConstZero, Zero};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

register_problem!(165, "Intersections", problem165);

#[derive(Eq, Debug, Hash)]
struct Point {
    x: Fraction,
    y: Fraction,
}

#[derive(Eq, Debug, PartialEq)]
struct Segment {
    p1: Point,
    p2: Point,
}

impl Point {
    fn new(x: Fraction, y: Fraction) -> Point {
        Point { x, y }
    }
}

impl Segment {
    fn new(p1: Point, p2: Point) -> Segment {
        Segment { p1, p2 }
    }

    fn intersection(&self, s: &Segment) -> Option<Point> {
        let d = (s.p2.y - s.p1.y) * (self.p2.x - self.p1.x)
            - (s.p2.x - s.p1.x) * (self.p2.y - self.p1.y);
        if d.is_zero() {
            return None;
        }

        let u = ((s.p2.x - s.p1.x) * (self.p1.y - s.p1.y)
            - (s.p2.y - s.p1.y) * (self.p1.x - s.p1.x))
            / d;
        let v = ((self.p2.x - self.p1.x) * (self.p1.y - s.p1.y)
            - (self.p2.y - self.p1.y) * (self.p1.x - s.p1.x))
            / d;
        if (u <= Fraction::ZERO)
            || (u >= Fraction::ONE)
            || (v <= Fraction::ZERO)
            || (v >= Fraction::ONE)
        {
            return None;
        }

        Some(Point::new(
            self.p1.x + u * (self.p2.x - self.p1.x),
            self.p1.y + u * (self.p2.y - self.p1.y),
        ))
    }
}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd<Self> for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x != other.x {
            return self.x.cmp(&other.x);
        }

        self.y.cmp(&other.y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}, {}]", self.p1, self.p2))
    }
}

pub fn problem165() -> String {
    // A segment is uniquely defined by its two endpoints.
    // By considering two line segments in plane geometry there are three possibilities: the segments have zero points,
    // one point, or infinitely many points in common.
    //
    // Moreover when two segments have exactly one point in common it might be the case that that common point is an
    // endpoint of either one of the segments or of both. If a common point of two segments is not an endpoint of either
    // of the segments it is an interior point of both segments.
    // We will call a common point T of two segments L1 and L2 a true intersection point of L1 and
    // L2 if T is the only common point of L1 and L2 and T is an interior point of both segments.
    //
    // Consider the three segments L1, L2, and L3:
    //
    //          L1: (27, 44) to (12, 32)
    //          L2: (46, 53) to (17, 62)
    //          L3: (46, 70) to (22, 40)
    //
    // It can be verified that line segments L2 and L3 have a true intersection point. We note that as the one of the
    // end points of L3: (22,40) lies on L1 this is not considered to be a true point of intersection. L1 and L2 have no
    // common point. So among the three line segments, we find one true intersection point.
    //
    // Now let us do the same for 5000 line segments. To this end, we generate 20000 numbers using the so-called
    // "Blum Blum Shub" pseudo-random number generator.
    //
    //          s0 = 290797
    //          sn+1 = sn×sn (modulo 50515093)
    //          tn = sn (modulo 500)
    //
    // To create each line segment, we use four consecutive numbers tn. That is, the first line segment is given by:
    //
    // (t1, t2) to (t3, t4)
    //
    // The first four numbers computed according to the above generator should be: 27, 144, 12 and 232. The first
    // segment would thus be (27,144) to (12,232).
    //
    // How many distinct true intersection points are found among the 5000 line segments?
    let limite = 20000;

    let l1 = Segment::new(Point::new(27.into(), 44.into()), Point::new(12.into(), 32.into()));
    let l2 = Segment::new(Point::new(46.into(), 53.into()), Point::new(17.into(), 62.into()));
    let l3 = Segment::new(Point::new(46.into(), 70.into()), Point::new(22.into(), 40.into()));

    println!("L1 = {l1}");
    println!("L2 = {l2}");
    println!("L3 = {l3}");

    if let Some(p1) = l1.intersection(&l2) {
        println!("L1 x L2 = {p1}");
    }

    if let Some(p1) = l1.intersection(&l3) {
        println!("L1 x L3 = {p1}");
    }
    if let Some(p1) = l2.intersection(&l3) {
        println!("L2 x L3 = {p1}");
    }

    let mut t:Vec<u64> = Vec::new();
    let mut s = 290797;
    for _ in 0..limite {
        s = (s * s) % 50515093;
        t.push(s % 500);
    }

    let mut segments = Vec::new();
    for n in (0..limite).step_by(4) {
        segments.push(Segment::new(Point::new(t[n].into(),t[n + 1].into()), Point::new(t[n+2].into(), t[n+3].into())));
    }

    let mut intersection = HashSet::new();

    for i in 0..segments.len() {
        for j in (i + 1)..segments.len() {
            if let Some(p) = segments[i].intersection(&segments[j]) {
                intersection.insert(p);
            }
        }
    }

    intersection.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem165() {
        let result = problem165();
        assert_eq!(result, "2868868");
    }
}
