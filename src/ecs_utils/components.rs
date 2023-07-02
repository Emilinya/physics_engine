use crate::shapes::shape::ShapeEnum;
use crate::ecs_utils::ecs::{TextureIndex, EntityIndex};

macro_rules! use_components {
    () => {
        use $crate::ecs_utils::components::{PositionComponent, RotationComponent, SizeComponent, ShapeComponent, PhysicsComponent, SpringForceComponent, ConnectionComponent, TextureComponent};
    }
}
pub(crate) use use_components;

#[derive(Debug, Clone)]
pub struct PositionComponent {
    pub position: cgmath::Vector2<f32>,
}

#[derive(Debug, Clone)]
pub struct RotationComponent {
    pub rotation: cgmath::Rad<f32>,
}

#[derive(Debug, Clone)]
pub struct SizeComponent {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub struct ShapeComponent {
    pub shape: ShapeEnum,
}

#[derive(Debug, Clone)]
pub struct PhysicsComponent {
    pub velocity: cgmath::Vector2<f32>,
    pub acceleration: cgmath::Vector2<f32>,
    pub mass: f32,
}

#[derive(Debug, Clone)]
pub struct SpringForceComponent {
    pub spring_constant: f32,
    pub equilibrium_length: f32,
}

#[derive(Debug, Clone)]
pub struct ConnectionComponent {
    pub entity1: EntityIndex,
    pub entity2: EntityIndex,
}

#[derive(Debug, Clone)]
pub struct TextureComponent {
    pub texture: TextureIndex,
}
