use std::f32::consts::PI;
use core::hash::{Hash, Hasher};

use cgmath::Angle;

use crate::rendering::model::ModelVertex;
use crate::shapes::shape::Shape;

fn spring_top_curve(t: f32, cc: f32, d: f32) -> (f32, f32) {
    let (sin, cos) = cgmath::Rad(cc * 2.0 * PI * t).sin_cos();
    let f = 0.5 * (1.0 - d) * sin;
    let fp = 0.5 * (1.0 - d) * cc * 2.0 * PI * cos;
    let nfp = (1.0 + fp.powi(2)).sqrt();

    (t - 0.5 * d * fp / nfp, f + 0.5 * d / nfp)
}

fn spring_bottom_curve(t: f32, cc: f32, d: f32) -> (f32, f32) {
    let (sin, cos) = cgmath::Rad(cc * 2.0 * PI * t).sin_cos();
    let f = 0.5 * (1.0 - d) * sin;
    let fp = 0.5 * (1.0 - d) * cc * 2.0 * PI * cos;
    let nfp = (1.0 + fp.powi(2)).sqrt();

    (t + 0.5 * d * fp / nfp, f - 0.5 * d / nfp)
}

fn interpolate(x: f32, points: &Vec<(f32, f32)>) -> f32 {
    let mut p = 0.0;
    for i in 0..points.len() {
        let mut l = 1.0;
        for j in 0..points.len() {
            if j == i {
                continue;
            }
            l *= (x - points[j].0) / (points[i].0 - points[j].0);
        }
        p += l * points[i].1;
    }

    p
}

fn curve2function(x: f32, delta: f32, f: impl Fn(f32) -> (f32, f32)) -> f32 {
    let (fist_px, fist_py) = f(x);
    if (fist_px - x).abs() < 1e-8 {
        return fist_py;
    }

    let sign = (x - fist_px).signum();

    let mut nx = x + delta * sign;
    while sign == (x - f(nx).0).signum() {
        nx += delta * sign;
    }

    let points = vec![
        f(nx - delta * sign),
        f(nx - 0.25 * delta * sign),
        f(nx - 0.5 * delta * sign),
        f(nx - 0.75 * delta * sign),
        f(nx),
    ];

    interpolate(x, &points)
}

#[derive(Debug, Clone, Copy)]
pub struct Spring {
    coil_count: u32,
    coil_diameter: f32,
}

impl Spring {
    pub fn new(coil_count: u32, coil_diameter: f32) -> Self {
        Self {coil_count, coil_diameter}
    }

    fn canonicalize(&self) -> (u32, i64) {
        // to be able to hash shape, we need to do something with our float.
        // diameters smaller than 0.001 don't show up on screen anyway
        (self.coil_count, (self.coil_diameter * 1000.0).round() as i64)
    } 
}

impl PartialEq for Spring {
    fn eq(&self, other: &Spring) -> bool {
        self.canonicalize() == other.canonicalize()
    }
}

impl Eq for Spring {}

impl Hash for Spring {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.canonicalize().hash(state);
    }
}

impl Shape for Spring {
    fn get_vertices(&self) -> Vec<[f32; 2]> {
        let n = 20 * self.coil_count;
        let delta = 0.01 / (self.coil_diameter.sqrt() * self.coil_count.pow(2) as f32);
        let num_points = n * 2 + 1;
        let dx = 1.0 / (n - 1) as f32;

        let top_curve = |t: f32| spring_top_curve(t, self.coil_count as f32, self.coil_diameter);
        let bottom_curve = |t: f32| spring_bottom_curve(t, self.coil_count as f32, self.coil_diameter);

        let mut points = Vec::with_capacity(num_points as usize);
        points.push([-0.5, curve2function(-0.5, delta, top_curve)]);
        points.push([-0.5, curve2function(-0.5, delta, bottom_curve)]);
        for i in 1..n {
            let x = -0.5 + (i as f32) * dx;
            points.push([x, curve2function(x, delta, top_curve)]);
            points.push([
                x - 0.5 * dx,
                curve2function(x - 0.5 * dx, delta, bottom_curve),
            ]);
        }
        points.push([0.5, curve2function(0.5, delta, bottom_curve)]);

        points
    }

    fn get_model(&self) -> (Vec<ModelVertex>, Vec<u32>) {
        let n = 20 * self.coil_count;
        let num_indices = 3 * (2 * n - 1);
        let mut indices = Vec::with_capacity(num_indices as usize);

        for i in 0..n {
            if i != 0 {
                indices.push(2 * i);
                indices.push(2 * i + 1);
                indices.push(2 * (i - 1));
            }
            if i != n - 1 {
                indices.push(2 * i + 3);
                indices.push(2 * i + 1);
                indices.push(2 * i);
            }
        }
        indices.push(2 * n);
        indices.push(2 * n - 1);
        indices.push(2 * n - 2);

        (self.to_model_vertices(), indices)
    }
}
