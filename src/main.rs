mod components;
mod debug_menu;
mod mouse;
mod physics;
mod scenes;
mod shapes;
mod spawners;

use debug_menu::DebugPlugin;
use mouse::InteractivityPlugin;
use physics::PhysicsPlugin;
use scenes::{GameScene, ScenePlugin};

use std::ffi::OsString;
use std::fs::File;

use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowPosition, WindowResolution};
use clap::Parser;

#[derive(Resource, Default)]
struct Energy(f64);

#[derive(Resource, Default)]
struct EnergyFile(Option<OsString>);

fn add_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 4.0,
            ..OrthographicProjection::default_2d()
        },
    ));
}

#[derive(Parser, Debug)]
struct Args {
    /// File to save energy data to. If not set, no data is saved
    #[arg(short, long)]
    energy_file: Option<OsString>,
}

fn main() {
    let args = Args::parse();

    if let Some(file) = &args.energy_file {
        if let Err(err) = File::create(file) {
            panic!("Failed to create energy file: {}", err);
        }
    }

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
        .insert_resource(EnergyFile(args.energy_file))
        .init_state::<GameScene>()
        .add_plugins((PhysicsPlugin, DebugPlugin, ScenePlugin, InteractivityPlugin))
        .add_systems(Startup, add_camera)
        .run();
}
