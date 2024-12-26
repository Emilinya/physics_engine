use crate::components::*;
use crate::shapes::{spring, square};

use bevy::math::DVec2;
use bevy::prelude::*;

use super::{despawn_scene, GameScene};
use crate::spawners::{spring::spring_bundle, square::physics_square_bundle, Spawner};

#[derive(Component)]
struct BouncyCastleEntity;

pub struct BouncyCastlePlugin;

impl Plugin for BouncyCastlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScene::BouncyCastle), bouncy_castle_setup)
            .add_systems(
                OnExit(GameScene::BouncyCastle),
                despawn_scene::<BouncyCastleEntity>,
            );
    }
}

fn spawn_square(
    position: DVec2,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    Spawner::new(BouncyCastleEntity, commands)
        .with_bundle(physics_square_bundle(0.1, 0.5, 0.5, position))
        .with_mesh(square::Square, meshes)
        .with_color(Color::srgb_u8(150, 50, 100), materials)
        .id()
}

fn spawn_spring(
    entity1: Entity,
    entity2: Entity,
    strength: f64,
    length: f64,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    Spawner::new(BouncyCastleEntity, commands)
        .with_bundle(spring_bundle(0.1, entity1, entity2, strength, length))
        .with_mesh(
            spring::Spring {
                coil_count: 20,
                coil_diameter: 0.01,
            },
            meshes,
        )
        .with_color(Color::srgb_u8(0, 100, 200), materials)
        .with_z_value(-1.0)
        .id()
}

pub fn bouncy_castle_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    debug!("Setting up bouncy castle");

    let fixed_point = commands
        .spawn((BouncyCastleEntity, Position(DVec2::new(0.0, 2.5))))
        .id();

    let top_square = spawn_square(
        DVec2::new(0.0, 1.5),
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    spawn_spring(
        fixed_point,
        top_square,
        80.0,
        1.0,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    let left_square_top = spawn_square(
        DVec2::new(-1.0, 0.5),
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    spawn_spring(
        top_square,
        left_square_top,
        20.0,
        2f64.sqrt(),
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    let right_square_top = spawn_square(
        DVec2::new(1.0, 0.5),
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    spawn_spring(
        top_square,
        right_square_top,
        20.0,
        2f64.sqrt(),
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    spawn_spring(
        right_square_top,
        left_square_top,
        1.0,
        2.0,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    let left_square_bottom = spawn_square(
        DVec2::new(-1.0, -0.5),
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    spawn_spring(
        left_square_top,
        left_square_bottom,
        10.0,
        1.0,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    let right_square_bottom = spawn_square(
        DVec2::new(1.0, -0.5),
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    spawn_spring(
        right_square_top,
        right_square_bottom,
        10.0,
        1.0,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    spawn_spring(
        right_square_bottom,
        left_square_bottom,
        1.0,
        2.0,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}
