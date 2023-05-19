use std::cell::RefCell;
use std::rc::Rc;

use wgpu::util::DeviceExt;

use crate::model::Model;
use crate::entity::{Entity, EntityModel};

pub struct EntityGroup {
    pub entities: Vec<Rc<RefCell<Entity>>>,
    pub model: Rc<Model>,
}

impl EntityGroup {
    pub fn get_instance_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        let instance_data: Vec<EntityModel> = self.entities.iter().map(|e| e.borrow().to_raw()).collect();
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }
}

