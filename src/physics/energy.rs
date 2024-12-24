use bevy::prelude::*;

use crate::{components::*, Energy};

use crate::physics::gravity::gravitational_potential_energy;
use crate::physics::spring::spring_potential_energy;

pub fn calculate_total_energy(
    timer: Res<Time>,
    mut total_energy_resource: ResMut<Energy>,
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
    let previous_value = total_energy_resource.0;
    let ema_smoothing_factor = 0.1;
    let delta = timer.delta_secs_f64();
    let alpha = (delta / ema_smoothing_factor).clamp(0.0, 1.0);
    total_energy_resource.0 = previous_value + alpha * (total_energy - previous_value);
}
