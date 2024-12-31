use crate::components::{Position, Rotation, Size, Tangible};
use crate::shapes::{Shape, ShapeImpl};
use crate::{MousePosition, WindowSize};

use bevy::prelude::*;

#[derive(Component)]
struct BoundingBox;

#[derive(Component)]
struct EntityPointer(Entity);

pub struct ShowBoundingBoxPlugin;

impl Plugin for ShowBoundingBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (create_bounding_box, move_mounding_box))
            .add_systems(Startup, set_gizmo_config);
    }
}

#[allow(clippy::type_complexity)]
fn create_bounding_box(
    mut commands: Commands,
    query: Query<
        (Entity, &Position, &Size, &Rotation),
        (With<Tangible>, With<Shape>, Without<BoundingBox>),
    >,
) {
    for (entity, _position, _size, _rotation) in query.iter() {
        commands.entity(entity).try_insert(BoundingBox);
        commands.spawn(EntityPointer(entity));
    }
}

fn move_mounding_box(
    mut gizmos: Gizmos,
    mut commands: Commands,
    window: Res<WindowSize>,
    mouse_position: Res<MousePosition>,
    shape_query: Query<(&Shape, &Position, &Size, &Rotation), With<BoundingBox>>,
    mut pointer_query: Query<(Entity, &EntityPointer), Without<BoundingBox>>,
) {
    for (entity, entity_pointer) in pointer_query.iter_mut() {
        let Ok((shape, position, size, rotation)) = shape_query.get(entity_pointer.0) else {
            commands.entity(entity).despawn();
            continue;
        };

        let color = if shape.collides_with_point(
            (*position, *size, *rotation).into(),
            mouse_position.0.as_dvec2(),
        ) {
            Color::srgb_u8(50, 200, 50)
        } else {
            Color::BLACK
        };

        let bounding_box = shape.get_bounding_box((*position, *size, *rotation).into());
        gizmos.rect_2d(
            bounding_box.center().as_vec2() * window.scale,
            bounding_box.size().as_vec2() * window.scale,
            color,
        );
    }
}

fn set_gizmo_config(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 6.0;
}
