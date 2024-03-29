mod add_utils;
mod components;
mod debug_menu;
mod integrators;
mod physics_system;
mod shapes;

use add_utils::add_entities;
use debug_menu::DebugPlugin;
use physics_system::PhysicsPlugin;

use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowPosition, WindowResolution};

#[derive(Resource)]
struct TotalEnergy {
    initial: Option<f64>,
    current: Option<f64>,
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scale: 4.0,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .insert_resource(TotalEnergy {
            initial: None,
            current: None,
        })
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // mode: WindowMode::Fullscreen,
                resolution: WindowResolution::new(960.0, 540.0),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                title: "Physics engine".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((PhysicsPlugin, DebugPlugin))
        .add_systems(Startup, (add_entities, add_camera))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
