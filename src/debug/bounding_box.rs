use crate::components::{Position, Rotation, Size, Tangible};
use crate::shapes::Shape;
use crate::shapes::ShapeImpl;

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
    camera_query: Query<&Camera>,
    shape_query: Query<(&Shape, &Position, &Size, &Rotation), With<BoundingBox>>,
    mut pointer_query: Query<(Entity, &EntityPointer), Without<BoundingBox>>,
) {
    let camera = camera_query.single();
    let viewport_size = camera
        .logical_viewport_size()
        .expect("Can't get viewport size?!");
    let scale = viewport_size.min_element();

    for (entity, entity_pointer) in pointer_query.iter_mut() {
        let Ok((shape, position, size, rotation)) = shape_query.get(entity_pointer.0) else {
            commands.entity(entity).despawn();
            continue;
        };

        let bounding_box = shape.get_bounding_box(position, size, rotation);
        gizmos.rect_2d(
            bounding_box.center() * scale,
            bounding_box.size() * scale,
            Color::BLACK,
        );
    }
}

fn set_gizmo_config(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 6.0;
}
