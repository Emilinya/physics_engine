use core::iter::zip;
use std::cmp::Ordering;
use std::collections::{hash_map::Entry, HashMap};

use cgmath::InnerSpace;

use crate::instance::{Instance, InstanceModel};
use crate::rendering::model::Model;
use crate::shapes::{shape::ShapeEnum, spring::Spring, square::Square};

#[derive(Clone)]
pub struct PositionComponent {
    position: cgmath::Vector2<f32>,
}
pub struct RotationComponent {
    rotation: cgmath::Rad<f32>,
}
pub struct SizeComponent {
    width: f32,
    height: f32,
}
pub struct ShapeComponent {
    shape: ShapeEnum,
}

pub struct PlayerComponent {
    speed: f32,
}
#[allow(dead_code)]
pub struct PhysicsComponent {
    velocity: f32,
    force: f32,
    mass: f32,
}
#[allow(dead_code)]
pub struct SpringForceComponent {
    spring_constant: f32,
    equilibrium_length: f32,
}

pub struct ConnectionComponent {
    entity1: EntityIndex,
    entity2: EntityIndex,
}

type EntityIndex = usize;

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
    player_components: PlayerComponent,
    shape_components: ShapeComponent,
    size_components: SizeComponent
);

impl Ecs {
    pub fn convert_to_player(&mut self, entity: EntityIndex, speed: f32) {
        if entity >= self.empty_index {
            panic!("Can't make non-existant entity a player! Got index {:?}, current empty index is {:?}", entity, self.empty_index);
        }
        if self.player_components[entity].is_none() {
            self.player_components[entity] = Some(PlayerComponent { speed });
        }
    }

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
        width: f32,
        height: f32,
    ) -> EntityIndex {
        let position_component = PositionComponent { position };
        let rotation_component = RotationComponent {
            rotation: cgmath::Rad(0.0),
        };
        let shape_component = ShapeComponent {
            shape: ShapeEnum::Square(Square::new()),
        };
        let size_component = SizeComponent { width, height };

        self.position_components.push(Some(position_component));
        self.rotation_components.push(Some(rotation_component));
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

        self.spring_force_components.push(Some(spring_force_component));
        self.connection_components.push(Some(connection_component));
        self.position_components.push(Some(position_component));
        self.rotation_components.push(Some(rotation_component));
        self.shape_components.push(Some(shape_component));
        self.size_components.push(Some(size_component));

        self.entities.push(self.empty_index);
        self.empty_index += 1;

        self.align_components();

        self.empty_index - 1
    }
}

// todo: how do I avoid this function taking idx as an argument?
macro_rules! zip_filter_unwrap {
    ($ray: expr ; $reftype: tt) => {
        zip_filter_unwrap!($ray ; $reftype ; 0)
    };
    ($ray: expr ; $reftype: tt ; $idx: tt) => {
        $ray.iter().filter_map(|v| v.$reftype())
    };
    ($($rays: expr ; $reftypes: tt ; $idxs: tt),+) => {
        itertools::izip!($($rays),+)
            .filter(|v| $(v.$idxs.is_some())&+)
            .map(|v| ($(v.$idxs.$reftypes().unwrap()),+))
    };
}

pub fn player_movement_system(
    player_components: &Vec<Option<PlayerComponent>>,
    position_components: &mut Vec<Option<PositionComponent>>,
    dt: &instant::Duration,
) {
    let iter = zip(player_components, position_components)
        .filter(|(a, b)| a.is_some() & b.is_some())
        .map(|(a, b)| (a.as_ref().unwrap(), b.as_mut().unwrap()));

    for (player, position) in iter {
        position.position += cgmath::Vector2::new(-player.speed * dt.as_secs_f32(), 0.0);
    }
}

pub fn connection_system(
    connection_components: &Vec<Option<ConnectionComponent>>,
    position_components: &mut Vec<Option<PositionComponent>>,
    rotation_components: &mut [Option<RotationComponent>],
    size_components: &mut [Option<SizeComponent>],
) {
    // get position of connections
    let num_components = position_components.len();
    let mut connection_positions: Vec<Option<(PositionComponent, PositionComponent)>> = Vec::with_capacity(num_components);
    for connection_component in connection_components {
        if let Some(connection) = connection_component {
            if (connection.entity1 >= num_components) | (connection.entity2 >= num_components) {
                panic!(
                    "Error when updating connection: entity1 ({:?}) or entity2 ({:?}) does not exist! Number of components is {:?}",
                    connection.entity1, connection.entity2, num_components
                );
            }
            match (&position_components[connection.entity1], &position_components[connection.entity2]) {
                (Some(c1), Some(c2)) => connection_positions.push(Some((c1.clone(), c2.clone()))),
                _ => {
                    panic!(
                        "Error when updating connection: entity1 ({:?}) or entity2 ({:?}) does not have a position!",
                        connection.entity1, connection.entity2
                    );
                }
            };
        } else {
            connection_positions.push(None);
        }
    }

    // create component iterator
    let iterator = zip_filter_unwrap!(
        connection_positions.iter(); as_ref; 0,
        position_components; as_mut; 1,
        rotation_components; as_mut; 2,
        size_components; as_mut; 3
    );

    // update values of connection
    for (connection_positions, position, rotation, size) in iterator {
        let midpoint = (connection_positions.1.position + connection_positions.0.position) / 2.0;
        let between = connection_positions.1.position - connection_positions.0.position;
        let length = between.magnitude();

        position.position = midpoint;
        rotation.rotation = cgmath::Vector2::angle(cgmath::Vector2::unit_x(), between);
        size.width = length;
    }
}

#[allow(clippy::too_many_arguments)]
pub fn instance_system(
    instance_map: &mut HashMap<ShapeEnum, (Model, Vec<InstanceModel>)>,
    shape_components: &Vec<Option<ShapeComponent>>,
    position_components: &Vec<Option<PositionComponent>>,
    rotation_components: &Vec<Option<RotationComponent>>,
    size_components: &Vec<Option<SizeComponent>>,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
    texture_data: &[u8],
) {
    let iter = zip_filter_unwrap!(
        shape_components; as_ref; 0,
        position_components; as_ref; 1,
        rotation_components; as_ref; 2,
        size_components; as_ref; 3
    ).map(|(shape, position, rotation, size)| {
        let instance_model = Instance {
            position: position.position,
            rotation: rotation.rotation,
            width: size.width,
            height: size.height,
        }.to_raw();
        (shape, instance_model)
    });

    // clear previous instances
    for (_, instances) in instance_map.values_mut() {
        *instances = Vec::new();
    }

    // add current instances, create new model where neccesary
    for (shape, instance_model) in iter {
        match instance_map.entry(shape.shape) {
            Entry::Vacant(e) => {
                let model = Model::from_shape(shape.shape, texture_data, device, queue, layout).unwrap();
                e.insert((model, vec![instance_model]));
            }
            Entry::Occupied(mut e) => e.get_mut().1.push(instance_model),
        }
    }
}
