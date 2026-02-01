use crate::register_problem;
use crate::utils::epsilon::equal_epsilon;
use std::cmp::{Ordering, Reverse};
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

register_problem!(247, "Squares under a hyperbola", problem247);

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        equal_epsilon(self.x, other.x) && equal_epsilon(self.y, other.y)
    }
}

#[derive(Debug)]
struct Square {
    position: Point,
    index: (i64, i64),
    area: f64,
}

impl Square {
    fn new(position: Point, index: (i64, i64)) -> Self {
        let area = Self::compute_area(&position);
        Self {
            position,
            index,
            area,
        }
    }

    fn compute_area(position: &Point) -> f64 {
        let b = position.y - position.x;
        let delta = b * b + 4.;
        if delta > 0. {
            (f64::sqrt(delta) - b) / 2. - position.x
        } else {
            f64::MAX
        }
    }

    fn next(&self) -> (Square, Square) {
        let x = self.position.x;
        let y = self.position.y;
        let index = self.index;

        let s1 = Square::new(Point::new(x + self.area, y), (index.0 + 1, index.1));
        let s2 = Square::new(Point::new(x, y + self.area), (index.0, index.1 + 1));
        (s1, s2)
    }
}

pub struct OrderedFloat(pub f64);

impl Eq for OrderedFloat {}

impl PartialEq<Self> for OrderedFloat {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 < other.0 {
            Ordering::Less
        } else if self.0 > other.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

pub fn problem247() -> String {
    // Consider the region constrained by 1 ≤ x and 0 ≤ y ≤ 1/x.
    //
    // Let S1 be the largest square that can fit under the curve.
    // Let S2 be the largest square that fits in the remaining area, and so on.
    // Let the index of Sn be the pair (left, below) indicating the number of
    // squares to the left of Sn and the number of squares below Sn.
    //
    // The diagram shows some such squares labelled by number.
    // S2 has one square to its left and none below, so the index of S2 is (1,0).
    // It can be seen that the index of S32 is (1,1) as is the index of S50.
    // 50 is the largest n for which the index of Sn is (1,1).
    //
    // What is the largest n for which the index of Sn is (3,3)?
    let objectif = (3, 3);

    let s0 = Square::new(Point::new(1.0, 0.0), (0, 0));

    let mut pq = BTreeMap::new();
    let area = s0.area;
    pq.insert(Reverse(OrderedFloat(area)), s0);

    let mut result = 0;
    let mut count = 1;
    while count > 0 {
        result += 1;

        let entry = pq.first_entry().unwrap();
        let sn = entry.remove();
        if sn.index.0 <= objectif.0 && sn.index.1 <= objectif.1 {
            count -= 1;
        }

        let (s1, s2) = sn.next();
        if s1.index.0 <= objectif.0 && s1.index.1 <= objectif.1 {
            count += 1;
        }

        if s2.index.0 <= objectif.0 && s2.index.1 <= objectif.1 {
            count += 1;
        }

        let s1_area = s1.area;
        let s2_area = s2.area;

        pq.insert(Reverse(OrderedFloat(s1_area)), s1);
        pq.insert(Reverse(OrderedFloat(s2_area)), s2);

        if count == 0 {
            println!("S_{} = {:?}", result, sn);
            break;
        } else if result % 1000 == 0 {
            println!("S_{} = {:?}", result, sn);
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem247::problem247;

    #[test]
    fn test_problem247() {
        let result = problem247();
        assert_eq!(result, "782252");
    }
}
