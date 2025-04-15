use std::iter::zip;

use crate::MousePosition;
use crate::components::{Position, Rotation, Size, Tangible};
use crate::debug::bounding_box::BoundingBoxColor;
use crate::mouse::get_clicked_entity;
use crate::shapes::{Shape, ShapeImpl};

use bevy::input::common_conditions::{input_just_pressed, input_just_released, input_pressed};
use bevy::math::DVec2;
use bevy::prelude::*;

use super::{GameScene, despawn_scene};
use crate::spawners::Spawner;

#[derive(Component)]
struct CollisionTestEntity;

#[derive(Component)]
struct ShapeMoverEntity;

#[derive(Component)]
struct Pointer(Entity);

#[derive(Component)]
struct ShapeMoverOffset(DVec2);

pub struct CollisionTestPlugin;

impl Plugin for CollisionTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScene::CollisionTest), shapes_setup)
            .add_systems(
                Update,
                (
                    highlight_colliding,
                    create_shape_mover.run_if(input_just_pressed(MouseButton::Left)),
                    move_shape.run_if(input_pressed(MouseButton::Left)),
                    destroy_shape_mover.run_if(input_just_released(MouseButton::Left)),
                ),
            )
            .add_systems(
                OnExit(GameScene::CollisionTest),
                despawn_scene::<CollisionTestEntity>,
            );
    }
}

#[allow(clippy::type_complexity)]
fn highlight_colliding(
    mut query: Query<
        (
            Entity,
            &Shape,
            &Position,
            &Size,
            &Rotation,
            &mut BoundingBoxColor,
        ),
        With<CollisionTestEntity>,
    >,
) {
    let mut colliding_entities = Vec::new();
    for (entity1, shape1, position1, size1, rotation1, _) in &query {
        let data1 = (*position1, *size1, *rotation1).into();
        for (entity2, shape2, position2, size2, rotation2, _) in &query {
            if entity1 == entity2 {
                continue;
            }
            let data2 = (*position2, *size2, *rotation2).into();

            if shape1.collides_with_shape(&data1, shape2, &data2) {
                colliding_entities.push(entity1);
            }
        }
    }

    for (_, _, _, _, _, mut color) in &mut query {
        color.0 = Color::srgba(0.0, 0.0, 0.0, 0.0);
    }

    for entity in colliding_entities {
        let Ok((_, _, _, _, _, mut color)) = query.get_mut(entity) else {
            continue;
        };

        color.0 = Color::srgb_u8(0, 100, 200);
    }
}

fn shapes_setup(
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
        Spawner::new(CollisionTestEntity, &mut commands)
            .with_bundle((
                Tangible,
                Size {
                    width: size * 0.75,
                    height: size,
                },
                Position(DVec2::new(start + step_size * i as f64, 0.0)),
                Rotation(i as f64 * 0.3),
            ))
            .with_shape(shape, &mut meshes)
            .with_color(Color::from(color), &mut materials);
    }
}

fn create_shape_mover(
    mouse_position_resource: Res<MousePosition>,
    entity_query: Query<(Entity, &Shape, &Position, &Size, &Rotation), With<CollisionTestEntity>>,
    mut commands: Commands,
) {
    let mouse_position = mouse_position_resource.0.as_dvec2();

    let Some((clicked_entity, entity_position)) = get_clicked_entity(mouse_position, &entity_query)
    else {
        return;
    };

    Spawner::new(ShapeMoverEntity, &mut commands).with_bundle((
        Position(mouse_position),
        ShapeMoverOffset(entity_position - mouse_position),
        Pointer(clicked_entity),
    ));
}

fn move_shape(
    mouse_position_resource: Res<MousePosition>,
    mut mover_query: Query<(&mut Position, &ShapeMoverOffset, &Pointer), With<ShapeMoverEntity>>,
    mut position_query: Query<&mut Position, Without<ShapeMoverEntity>>,
) {
    let mouse_position = mouse_position_resource.0.as_dvec2();

    for (mut mover_position, offset, moving_entity) in &mut mover_query {
        let Ok(mut entity_position) = position_query.get_mut(moving_entity.0) else {
            return;
        };

        mover_position.0 = mouse_position;
        entity_position.0 = mover_position.0 + offset.0;
    }
}

fn destroy_shape_mover(
    mouse_entity_query: Query<Entity, With<ShapeMoverEntity>>,
    mut commands: Commands,
) {
    for entity in &mouse_entity_query {
        commands.entity(entity).despawn_recursive();
    }
}
