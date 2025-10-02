use crate::components::{Position, Size, Tangible};
use crate::shapes::Shape;
use crate::spawners::circle::physics_circle_bundle;

use bevy::math::DVec2;
use bevy::prelude::*;

use super::{GameScene, despawn_scene};
use crate::spawners::Spawner;

#[derive(Component)]
struct BouncyBallsEntity;

pub struct BouncyBallsPlugin;

impl Plugin for BouncyBallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScene::BouncyBalls), bouncy_balls_setup)
            .add_systems(
                OnExit(GameScene::BouncyBalls),
                despawn_scene::<BouncyBallsEntity>,
            );
    }
}

fn spawn_wall(
    position: DVec2,
    width: f64,
    height: f64,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    Spawner::new(BouncyBallsEntity, commands)
        .with_bundle((Position(position), Size { width, height }, Tangible))
        .with_shape(Shape::Square, meshes)
        .with_color(Color::srgb_u8(150, 50, 100), materials)
        .id()
}

fn spawn_ball(
    position: DVec2,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    Spawner::new(BouncyBallsEntity, commands)
        .with_bundle(physics_circle_bundle(0.1, 0.5, position))
        .with_shape(Shape::Circle, meshes)
        .with_color(Color::srgb_u8(0, 100, 200), materials)
        .with_z_value(-1.0)
        .id()
}

pub fn bouncy_balls_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    debug!("Setting up bouncy ball");

    let width = 0.1;
    let top = 1.5;
    let bottom = -1.5;
    let left = -2.5;
    let right = 2.5;
    spawn_wall(
        DVec2::new(0.0, top + width / 2.0),
        right - left,
        width,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    spawn_wall(
        DVec2::new(0.0, bottom - width / 2.0),
        right - left,
        width,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    spawn_wall(
        DVec2::new(right + width / 2.0, 0.0),
        width,
        top - bottom,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    spawn_wall(
        DVec2::new(left - width / 2.0, 0.0),
        width,
        top - bottom,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    for _ in 0..10 {
        let x = rand::random_range(left..right);
        let y = rand::random_range(bottom..top);
        spawn_ball(DVec2::new(x, y), &mut commands, &mut meshes, &mut materials);
    }
}
