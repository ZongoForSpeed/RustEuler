use crate::register_problem;

register_problem!(262, "Mountain Range", problem262);

/// Elevation function h(x, y).
fn h(x: f64, y: f64) -> f64 {
    let poly = 5000.0 - 0.005 * (x * x + y * y + x * y) + 12.5 * (x + y);
    let exponent = -(0.000001 * (x * x + y * y) - 0.0015 * (x + y) + 0.7).abs();
    poly * exponent.exp()
}

/// Numerical derivative of h with respect to x.
fn dh_dx(x: f64, y: f64) -> f64 {
    const EPS: f64 = 1e-6;
    (h(x + EPS, y) - h(x - EPS, y)) / (2.0 * EPS)
}

/// Bisection method to find the root of f(x) = target in [low, high].
fn bisect<F>(mut low: f64, mut high: f64, target: f64, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    for _ in 0..100 {
        let mid = (low + high) / 2.0;
        if f(mid) < target {
            high = mid;
        } else {
            low = mid;
        }
    }
    (low + high) / 2.0
}

/// Traces the path along the constant elevation f_min from the starting point (x_start, 0)
/// towards the objective (obj_x, obj_y).
fn walk_along_isoline(x_start: f64, f_min: f64, obj_x: f64, obj_y: f64, step_sign: f64) -> f64 {
    let mut total_distance = 0.0;
    let mut curr_x = x_start;
    let mut curr_y = 0.0;
    let mut prev_slope_sign = None;
    let mut alpha = x_start; // alpha = x + y

    loop {
        alpha += step_sign;
        // Find x such that elevation(x, alpha - x) = f_min.
        // We search in a range around the previous curr_x.
        let next_x = bisect(curr_x - 5.0, curr_x + 5.0, f_min, |x| h(x, alpha - x));
        let next_y = alpha - next_x;

        // Vector from current point to next point.
        let dx = next_x - curr_x;
        let dy = next_y - curr_y;

        // Current slope of the path (dy/dx).
        let path_slope = dy / dx;

        // We check if the line passing through (next_x, next_y) with slope path_slope
        // passes through the objective.
        // Line equation: Y - next_y = path_slope * (X - next_x)
        // At X = obj_x, Y = next_y + path_slope * (obj_x - next_x)
        let y_at_obj_x = next_y + path_slope * (obj_x - next_x);
        let vertical_diff = y_at_obj_x - obj_y;

        if let Some(prev_sign) = prev_slope_sign
            && prev_sign * vertical_diff < 0.0
            && vertical_diff.abs() < 100.0
        {
            // We crossed the target point's "view line".
            // For simplicity, we just add the distance to the objective.
            return total_distance + (curr_x - obj_x).hypot(curr_y - obj_y);
        }

        total_distance += dx.hypot(dy);
        curr_x = next_x;
        curr_y = next_y;
        prev_slope_sign = Some(vertical_diff);
    }
}

/// The following equation represents the continuous topography of a mountainous region, giving the elevation h at
/// any point (x,y):
///
///                             p262_formula1.gif
///
/// A mosquito intends to fly from A(200,200) to B(1400,1400), without leaving the area given by 0 ≤ x, y ≤ 1600.
///
/// Because of the intervening mountains, it first rises straight up to a point A', having elevation f. Then, while
/// remaining at the same elevation f, it flies around any obstacles until it arrives at a point B' directly above B.
///
/// First, determine fmin which is the minimum constant elevation allowing such a trip from A to B, while remaining
/// in the specified area. Then, find the length of the shortest path between A' and B', while flying at that
/// constant elevation fmin.
///
/// Give that length as your answer, rounded to three decimal places.
///
/// Note: For convenience, the elevation function shown above is repeated below, in a form suitable for most
/// programming languages:
/// h = ( 5000-0.005*(x*x+y*y+x*y)+12.5*(x+y) ) * exp( -abs(0.000001*(x*x+y*y)-0.0015*(x+y)+0.7) )
pub fn problem262() -> String {
    // The minimum elevation f_min is at the saddle point on the boundary y=0.
    let x_saddle = bisect(0.0, 1600.0, 0.0, |x| dh_dx(x, 0.0));
    let f_min = h(x_saddle, 0.0);

    // Shortest path is from A(200, 200) to the saddle point and then to B(1400, 1400)
    // following the f_min isoline.
    let dist_to_a = walk_along_isoline(x_saddle, f_min, 200.0, 200.0, -1.0);
    let dist_to_b = walk_along_isoline(x_saddle, f_min, 1400.0, 1400.0, 1.0);

    format!("{:.3}", dist_to_a + dist_to_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem262() {
        assert_eq!(problem262(), "2531.205");
    }
}
