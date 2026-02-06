use crate::register_problem;
use crate::utils::timer::ScopeTimer;
use std::collections::HashMap;

register_problem!(252, "Convex Holes", problem252);

const EPSILON: f64 = 0.000001;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Debug)]
struct Polygon {
    points: Vec<Point>,
    area: f64,
}

impl Polygon {
    fn new(points: Vec<Point>, area: f64) -> Polygon {
        Polygon { points, area }
    }

    fn contains_point(&self, c: &Point) -> bool {
        let n = self.points.len();
        let mut accumulated_area = 0.;
        for k in 0..n {
            accumulated_area += area(&self.points[k], &self.points[(k + 1) % n], c);
        }
        (accumulated_area - self.area).abs() < 0.000001
    }

    fn convex(&self) -> bool {
        let n = self.points.len();
        let mut signum = 0;
        for k in 0..n {
            let d = determinant(
                &self.points[k],
                &self.points[(k + 1) % n],
                &self.points[(k + 2) % n],
            );
            if signum == 0 {
                signum = sign(d);
            } else if sign(d) != signum {
                return false;
            }
        }

        true
    }

    fn add(&self, p: &Point, area: f64) -> Polygon {
        let mut points = self.points.clone();
        points.push(p.clone());
        let _area = self.area + area;

        Polygon {
            points,
            area: _area,
        }
    }

    fn first(&self) -> Point {
        self.points[0].clone()
    }

    fn last(&self) -> Point {
        self.points[self.points.len() - 1].clone()
    }
}

fn area(p1: &Point, p2: &Point, p3: &Point) -> f64 {
    let det = (p2.x - p1.x) * (p3.y - p1.y) - (p3.x - p1.x) * (p2.y - p1.y);
    det.abs() as f64 / 2.
}

fn determinant(p1: &Point, p2: &Point, p3: &Point) -> i64 {
    let det = (p2.x - p1.x) * (p3.y - p1.y) - (p3.x - p1.x) * (p2.y - p1.y);
    det
}

fn sign(n: i64) -> i8 {
    if n > 0 {
        1
    } else if n < 0 {
        -1
    } else {
        0
    }
}

fn algorithm(
    polygons: &[Polygon],
    dictionary: &HashMap<(Point, Point), Vec<(Point, f64)>>,
) -> Vec<Polygon> {
    let mut result = Vec::new();
    for item in polygons {
        let edge = (item.first(), item.last());
        if let Some(t) = dictionary.get(&edge) {
            for (p, area) in t {
                if !item.contains_point(p) {
                    let q = item.add(p, *area);
                    if q.convex() {
                        result.push(q);
                    }
                }
            }
        }
    }
    result
}


/// Given a set of points on a plane, we define a convex hole to be a convex
/// polygon having as vertices any of the given points and not containing any
/// of the given points in its interior (in addition to the vertices, other
/// given points may lie on the perimeter of the polygon).
///
/// As an example, the image below shows a set of twenty points and a few such
/// convex holes. The convex hole shown as a red heptagon has an area equal
/// to 1049694.5 square units, which is the highest possible area for a convex
/// hole on the given set of points.
///
/// For our example, we used the first 20 points (T2k−1, T2k), for k = 1,2,…,20,
/// produced with the pseudo-random number generator:
///
///                              S0	    = 	290797
///                              Sn+1	= 	Sn² mod 50515093
///                              Tn  	= 	(Sn mod 2000) − 1000
///
/// i.e. (527, 144), (−488, 732), (−454, −947), …
///
/// What is the maximum area for a convex hole on the set containing the first
/// 500 points in the pseudo-random sequence?
///
/// Specify your answer including one digit after the decimal point.
pub fn problem252() -> String {
    let taille = 500;

    let mut sn = vec![290797];
    for _ in 0..2 * taille {
        let back = sn[sn.len() - 1];
        sn.push((back * back) % 50515093);
    }

    let mut tn = Vec::new();
    for s in sn {
        tn.push((s % 2000) - 1000);
    }

    let mut points = Vec::new();
    for k in 1..=taille {
        points.push(Point {
            x: tn[2 * k - 1],
            y: tn[2 * k],
        });
    }

    let mut triangles = Vec::new();
    {
        let _timer = ScopeTimer::new("Build triangles", false);
        for i in 0..points.len() {
            let a = &points[i];
            for j in i + 1..points.len() {
                let b = &points[j];
                for k in j + 1..points.len() {
                    let c = &points[k];
                    let abc = area(a, b, c);
                    let mut minimal = true;
                    for d in &points {
                        if a == d || b == d || c == d {
                            continue;
                        }
                        if (area(a, b, d) + area(a, d, c) + area(d, b, c) - abc).abs() < EPSILON {
                            minimal = false;
                            break;
                        }
                    }
                    if minimal {
                        let triangle = Polygon::new(vec![a.clone(), b.clone(), c.clone()], abc);
                        triangles.push(triangle);
                    }
                }
            }
        }
    }

    let mut dictionary: HashMap<(Point, Point), Vec<(Point, f64)>> = HashMap::new();
    {
        let _timer = ScopeTimer::new("Build dictionary", false);
        for t in &triangles {
            let p0 = &t.points[0];
            let p1 = &t.points[1];
            let p2 = &t.points[2];
            let area = t.area;
            dictionary
                .entry((p0.clone(), p1.clone()))
                .or_insert_with(Vec::new)
                .push((p2.clone(), area));
            dictionary
                .entry((p1.clone(), p2.clone()))
                .or_insert_with(Vec::new)
                .push((p0.clone(), area));
            dictionary
                .entry((p2.clone(), p0.clone()))
                .or_insert_with(Vec::new)
                .push((p1.clone(), area));
            dictionary
                .entry((p1.clone(), p0.clone()))
                .or_insert_with(Vec::new)
                .push((p2.clone(), area));
            dictionary
                .entry((p2.clone(), p1.clone()))
                .or_insert_with(Vec::new)
                .push((p0.clone(), area));
            dictionary
                .entry((p0.clone(), p2.clone()))
                .or_insert_with(Vec::new)
                .push((p1.clone(), area));
        }
    }

    let mut result = 0.;

    let mut polygons = triangles;
    while !polygons.is_empty() {
        println!("Polygons: {}", polygons.len());
        if let Some(max) = polygons.iter().max_by(|&a, &b| a.area.total_cmp(&b.area)) {
            println!("Max: {:?}", max);
            if max.area > result {
                result = max.area;
            }
            polygons = algorithm(&polygons, &dictionary);
        }
    }

    format!("{:.1}", result)
}

#[cfg(test)]
mod tests {
    use crate::problems::problems_2xx::problem252::problem252;

    #[test]
    fn test_problem252() {
        let result = problem252();
        assert_eq!(result, "104924.0");
    }
}
