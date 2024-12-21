use bevy::math::DVec2;
use bevy::prelude::*;

use crate::components::*;

pub trait Integrator {
    fn build<M>(&self, app: &mut App, apply_forces: impl IntoSystemConfigs<M> + Copy);
}

// TODO: Implement RK4
#[allow(dead_code)]
pub enum Integrators {
    EulerChromer,
    VelocityVerlet,
}

impl Integrator for Integrators {
    fn build<M>(&self, app: &mut App, apply_forces: impl IntoSystemConfigs<M> + Copy) {
        match self {
            Integrators::EulerChromer => EulerChromerStep.build(app, apply_forces),
            Integrators::VelocityVerlet => VelocityVerletStep.build(app, apply_forces),
        };
    }
}

struct EulerChromerStep;
impl EulerChromerStep {
    fn step(timer: Res<Time>, mut query: Query<(&mut Position, &mut PhysicsObject)>) {
        let dt = timer.delta_secs_f64();
        if dt > 1.0 / 30.0 {
            bevy::log::warn!("Ignoring a large step size equal to {}", dt);
            for (_, mut physics_object) in query.iter_mut() {
                physics_object.acceleration = DVec2::ZERO;
            }
            return;
        }

        for (mut position, mut physics_object) in query.iter_mut() {
            let acceleration = physics_object.acceleration;
            physics_object.acceleration = DVec2::ZERO;

            physics_object.velocity += acceleration * dt;
            position.0 += physics_object.velocity * dt;
        }
    }
}

impl Integrator for EulerChromerStep {
    fn build<M>(&self, app: &mut App, apply_forces: impl IntoSystemConfigs<M> + Copy) {
        app.add_systems(
            FixedUpdate,
            // Use velocity verlet integration
            (apply_forces, Self::step).chain(),
        );
    }
}

struct VelocityVerletStep;
impl VelocityVerletStep {
    /// Runs before acceleration is calculated => uses previous acceleration
    fn update_positions(timer: Res<Time>, mut query: Query<(&mut Position, &mut PhysicsObject)>) {
        let dt = timer.delta_secs_f64();
        if dt > 1.0 / 30.0 {
            return;
        }

        for (mut position, mut physics_object) in query.iter_mut() {
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
        if dt > 1.0 / 30.0 {
            bevy::log::warn!("Ignoring a large step size equal to {}", dt);
            for mut physics_object in query.iter_mut() {
                physics_object.acceleration = DVec2::ZERO;
            }
            return;
        }

        for mut physics_object in query.iter_mut() {
            let acceleration = physics_object.acceleration;
            physics_object.velocity += 0.5 * acceleration * dt;
        }
    }
}

impl Integrator for VelocityVerletStep {
    fn build<M>(&self, app: &mut App, apply_forces: impl IntoSystemConfigs<M> + Copy) {
        app.add_systems(PostStartup, apply_forces).add_systems(
            FixedUpdate,
            // Use velocity verlet integration
            (
                Self::update_positions,
                apply_forces,
                Self::update_velocities,
            )
                .chain(),
        );
    }
}
