use bevy::diagnostic::{
    Diagnostic, DiagnosticsStore, FrameTimeDiagnosticsPlugin, RegisterDiagnostic,
};
use bevy::prelude::*;

use crate::Energy;

#[derive(Component)]
struct DebugRoot;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct EnergyText;

pub struct DebugInfoPlugin;

impl Plugin for DebugInfoPlugin {
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
            Text::new(text),
            TextFont {
                font_size: 16.0,
                ..Default::default()
            },
            TextColor::from(Color::WHITE),
        ))
        .with_child((
            label,
            TextSpan::new(" N/A"),
            TextFont {
                font_size: 16.0,
                ..Default::default()
            },
            TextColor::from(Color::WHITE),
        ))
        .id()
}

fn setup_fps_counter(mut commands: Commands) {
    let root = commands
        .spawn((
            DebugRoot,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Percent(1.),
                right: Val::Auto,
                bottom: Val::Auto,
                left: Val::Percent(0.),
                padding: UiRect::all(Val::Px(4.0)),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            GlobalZIndex(i32::MAX),
            BackgroundColor(Color::BLACK.with_alpha(0.5)),
        ))
        .id();

    let fps_text = spawn_debug_text(&mut commands, FpsText, "FPS: ");
    let initial_energy_text = spawn_debug_text(&mut commands, EnergyText, "  E: ");

    commands
        .entity(root)
        .add_children(&[fps_text, initial_energy_text]);
}

fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            text.0 = format!("{value:.0}");
        } else {
            text.0 = " N/A".into();
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_energy_text(energy: Res<Energy>, mut query: Query<&mut TextSpan, With<EnergyText>>) {
    for mut text in query.iter_mut() {
        text.0 = format!("{:.3}", energy.0);
    }
}
