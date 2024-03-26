use crate::components::*;
use crate::shapes::shape::Shape;

use bevy::math::{DVec2, Vec2};
use bevy::prelude as bvy;

fn add_physics_cube(
    commands: &mut bvy::Commands,
    asset_server: &bvy::Res<bvy::AssetServer>,
    position: DVec2,
    mass: f64,
    width: f64,
    height: f64,
) -> bvy::Entity {
    commands
        .spawn((
            Square,
            Position(position),
            Rotation(0.0),
            Size { width, height },
            PhysicsObject::at_rest(mass),
            bevy::sprite::SpriteBundle {
                texture: asset_server.load("happy-tree.png"),
                sprite: bvy::Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id()
}

fn add_spring(
    commands: &mut bvy::Commands,
    meshes: &mut bvy::ResMut<bvy::Assets<bvy::Mesh>>,
    materials: &mut bvy::ResMut<bvy::Assets<bvy::ColorMaterial>>,
    width: f64,
    entity1: bvy::Entity,
    entity2: bvy::Entity,
) {
    let spring = crate::shapes::spring::Spring {
        coil_count: 20,
        coil_diameter: 0.01,
    };
    commands.spawn((
        Spring,
        Position(DVec2::ZERO),
        Rotation(0.0),
        Size {
            width: 0.0,
            height: width,
        },
        SpringForce {
            spring_constant: 20000.0,
            equilibrium_length: 1.0,
        },
        Connection { entity1, entity2 },
        bevy::sprite::MaterialMesh2dBundle {
            mesh: meshes.add(spring.get_mesh()).into(),
            material: materials.add(bvy::Color::BLACK),
            transform: bvy::Transform::from_xyz(0.0, 0.0, -1.0),
            ..Default::default()
        },
    ));
}

pub fn add_entities(
    mut commands: bvy::Commands,
    mut meshes: bvy::ResMut<bvy::Assets<bvy::Mesh>>,
    mut materials: bvy::ResMut<bvy::Assets<bvy::ColorMaterial>>,
    asset_server: bvy::Res<bvy::AssetServer>,
) {
    let fixed_point = commands.spawn(Position(DVec2::new(0.0, 2.0))).id();
    let mut entity1 = fixed_point;

    for i in 0..4 {
        let entity2 = add_physics_cube(
            &mut commands,
            &asset_server,
            DVec2::new(i as f64 + 1.0, 2.0),
            0.1,
            0.5,
            0.5,
        );
        add_spring(
            &mut commands,
            &mut meshes,
            &mut materials,
            0.1,
            entity1,
            entity2,
        );
        entity1 = entity2;
    }
}
