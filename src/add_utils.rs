use crate::components::*;
use crate::shapes::shape::Shape;

use bevy::math::DVec2;
use bevy::prelude::*;

fn add_physics_cube(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: DVec2,
    mass: f64,
    width: f64,
    height: f64,
) -> Entity {
    commands
        .spawn((
            Square,
            Position(position),
            Size { width, height },
            PhysicsObject::at_rest(mass),
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(Color::srgb_u8(10, 10, 200))),
        ))
        .id()
}

fn add_spring(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    width: f64,
    entity1: Entity,
    entity2: Entity,
) {
    let spring = crate::shapes::spring::Spring {
        coil_count: 20,
        coil_diameter: 0.01,
    };
    commands.spawn((
        Spring,
        Size {
            width: 0.0,
            height: width,
        },
        SpringForce {
            spring_constant: 20.0,
            equilibrium_length: 1.0,
        },
        Connection { entity1, entity2 },
        Mesh2d(meshes.add(spring.get_mesh())),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));
}

pub fn add_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let fixed_point = commands.spawn(Position(DVec2::new(0.0, 2.0))).id();
    let mut entity1 = fixed_point;

    for i in 0..3 {
        let entity2 = add_physics_cube(
            &mut commands,
            &mut meshes,
            &mut materials,
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
