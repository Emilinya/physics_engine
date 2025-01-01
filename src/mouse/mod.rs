use crate::components::*;
use crate::shapes::{Shape, ShapeImpl, SpringShape};
use crate::spawners::{spring::spring_bundle, Spawner};
use crate::MousePosition;

use bevy::input::common_conditions::*;
use bevy::math::DVec2;
use bevy::prelude::*;

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
    mouse_position_resource: Res<MousePosition>,
    entity_query: Query<(Entity, &Shape, &Position, &Size, &Rotation), With<PhysicsObject>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mouse_position = mouse_position_resource.0.as_dvec2();

    let Some((clicked_entity, entity_position)) = get_clicked_entity(mouse_position, &entity_query)
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
    mouse_position: Res<MousePosition>,
    mut mouse_entity_query: Query<&mut Position, (With<MouseEntity>, Without<Spring>)>,
) {
    let Ok(mut mouse_entity_pos) = mouse_entity_query.get_single_mut() else {
        return;
    };

    mouse_entity_pos.0 = mouse_position.0.as_dvec2();
}

fn destroy_mouse_spring(
    mouse_entity_query: Query<Entity, With<MouseEntity>>,
    mut commands: Commands,
) {
    for entity in &mouse_entity_query {
        commands.entity(entity).despawn_recursive();
    }
}

fn get_clicked_entity(
    mouse_position: DVec2,
    entity_query: &Query<(Entity, &Shape, &Position, &Size, &Rotation), With<PhysicsObject>>,
) -> Option<(Entity, DVec2)> {
    for (entity, shape, position, size, rotation) in entity_query.iter() {
        if shape.collides_with_point(&(*position, *size, *rotation).into(), mouse_position) {
            return Some((entity, **position));
        }
    }

    None
}
