use bevy::math::DVec2;
use bevy::prelude::*;

use crate::components::{PhysicsObject, Position};

/// Smallest allowable dt
const DT_THRESHOLD: f64 = 1.0 / 30.0;

macro_rules! check_dt_size {
    ($dt:ident, $physics_iter:expr) => {
        if $dt > DT_THRESHOLD {
            warn!("Ignoring a large step size equal to {}", $dt);
            for mut physics_object in $physics_iter {
                physics_object.acceleration = DVec2::ZERO;
            }
            return;
        }
    };
}

pub trait Integrator {
    fn build<M>(&self, app: &mut App, apply_forces: impl IntoSystemConfigs<M> + Copy);
}

// TODO: Implement RK4
#[allow(dead_code)]
pub enum Integrators {
    Euler,
    EulerChromer,
    VelocityVerlet,
}

impl Integrator for Integrators {
    fn build<M>(&self, app: &mut App, apply_forces: impl IntoSystemConfigs<M> + Copy) {
        match self {
            Self::Euler => EulerStep.build(app, apply_forces),
            Self::EulerChromer => EulerChromerStep.build(app, apply_forces),
            Self::VelocityVerlet => VelocityVerletStep.build(app, apply_forces),
        };
    }
}

struct EulerStep;
impl EulerStep {
    fn step(timer: Res<Time>, mut query: Query<(&mut Position, &mut PhysicsObject)>) {
        let dt = timer.delta_secs_f64();
        check_dt_size!(dt, query.iter_mut().map(|(_, p)| p));

        for (mut position, mut physics_object) in &mut query {
            let acceleration = physics_object.acceleration;
            physics_object.acceleration = DVec2::ZERO;

            position.0 += physics_object.velocity * dt;
            physics_object.velocity += acceleration * dt;
        }
    }
}

impl Integrator for EulerStep {
    fn build<M>(&self, app: &mut App, apply_forces: impl IntoSystemConfigs<M> + Copy) {
        app.add_systems(FixedUpdate, (apply_forces, Self::step).chain());
    }
}

struct EulerChromerStep;
impl EulerChromerStep {
    fn step(timer: Res<Time>, mut query: Query<(&mut Position, &mut PhysicsObject)>) {
        let dt = timer.delta_secs_f64();
        check_dt_size!(dt, query.iter_mut().map(|(_, p)| p));

        for (mut position, mut physics_object) in &mut query {
            let acceleration = physics_object.acceleration;
            physics_object.acceleration = DVec2::ZERO;

            physics_object.velocity += acceleration * dt;
            position.0 += physics_object.velocity * dt;
        }
    }
}

impl Integrator for EulerChromerStep {
    fn build<M>(&self, app: &mut App, apply_forces: impl IntoSystemConfigs<M> + Copy) {
        app.add_systems(FixedUpdate, (apply_forces, Self::step).chain());
    }
}

struct VelocityVerletStep;
impl VelocityVerletStep {
    /// Runs before acceleration is calculated => uses previous acceleration
    fn update_positions(timer: Res<Time>, mut query: Query<(&mut Position, &mut PhysicsObject)>) {
        let dt = timer.delta_secs_f64();
        if dt > DT_THRESHOLD {
            return;
        }

        for (mut position, mut physics_object) in &mut query {
            let acceleration = physics_object.acceleration;
            physics_object.acceleration = DVec2::ZERO;

            // half velocity step
            physics_object.velocity += 0.5 * acceleration * dt;
            position.0 += physics_object.velocity * dt;
        }
    }

    /// Runs after acceleration is calculated => uses new acceleration
    fn update_velocities(timer: Res<Time>, mut query: Query<&mut PhysicsObject>) {
        let dt = timer.delta_secs_f64();
        check_dt_size!(dt, query.iter_mut());

        for mut physics_object in &mut query {
            let acceleration = physics_object.acceleration;
            physics_object.velocity += 0.5 * acceleration * dt;
        }
    }
}

impl Integrator for VelocityVerletStep {
    fn build<M>(&self, app: &mut App, apply_forces: impl IntoSystemConfigs<M> + Copy) {
        app.add_systems(PostStartup, apply_forces).add_systems(
            FixedUpdate,
            (
                Self::update_positions,
                apply_forces,
                Self::update_velocities,
            )
                .chain(),
        );
    }
}
