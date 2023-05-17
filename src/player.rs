use std::rc::Rc;
use std::cell::RefCell;
use core::cmp::{min_by, max_by};

use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

use crate::entity::{Entity, self};

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

    fn get_bounding_box(&self) -> (cgmath::Vector2<f32>, cgmath::Vector2<f32>) {
        // why is this neccesary ...
        let max_func = |x: &f32, y: &f32| x.total_cmp(y);
        let min_func = |x: &f32, y: &f32| y.total_cmp(x);

        let transformation_matrix = self.entity.borrow().get_model_matrix();
        let square_vertices = [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
        let entity_vertices: Vec<cgmath::Vector2<f32>> = square_vertices.iter().map(|v| {
            let vec3 = transformation_matrix * cgmath::Vector3::new(v[0], v[1], 1.0);
            cgmath::Vector2::new(vec3.x, vec3.y)
        }).collect();

        let mut top_left = entity_vertices[0];
        let mut bottom_right = entity_vertices[0];
        for v in &entity_vertices[1..] {
            top_left.x = min_by(top_left.x, v.x, min_func);
            top_left.y = max_by(top_left.y, v.y, max_func);
            bottom_right.x = max_by(bottom_right.x, v.x, min_func);
            bottom_right.y = min_by(bottom_right.y, v.y, max_func);
        }

        (top_left, bottom_right)
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

        let (top_left, bottom_right) = self.get_bounding_box();
        let (width, height) = (top_left - bottom_right).into();
        let mut entity_mutref = self.entity.borrow_mut();
        entity_mutref.position.x = entity_mutref.position.x.clamp(-world_size.0 + width / 2.0, world_size.0 - width / 2.0);
        entity_mutref.position.y = entity_mutref.position.y.clamp(-world_size.1 + height / 2.0, world_size.1 - height / 2.0);
    }
}
