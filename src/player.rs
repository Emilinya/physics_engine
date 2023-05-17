use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

use crate::entity::Entity;

pub struct Player {
    entity_idx: usize,
    speed: f32,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl Player {
    pub fn new(entity_idx: usize, speed: f32) -> Self {
        Self {
            entity_idx,
            speed,
            is_up_pressed: false,
            is_down_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update(&self, entities: &mut [Entity], world_size: &(f32, f32)) {
        let entity = &mut entities[self.entity_idx];
        entity.rotation += cgmath::Rad(0.01);
        if self.is_left_pressed {
            entity.position.x -= self.speed;
        }
        if self.is_right_pressed {
            entity.position.x += self.speed;
        }
        if self.is_down_pressed {
            entity.position.y -= self.speed;
        }
        if self.is_up_pressed {
            entity.position.y += self.speed;
        }

        entity.position.x = entity.position.x.clamp(-world_size.0 + entity.width / 2.0, world_size.0 - entity.width / 2.0);
        entity.position.y = entity.position.y.clamp(-world_size.1 + entity.height / 2.0, world_size.1 - entity.height / 2.0);
    }
}
