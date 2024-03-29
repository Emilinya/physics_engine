use bevy::diagnostic::{
    Diagnostic, DiagnosticsStore, FrameTimeDiagnosticsPlugin, RegisterDiagnostic,
};
use bevy::prelude::*;

use crate::TotalEnergy;

#[derive(Component)]
struct DebugRoot;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct InitialEnergyText;

#[derive(Component)]
struct CurrentEnergyText;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.register_diagnostic(
            Diagnostic::new(FrameTimeDiagnosticsPlugin::FPS).with_smoothing_factor(0.2),
        )
        .add_systems(Startup, setup_fps_counter)
        .add_systems(Update, FrameTimeDiagnosticsPlugin::diagnostic_system)
        .add_systems(Update, (update_fps_text, update_energy_text));
    }
}

fn spawn_debug_text<T: Component>(commands: &mut Commands, label: T, text: &str) -> Entity {
    commands
        .spawn((
            label,
            TextBundle {
                text: Text::from_sections([
                    TextSection {
                        value: text.into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id()
}

fn setup_fps_counter(mut commands: Commands) {
    let root = commands
        .spawn((
            DebugRoot,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(1.),
                    right: Val::Auto,
                    bottom: Val::Auto,
                    left: Val::Percent(0.),
                    padding: UiRect::all(Val::Px(4.0)),
                    flex_direction: FlexDirection::Column,
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

fn update_fps_text(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
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
    energy: Res<TotalEnergy>,
    mut set: ParamSet<(
        Query<&mut Text, With<InitialEnergyText>>,
        Query<&mut Text, With<CurrentEnergyText>>,
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
