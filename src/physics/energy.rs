use bevy::prelude::*;

use crate::{components::*, TotalEnergy};

use crate::physics::gravity::gravitational_potential_energy;
use crate::physics::spring::spring_potential_energy;

pub fn calculate_total_energy(
    timer: Res<Time>,
    mut total_energy_resource: ResMut<TotalEnergy>,
    spring_query: Query<(&SpringForce, &Connection)>,
    position_query: Query<&Position, Without<Spring>>,
    body_query: Query<(&Position, &PhysicsObject)>,
) {
    // calculate kinetic energy
    let mut total_energy = body_query.iter().fold(0.0, |acc, (_, physics_object)| {
        acc + 0.5 * physics_object.mass * physics_object.velocity.length_squared()
    });

    // calculate potential energies
    total_energy += gravitational_potential_energy(body_query);
    total_energy += spring_potential_energy(spring_query, position_query);

    // update total energy counter
    if let Some(previous_value) = total_energy_resource.current {
        let ema_smoothing_factor = 0.2;
        let delta = timer.delta_secs_f64();
        let alpha = (delta / ema_smoothing_factor).clamp(0.0, 1.0);
        total_energy_resource.current =
            Some(previous_value + alpha * (total_energy - previous_value));
    } else {
        total_energy_resource.current = Some(total_energy);
        total_energy_resource.initial = Some(total_energy);
    }
}
