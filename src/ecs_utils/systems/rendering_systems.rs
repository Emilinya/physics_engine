use std::collections::{hash_map::Entry, HashMap};

use cgmath::InnerSpace;

use crate::ecs_utils::components::{ConnectionComponent, PositionComponent, RotationComponent, ShapeComponent, SizeComponent, TextureComponent};
use crate::ecs_utils::systems::zip_filter_unwrap;
use crate::rendering::instance::{Instance, InstanceModel};
use crate::shapes::shape::ShapeEnum;
use crate::rendering::model::Model;
use crate::ecs_utils::ecs::TextureIndex;

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
    instance_map: &mut HashMap<(ShapeEnum, TextureIndex), (Model, Vec<InstanceModel>)>,
    texture_map: &[Box<[u8]>],
    shape_components: &Vec<Option<ShapeComponent>>,
    position_components: &Vec<Option<PositionComponent>>,
    rotation_components: &Vec<Option<RotationComponent>>,
    texture_components: &Vec<Option<TextureComponent>>,
    size_components: &Vec<Option<SizeComponent>>,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
) {
    let iter = zip_filter_unwrap!(
        shape_components; as_ref; 0,
        position_components; as_ref; 1,
        rotation_components; as_ref; 2,
        texture_components; as_ref; 3,
        size_components; as_ref; 4
    ).map(|(shape, position, rotation, texture, size)| {
        let instance_model = Instance {
            position: position.position,
            rotation: rotation.rotation,
            width: size.width,
            height: size.height,
        }.to_raw();
        (shape, texture, instance_model)
    });

    // clear previous instances
    for (_, instances) in instance_map.values_mut() {
        *instances = Vec::new();
    }

    // add current instances, create new model where neccesary
    for (shape, texture, instance_model) in iter {
        match instance_map.entry((shape.shape, texture.texture)) {
            Entry::Vacant(e) => {
                let model = Model::from_shape(shape.shape, texture_map[texture.texture].as_ref(), device, queue, layout).unwrap();
                e.insert((model, vec![instance_model]));
            }
            Entry::Occupied(mut e) => e.get_mut().1.push(instance_model),
        }
    }
}
