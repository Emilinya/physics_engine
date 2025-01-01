use crate::components::Position;
use crate::shapes::{Shape, SpringShape};

use bevy::math::DVec2;
use bevy::prelude::*;

use super::{despawn_scene, GameScene};
use crate::spawners::{spring::spring_bundle, square::physics_square_bundle, Spawner};

#[derive(Component)]
struct SpringPendulumEntity;

pub struct SpringPendulumPlugin;

impl Plugin for SpringPendulumPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScene::SpringPendulum), spring_pendulum_setup)
            .add_systems(
                OnExit(GameScene::SpringPendulum),
                despawn_scene::<SpringPendulumEntity>,
            );
    }
}

pub fn spring_pendulum_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    debug!("Setting up spring pendulum");

    let fixed_point = commands
        .spawn((SpringPendulumEntity, Position(DVec2::new(0.0, 2.0))))
        .id();
    let mut entity1 = fixed_point;

    for i in 0..3 {
        let entity2 = Spawner::new(SpringPendulumEntity, &mut commands)
            .with_bundle(physics_square_bundle(
                0.1,
                0.5,
                0.5,
                DVec2::new(i as f64 + 1.0, 2.0),
            ))
            .with_shape(Shape::Square, &mut meshes)
            .with_color(Color::srgb_u8(10, 10, 200), &mut materials)
            .id();

        Spawner::new(SpringPendulumEntity, &mut commands)
            .with_bundle(spring_bundle(0.1, entity1, entity2, 0.0, 20.0, 1.0))
            .with_shape(
                Shape::Spring(SpringShape {
                    coil_count: 20,
                    coil_diameter: 0.01,
                }),
                &mut meshes,
            )
            .with_color(Color::BLACK, &mut materials)
            .with_z_value(-1.0)
            .id();
        entity1 = entity2;
    }
}
