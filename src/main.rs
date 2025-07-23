mod components;
mod debug;
mod mouse;
mod physics;
mod scenes;
mod shapes;
mod spawners;
mod utils;

use debug::bounding_box::ShowBoundingBoxPlugin;
use debug::menu::DebugInfoPlugin;
use mouse::InteractivityPlugin;
use physics::PhysicsPlugin;
use scenes::{GameScene, ScenePlugin};

use std::ffi::OsString;
use std::fs::File;

use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::{MonitorSelection, PrimaryWindow, WindowPosition, WindowResolution};
use clap::Parser;

#[derive(Resource, Default)]
struct Energy(f64);

#[derive(Resource, Default)]
struct EnergyFile(Option<OsString>);

#[derive(Resource, Default)]
struct WindowSize {
    size: Vec2,
    scale: f32,
}

#[derive(Resource, Default)]
struct MousePosition(Vec2);

fn add_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 4.0,
            ..OrthographicProjection::default_2d()
        },
    ));
}

fn update_window_size(camera_query: Query<&Camera>, mut window: ResMut<WindowSize>) {
    let camera = camera_query.single();
    let viewport_size = camera
        .logical_viewport_size()
        .expect("Should be able to get viewport size");
    let scale = viewport_size.min_element();

    window.scale = scale;
    window.size = viewport_size;
}

fn update_mouse_position(
    window_query: Query<&Window, With<PrimaryWindow>>,
    window: Res<WindowSize>,
    mut mouse_position: ResMut<MousePosition>,
) {
    // Should I set mouse position to None here?
    let Some(mut position) = window_query.single().cursor_position() else {
        return;
    };

    // Center position
    position -= window.size / 2.0;
    // positive y is up >:(
    position = Vec2::new(position.x, -position.y);
    // Normalize position (Why divide by 4?)
    position /= window.scale / 4.0;

    mouse_position.0 = position;
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
            panic!("Failed to create energy file: {err}");
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
        .insert_resource(WindowSize::default())
        .insert_resource(MousePosition::default())
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(EnergyFile(args.energy_file))
        .init_state::<GameScene>()
        .add_plugins((
            PhysicsPlugin,
            DebugInfoPlugin,
            ScenePlugin,
            InteractivityPlugin,
            ShowBoundingBoxPlugin,
        ))
        .add_systems(Startup, add_camera)
        .add_systems(PreUpdate, (update_window_size, update_mouse_position))
        .run();
}
