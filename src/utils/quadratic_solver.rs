const SQRT_EPSILON: f64 = 1e-5;
const EPSILON: f64 = SQRT_EPSILON * SQRT_EPSILON;

fn is_zero(float: f64) -> bool {
    float.abs() < EPSILON
}

fn sqrt_is_zero(float: f64) -> bool {
    float.abs() < SQRT_EPSILON
}

/// Return all real solutions to the equation `Ax² + Bx + C = 0`.
#[allow(non_snake_case)]
pub fn solve_quadratic(A: f64, B: f64, C: f64) -> Vec<f64> {
    if is_zero(A) {
        return solve_linear(B, C);
    }

    let b = B / A;
    let c = C / A;

    let discriminant = b.powi(2) - 4.0 * c;
    if sqrt_is_zero(discriminant) {
        // discriminant is zero, so we only get one solution
        vec![-b / 2.0]
    } else if discriminant < 0.0 {
        // both solutions are complex
        vec![]
    } else {
        let mid = -b / 2.0;
        let half_width = discriminant.sqrt() / 2.0;
        vec![mid - half_width, mid + half_width]
    }
}

/// Return all real solutions to the equation `Ax + B = 0`.
#[allow(non_snake_case)]
fn solve_linear(A: f64, B: f64) -> Vec<f64> {
    if !is_zero(A) {
        vec![-B / A]
    } else if is_zero(B) {
        vec![0.0]
    } else {
        vec![]
    }
}
