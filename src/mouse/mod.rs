use crate::components::*;
use crate::shapes::{Shape, SpringShape};
use crate::spawners::{spring::spring_bundle, Spawner};

use bevy::input::common_conditions::*;
use bevy::math::DVec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use core::f64;

pub struct InteractivityPlugin;

impl Plugin for InteractivityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                create_mouse_spring.run_if(input_just_pressed(MouseButton::Left)),
                move_mouse_spring.run_if(input_pressed(MouseButton::Left)),
                destroy_mouse_spring.run_if(input_just_released(MouseButton::Left)),
            ),
        );
    }
}

#[derive(Component)]
struct MouseEntity;

fn create_mouse_spring(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<&Camera>,
    entity_query: Query<(Entity, &Position), With<PhysicsObject>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Some(mouse_position) = get_mouse_position(&window_query, &camera_query) else {
        panic!("Tried to create mouse spring but got no mouse position?");
    };

    let Some((clicked_entity, entity_position)) =
        get_clicked_entity(&mouse_position, &entity_query)
    else {
        return;
    };

    let mouse_entity = Spawner::new(MouseEntity, &mut commands)
        .with_bundle(Position(mouse_position))
        .id();

    let between = (mouse_position - entity_position).length();

    Spawner::new(MouseEntity, &mut commands)
        .with_bundle(spring_bundle(
            0.1,
            mouse_entity,
            clicked_entity,
            1.0,
            50.0,
            between,
        ))
        .with_shape(
            Shape::Spring(SpringShape {
                coil_count: 20,
                coil_diameter: 0.01,
            }),
            &mut meshes,
        )
        .with_color(Color::srgb_u8(0, 100, 200), &mut materials)
        .with_z_value(-1.0)
        .id();
}

fn move_mouse_spring(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<&Camera>,
    mut mouse_entity_query: Query<&mut Position, (With<MouseEntity>, Without<Spring>)>,
) {
    let Ok(mut mouse_entity_pos) = mouse_entity_query.get_single_mut() else {
        return;
    };

    if let Some(position) = get_mouse_position(&window_query, &camera_query) {
        mouse_entity_pos.0 = position;
    }
}

fn destroy_mouse_spring(
    mouse_entity_query: Query<Entity, With<MouseEntity>>,
    mut commands: Commands,
) {
    for entity in &mouse_entity_query {
        commands.entity(entity).despawn_recursive();
    }
}

fn get_mouse_position(
    window_query: &Query<&Window, With<PrimaryWindow>>,
    camera_query: &Query<&Camera>,
) -> Option<DVec2> {
    let mut position = window_query.single().cursor_position()?;

    let camera = camera_query.single();
    let viewport_size = camera
        .logical_viewport_size()
        .expect("Can't get viewport size?!");
    let scale = viewport_size.min_element();

    // Center position
    position -= viewport_size / 2.0;

    // positive y is up >:(
    position = Vec2::new(position.x, -position.y);

    // Normalize position (Why divide by 4?)
    position /= scale / 4.0;

    Some(position.as_dvec2())
}

fn get_clicked_entity(
    mouse_position: &DVec2,
    entity_query: &Query<(Entity, &Position), With<PhysicsObject>>,
) -> Option<(Entity, DVec2)> {
    let mut min_distance = f64::INFINITY;
    let mut min_entity = None;

    for (entity, position) in entity_query.iter() {
        let distance = (position.0 - mouse_position).length();
        if distance < min_distance {
            min_distance = distance;
            min_entity = Some((entity, position.0));
        }
    }

    min_entity
}
