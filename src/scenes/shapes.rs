use std::iter::zip;

use crate::MousePosition;
use crate::components::{Position, Rotation, Size, Tangible};
use crate::debug::bounding_box::BoundingBoxColor;
use crate::shapes::{Shape, ShapeImpl};

use bevy::math::DVec2;
use bevy::prelude::*;

use super::{GameScene, despawn_scene};
use crate::spawners::Spawner;

#[derive(Component)]
struct ShapesEntity;

pub struct ShapesPlugin;

impl Plugin for ShapesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScene::Shapes), shapes_setup)
            .add_systems(Update, (highlight_hovered, rotate_shapes))
            .add_systems(OnExit(GameScene::Shapes), despawn_scene::<ShapesEntity>);
    }
}

fn highlight_hovered(
    mouse_position_resource: Res<MousePosition>,
    mut query: Query<
        (&Shape, &Position, &Size, &Rotation, &mut BoundingBoxColor),
        With<ShapesEntity>,
    >,
) {
    let mouse_position = mouse_position_resource.0.as_dvec2();

    for (shape, position, size, rotation, mut color) in &mut query {
        let data = (*position, *size, *rotation).into();
        if shape.collides_with_point(&data, mouse_position) {
            color.0 = Color::srgb_u8(50, 200, 50);
        } else {
            color.0 = Color::BLACK;
        }
    }
}

pub fn shapes_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    debug!("Setting up shapes");

    let shapes = [
        Shape::Square,
        Shape::Triangle,
        Shape::Pentagon,
        Shape::Hexagon,
        Shape::Heptagon,
        Shape::Octagon,
        Shape::Circle,
    ];

    let start_color = LinearRgba::from(Srgba::rgb_u8(91, 206, 250));
    let end_color = LinearRgba::from(Srgba::rgb_u8(245, 169, 184));

    let colors: Vec<_> = (0..shapes.len())
        .map(|i| start_color.mix(&end_color, i as f32 / (shapes.len() - 1) as f32))
        .collect();

    let size = 0.75;
    let step_size = size * 1.25;
    let start = -((shapes.len() - 1) as f64 * 0.5 * step_size);

    for (i, (color, shape)) in zip(colors, shapes).enumerate() {
        for (j, ratio) in [1.0, 0.5].iter().enumerate() {
            Spawner::new(ShapesEntity, &mut commands)
                .with_bundle((
                    Tangible,
                    Size {
                        width: size * ratio,
                        height: size,
                    },
                    Position(DVec2::new(start + step_size * i as f64, j as f64 - 0.5)),
                    Rotation(0.0),
                ))
                .with_shape(shape, &mut meshes)
                .with_color(Color::from(color), &mut materials);
        }
    }
}

fn rotate_shapes(timer: Res<Time>, mut query: Query<&mut Rotation, With<ShapesEntity>>) {
    let dt = timer.delta_secs_f64();
    for mut rotation in &mut query {
        rotation.0 += 0.05 * dt;
    }
}
