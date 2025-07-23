use crate::components::{PhysicsObject, Position, Rotation, Size, Spring, Tangible};
use crate::shapes::{Shape, ShapeImpl, SpringShape};
use crate::spawners::{Spawner, spring::spring_bundle};
use crate::{MousePosition, WindowSize};

use bevy::input::common_conditions::{input_just_pressed, input_just_released, input_pressed};
use bevy::math::DVec2;
use bevy::prelude::*;

pub struct InteractivityPlugin;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct HighlightGizmos;

impl Plugin for InteractivityPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<HighlightGizmos>()
            .add_systems(Startup, set_highlight_gizmo_config)
            .add_systems(
                Update,
                (
                    highlight_hovered_entity,
                    create_mouse_spring.run_if(input_just_pressed(MouseButton::Left)),
                    move_mouse_spring.run_if(input_pressed(MouseButton::Left)),
                    destroy_mouse_spring.run_if(input_just_released(MouseButton::Left)),
                ),
            );
    }
}

#[derive(Component)]
struct MouseEntity;

#[allow(clippy::type_complexity)]
fn highlight_hovered_entity(
    mut gizmos: Gizmos<HighlightGizmos>,
    window: Res<WindowSize>,
    mouse_position_resource: Res<MousePosition>,
    entity_query: Query<
        (&Shape, &Position, &Size, &Rotation),
        (With<Tangible>, With<PhysicsObject>),
    >,
) {
    let mouse_position = mouse_position_resource.0.as_dvec2();

    for (shape, position, size, rotation) in &entity_query {
        let data = (*position, *size, *rotation).into();
        if !shape.collides_with_point(&data, mouse_position) {
            continue;
        }

        let vertices = shape.get_shape_vertices(&data);
        let points = vertices
            .iter()
            .map(|vertex| vertex * window.scale)
            .chain([vertices[0] * window.scale]);
        gizmos.linestrip_2d(points, Color::srgb_u8(50, 200, 50));
    }
}

fn create_mouse_spring(
    mouse_position_resource: Res<MousePosition>,
    entity_query: Query<(Entity, &Shape, &Position, &Size, &Rotation), With<PhysicsObject>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mouse_position = mouse_position_resource.0.as_dvec2();

    let Some((clicked_entity, entity_position)) = get_clicked_entity(mouse_position, entity_query)
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
    if let Ok(mut mouse_entity_pos) = mouse_entity_query.single_mut() {
        mouse_entity_pos.0 = mouse_position.0.as_dvec2();
    }
}

fn destroy_mouse_spring(
    mouse_entity_query: Query<Entity, With<MouseEntity>>,
    mut commands: Commands,
) {
    for entity in &mouse_entity_query {
        commands.entity(entity).despawn();
    }
}

pub fn get_clicked_entity<'a>(
    mouse_position: DVec2,
    entity_query: impl IntoIterator<Item = (Entity, &'a Shape, &'a Position, &'a Size, &'a Rotation)>,
) -> Option<(Entity, DVec2)> {
    for (entity, shape, position, size, rotation) in entity_query {
        if shape.collides_with_point(&(*position, *size, *rotation).into(), mouse_position) {
            return Some((entity, **position));
        }
    }

    None
}

fn set_highlight_gizmo_config(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<HighlightGizmos>();
    config.line.width = 6.0;
    config.line.joints = GizmoLineJoint::Miter;
}
