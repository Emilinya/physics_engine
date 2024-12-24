mod components;
mod debug_menu;
mod physics;
mod scenes;
mod shapes;
mod utils;

use debug_menu::DebugPlugin;
use physics::PhysicsPlugin;
use scenes::{GameScene, ScenePlugin};

use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowPosition, WindowResolution};

#[derive(Resource, Default)]
struct Energy(f64);

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
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // mode: WindowMode::Fullscreen,
                        canvas: Some("#gameCanvas".into()),
                        resolution: WindowResolution::new(960.0, 540.0),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        title: "Physics engine".to_owned(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(LogPlugin {
                    filter: "physics_engine=debug".into(),
                    level: Level::WARN,
                    custom_layer: |_| None,
                }),
        )
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(Energy::default())
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .init_state::<GameScene>()
        .add_plugins((PhysicsPlugin, DebugPlugin, ScenePlugin))
        .add_systems(Startup, add_camera)
        .run();
}
