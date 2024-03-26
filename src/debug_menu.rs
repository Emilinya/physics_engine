use bevy::diagnostic::{
    Diagnostic, DiagnosticsStore, FrameTimeDiagnosticsPlugin, RegisterDiagnostic,
};
use bevy::prelude as bvy;
use bvy::BuildChildren;

use crate::TotalEnergy;

#[derive(bvy::Component)]
struct DebugRoot;

#[derive(bvy::Component)]
struct FpsText;

#[derive(bvy::Component)]
struct InitialEnergyText;

#[derive(bvy::Component)]
struct CurrentEnergyText;

pub struct DebugPlugin;

impl bvy::Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_diagnostic(
            Diagnostic::new(FrameTimeDiagnosticsPlugin::FPS).with_smoothing_factor(0.2),
        )
        .add_systems(bvy::Startup, setup_fps_counter)
        .add_systems(bvy::Update, FrameTimeDiagnosticsPlugin::diagnostic_system)
        .add_systems(bvy::Update, (update_fps_text, update_energy_text));
    }
}

fn spawn_debug_text<T: bvy::Component>(
    commands: &mut bvy::Commands,
    label: T,
    text: &str,
) -> bvy::Entity {
    commands
        .spawn((
            label,
            bvy::TextBundle {
                text: bvy::Text::from_sections([
                    bvy::TextSection {
                        value: text.into(),
                        style: bvy::TextStyle {
                            font_size: 16.0,
                            color: bvy::Color::WHITE,
                            ..Default::default()
                        },
                    },
                    bvy::TextSection {
                        value: " N/A".into(),
                        style: bvy::TextStyle {
                            font_size: 16.0,
                            color: bvy::Color::WHITE,
                            ..Default::default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id()
}

fn setup_fps_counter(mut commands: bvy::Commands) {
    let root = commands
        .spawn((
            DebugRoot,
            bvy::NodeBundle {
                background_color: bvy::BackgroundColor(bvy::Color::BLACK.with_a(0.5)),
                z_index: bvy::ZIndex::Global(i32::MAX),
                style: bvy::Style {
                    position_type: bvy::PositionType::Absolute,
                    top: bvy::Val::Percent(1.),
                    right: bvy::Val::Auto,
                    bottom: bvy::Val::Auto,
                    left: bvy::Val::Percent(0.),
                    padding: bvy::UiRect::all(bvy::Val::Px(4.0)),
                    flex_direction: bvy::FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    let fps_text = spawn_debug_text(&mut commands, FpsText, " FPS: ");
    let initial_energy_text = spawn_debug_text(&mut commands, InitialEnergyText, "E(0): ");
    let energy_text = spawn_debug_text(&mut commands, CurrentEnergyText, "E(t): ");

    commands
        .entity(root)
        .push_children(&[fps_text, initial_energy_text, energy_text]);
}

fn update_fps_text(
    diagnostics: bvy::Res<DiagnosticsStore>,
    mut query: bvy::Query<&mut bvy::Text, bvy::With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            text.sections[1].value = format!("{value:.0}");
        } else {
            text.sections[1].value = " N/A".into();
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_energy_text(
    energy: bvy::Res<TotalEnergy>,
    mut set: bvy::ParamSet<(
        bvy::Query<&mut bvy::Text, bvy::With<InitialEnergyText>>,
        bvy::Query<&mut bvy::Text, bvy::With<CurrentEnergyText>>,
    )>,
) {
    for mut text in &mut set.p0().iter_mut() {
        if let Some(value) = energy.initial {
            text.sections[1].value = format!("{value:.3}");
        }
    }

    for mut text in &mut set.p1().iter_mut() {
        if let Some(value) = energy.current {
            text.sections[1].value = format!("{value:.3}");
        }
    }
}
