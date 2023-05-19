use std::rc::Rc;
use std::cell::RefCell;

use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

use crate::entity::Entity;

pub struct Player {
    entity: Rc<RefCell<Entity>>,
    speed: f32,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl Player {
    pub fn new(entity: Rc<RefCell<Entity>>, speed: f32) -> Self {
        Self {
            entity,
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

    pub fn update(&self, world_size: &(f32, f32), dt: &instant::Duration) {
        {
            let mut entity_mutref = self.entity.borrow_mut();
            let dt_seconds = dt.as_secs_f32();

            entity_mutref.rotation += cgmath::Rad(1.0) * dt_seconds;
            if self.is_left_pressed {
                entity_mutref.position.x -= self.speed * dt_seconds;
            }
            if self.is_right_pressed {
                entity_mutref.position.x += self.speed * dt_seconds;
            }
            if self.is_down_pressed {
                entity_mutref.position.y -= self.speed * dt_seconds;
            }
            if self.is_up_pressed {
                entity_mutref.position.y += self.speed * dt_seconds;
            }
        }

        let entity_ref = self.entity.borrow();
        let (top_right, bottom_left) = entity_ref.model.get_shape().get_bounding_box(&entity_ref);
        let (width, height) = (top_right - bottom_left).into();
        let center_offset = entity_ref.position - (top_right + bottom_left) / 2.0;
        drop(entity_ref);

        let mut entity_mutref = self.entity.borrow_mut();
        entity_mutref.position.x = entity_mutref.position.x.clamp(-world_size.0 + center_offset.x + width / 2.0, world_size.0 + center_offset.x - width / 2.0);
        entity_mutref.position.y = entity_mutref.position.y.clamp(-world_size.1 + center_offset.y + height / 2.0, world_size.1 + center_offset.y - height / 2.0);
    }
}
