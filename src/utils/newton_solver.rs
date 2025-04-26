#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NewtonResult {
    pub value: f64,
    pub iterations: u32,
    pub converged: bool,
}

pub fn global_newton_solver(x0: f64, f_fp_func: impl Fn(f64) -> (f64, f64)) -> NewtonResult {
    const MAX_ITERS: u32 = 10;
    const EPS: f64 = 1e-12;

    let mut x = x0;
    let mut i = 1;

    let (mut f, mut fp) = f_fp_func(x);
    loop {
        let step = f / fp;
        if step.abs() < EPS {
            return NewtonResult {
                value: x,
                iterations: i,
                converged: true,
            };
        }
        x -= step;

        i += 1;
        if i > MAX_ITERS {
            return NewtonResult {
                value: x,
                iterations: i,
                converged: false,
            };
        }

        let prev_f = f;
        (f, fp) = f_fp_func(x);

        if f.abs() > prev_f.abs() {
            // step increased f! Try taking a smaller step
            x += step;
            let mut damping = 0.5;
            loop {
                (f, fp) = f_fp_func(x - step * damping);
                if f.abs() < prev_f.abs() {
                    // yay, with the damping, f decreased!
                    x -= step * damping;
                    break;
                }

                damping *= 0.5;
                if damping < 1e-4 {
                    // damping is approaching zero but f is still increasing,
                    // give up :(
                    return NewtonResult {
                        value: x,
                        iterations: i,
                        converged: false,
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn test_newton_solver() {
        // use newton to calculate sqrt(a) by solving x^2 - a = 0,
        // so f(x) = x^2 - a, f'(x) = 2x.
        fn get_ratio_func(a: f64) -> impl Fn(f64) -> (f64, f64) {
            move |x| (x * x - a, 2.0 * x)
        }

        for a in [2.0, 3.0, PI, 121.0] {
            let result = global_newton_solver(a, get_ratio_func(a));
            assert!(result.converged);
            assert!((result.value - a.sqrt()) < 1e-10);
        }

        // use newton to find zero of arctan(x) - regular newton fails to do
        // this when |x| ≳ 1.5, but global newton should manage.
        let arctan_ratio_func = |x: f64| (x.atan(), 1.0 / (x * x + 1.0));

        for x0 in [0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0] {
            let result = global_newton_solver(x0, arctan_ratio_func);
            assert!(result.converged);
            assert!(result.value.abs() < 1e-10);
        }
    }
}
