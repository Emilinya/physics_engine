use winit::event::*;

use crate::instances::Instance;

pub struct Player {
    instance_idx: usize,
    speed: f32,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl Player {
    pub fn new(instance_idx: usize, speed: f32) -> Self {
        Self {
            instance_idx,
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

    pub fn update(&self, instances: &mut Vec<Instance>, world_size: &(f32, f32)) {
        let instance = &mut instances[self.instance_idx];
        instance.rotation += cgmath::Rad(0.01);
        if self.is_left_pressed {
            instance.position.x -= self.speed;
        }
        if self.is_right_pressed {
            instance.position.x += self.speed;
        }
        if self.is_down_pressed {
            instance.position.y -= self.speed;
        }
        if self.is_up_pressed {
            instance.position.y += self.speed;
        }

        instance.position.x = instance.position.x.clamp(-world_size.0 + 0.5, world_size.0 - 0.5);
        instance.position.y = instance.position.y.clamp(-world_size.1 + 0.5, world_size.1 - 0.5);
    }
}
