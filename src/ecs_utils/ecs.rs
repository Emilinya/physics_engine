use std::cmp::Ordering;

use cgmath::InnerSpace;

crate::ecs_utils::components::use_components!();
use crate::shapes::{shape::ShapeEnum, square::Square, spring::Spring};

pub type TextureIndex = usize;
pub type EntityIndex = usize;

// create ecs using macro to avoid repetitive code
macro_rules! create_ecs {
    ($($comp: ident: $comp_type: ty),*) => {
        pub struct Ecs {
            empty_index: EntityIndex,

            $(pub $comp: Vec<Option<$comp_type>>,)*

            pub entities: Vec<EntityIndex>,
        }

        impl Ecs {
            pub fn new() -> Self {
                Self {
                    empty_index: 0,
                    $($comp: Vec::new(),)*
                    entities: Vec::new(),
                }
            }

            #[allow(dead_code)]
            pub fn is_aligned(&self) -> bool {
                let mut is_aligned = true;
                $(is_aligned &= (self.$comp.len() == self.empty_index);)*
                is_aligned
            }

            fn align_component<T>(empty_index: &EntityIndex, components: &mut Vec<Option<T>>) {
                let num_components = components.len();
                match num_components.cmp(empty_index) {
                    Ordering::Greater => panic!("Empty index ({empty_index}) is smaller than component container ({num_components}), something has gone wrong!"),
                    Ordering::Less => {
                        let missing_count = empty_index - num_components;
                        if missing_count > 1 {
                            println!("Potential error: component vector is missing more than one component");
                        }
                        components.append(&mut (0..missing_count).map(|_| None).collect());
                    },
                    _ => {}
                }
            }

            fn align_components(&mut self) {
                if self.entities.len() != self.empty_index {
                    panic!("Empty index is smaller than entity container, something has gone wrong!");
                }
                $(Ecs::align_component(&self.empty_index, &mut self.$comp);)*
            }
        }
    };

}

create_ecs!(
    spring_force_components: SpringForceComponent,
    connection_components: ConnectionComponent,
    position_components: PositionComponent,
    rotation_components: RotationComponent,
    physics_components: PhysicsComponent,
    texture_components: TextureComponent,
    shape_components: ShapeComponent,
    size_components: SizeComponent
);

impl Ecs {
    pub fn add_fixed_point(&mut self, position: cgmath::Vector2<f32>) -> EntityIndex {
        self.position_components.push(Some(PositionComponent { position }));

        self.entities.push(self.empty_index);
        self.empty_index += 1;

        self.align_components();

        self.empty_index - 1
    }

    pub fn add_cube(
        &mut self,
        position: cgmath::Vector2<f32>,
        velocity: cgmath::Vector2<f32>,
        mass: f32,
        width: f32,
        height: f32,
        texture: TextureIndex,
    ) -> EntityIndex {
        let position_component = PositionComponent { position };
        let rotation_component = RotationComponent {
            rotation: cgmath::Rad(0.0),
        };
        let physics_component = PhysicsComponent {
            velocity,
            acceleration: cgmath::Vector2::new(0.0, 0.0),
            mass
        };
        let shape_component = ShapeComponent {
            shape: ShapeEnum::Square(Square::new()),
        };
        let size_component = SizeComponent { width, height };
        let texture_component = TextureComponent { texture };

        self.position_components.push(Some(position_component));
        self.rotation_components.push(Some(rotation_component));
        self.physics_components.push(Some(physics_component));
        self.texture_components.push(Some(texture_component));
        self.shape_components.push(Some(shape_component));
        self.size_components.push(Some(size_component));

        self.entities.push(self.empty_index);
        self.empty_index += 1;

        self.align_components();

        self.empty_index - 1
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_spring(
        &mut self,
        coil_count: u32,
        coil_diameter: f32,
        spring_diameter: f32,
        equilibrium_length: f32,
        spring_constant: f32,
        entity1: EntityIndex,
        entity2: EntityIndex,
        texture: TextureIndex,
    ) -> EntityIndex {
        if (entity1 >= self.empty_index) | (entity2 >= self.empty_index) {
            panic!("Error when creating spring: entity1 ({:?}) or entity2 ({:?}) does not exist! Current empty index is {:?}", entity1, entity2, self.empty_index);
        }
        let (entity1_position, entity2_position) = match (&self.position_components[entity1], &self.position_components[entity2]) {
            (Some(c1), Some(c2)) => (c1.position, c2.position),
            _ => panic!("Error when creating spring: entity1 ({:?}) or entity2 ({:?}) does not have a position!", entity1, entity2),
        };
        let midpoint = (entity1_position + entity2_position) / 2.0;
        let length = (entity1_position - entity2_position).magnitude();

        let position_component = PositionComponent { position: midpoint };
        let rotation_component = RotationComponent {
            rotation: entity1_position.angle(entity2_position),
        };
        let size_component = SizeComponent {
            width: length,
            height: spring_diameter,
        };
        let shape_component = ShapeComponent {
            shape: ShapeEnum::Spring(Spring::new(coil_count, coil_diameter)),
        };
        let spring_force_component = SpringForceComponent {
            spring_constant,
            equilibrium_length,
        };
        let connection_component = ConnectionComponent { entity1, entity2 };
        let texture_component = TextureComponent { texture };

        self.spring_force_components.push(Some(spring_force_component));
        self.connection_components.push(Some(connection_component));
        self.position_components.push(Some(position_component));
        self.rotation_components.push(Some(rotation_component));
        self.texture_components.push(Some(texture_component));
        self.shape_components.push(Some(shape_component));
        self.size_components.push(Some(size_component));

        self.entities.push(self.empty_index);
        self.empty_index += 1;

        self.align_components();

        self.empty_index - 1
    }
}
