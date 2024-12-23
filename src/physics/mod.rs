mod energy;
mod gravity;
mod integrators;
mod spring;
mod transform;

use bevy::prelude::*;

use energy::calculate_total_energy;
use gravity::apply_gravity;
use integrators::{Integrator, Integrators};
use spring::{apply_spring_force, update_spring};
use transform::update_transform;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        Integrators::VelocityVerlet.build(app, (apply_gravity, apply_spring_force));
        app.add_systems(
            Update,
            (calculate_total_energy, update_transform, update_spring),
        );
    }
}
