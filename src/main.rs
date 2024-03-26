mod add_utils;
mod components;
mod debug_menu;
mod physics_system;
mod shapes;

use add_utils::add_entities;
use debug_menu::DebugPlugin;
use physics_system::PhysicsPlugin;

use bevy::prelude as bvy;
use bevy::window::{MonitorSelection, WindowPosition, WindowResolution};
use bvy::PluginGroup;

#[derive(bvy::Resource)]
struct TotalEnergy {
    initial: Option<f64>,
    current: Option<f64>,
}

fn add_camera(mut commands: bvy::Commands) {
    commands.spawn(bvy::Camera2dBundle {
        projection: bvy::OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scale: 4.0,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn main() {
    bvy::App::new()
        .insert_resource(bvy::ClearColor(bvy::Color::rgb(1.0, 1.0, 1.0)))
        .insert_resource(TotalEnergy {
            initial: None,
            current: None,
        })
        .insert_resource(bvy::Time::<bvy::Fixed>::from_hz(2400.0))
        .add_plugins(bvy::DefaultPlugins.set(bvy::WindowPlugin {
            primary_window: Some(bvy::Window {
                // mode: WindowMode::Fullscreen,
                resolution: WindowResolution::new(960.0, 540.0),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                title: "Physics engine".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((PhysicsPlugin, DebugPlugin))
        .add_systems(bvy::Startup, (add_entities, add_camera))
        .add_systems(bvy::Update, bevy::window::close_on_esc)
        .run();
}
