use crate::WindowSize;
use crate::components::{Position, Rotation, Size, Tangible};
use crate::shapes::{Shape, ShapeImpl};

use bevy::prelude::*;

#[derive(Component)]
struct BoundingBox;

#[derive(Component)]
pub struct BoundingBoxColor(pub Color);

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
    for (entity, _position, _size, _rotation) in &query {
        commands
            .entity(entity)
            .try_insert((BoundingBox, BoundingBoxColor(Color::BLACK)));
        commands.spawn(EntityPointer(entity));
    }
}

fn move_mounding_box(
    mut gizmos: Gizmos,
    mut commands: Commands,
    window: Res<WindowSize>,
    shape_query: Query<(&Shape, &Position, &Size, &Rotation, &BoundingBoxColor), With<BoundingBox>>,
    mut pointer_query: Query<(Entity, &EntityPointer), Without<BoundingBox>>,
) {
    for (entity, entity_pointer) in &mut pointer_query {
        let Ok((shape, position, size, rotation, color)) = shape_query.get(entity_pointer.0) else {
            commands.entity(entity).despawn();
            continue;
        };
        let data = (*position, *size, *rotation).into();

        let bounding_box = shape.get_bounding_box(&data);
        gizmos.rect_2d(
            bounding_box.center().as_vec2() * window.scale,
            bounding_box.size().as_vec2() * window.scale,
            color.0,
        );
    }
}

fn set_gizmo_config(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 6.0;
    config.line.joints = GizmoLineJoint::Miter;
}
