mod add_utils;
mod components;
mod debug_menu;
mod physics;
mod shapes;

use add_utils::add_entities;
use debug_menu::DebugPlugin;
use physics::PhysicsPlugin;

use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowPosition, WindowResolution};

#[derive(Resource, Default)]
struct TotalEnergy {
    initial: Option<f64>,
    current: Option<f64>,
}

fn add_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 4.0,
            ..OrthographicProjection::default_2d()
        },
    ));
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(TotalEnergy::default())
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // mode: WindowMode::Fullscreen,
                canvas: Some("#gameCanvas".into()),
                resolution: WindowResolution::new(960.0, 540.0),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                title: "Physics engine".to_owned(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((PhysicsPlugin, DebugPlugin))
        .add_systems(Startup, (add_entities, add_camera))
        .run();
}
