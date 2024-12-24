mod select;
mod spring_pendulum;

use crate::utils::button::button_is_pressed;
use select::SelectPlugin;
use spring_pendulum::SpringPendulumPlugin;

use bevy::prelude::*;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameScene {
    #[default]
    Select,
    SpringPendulum,
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((back_plugin, SelectPlugin, SpringPendulumPlugin));
    }
}

pub fn despawn_scene<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn back_plugin(app: &mut App) {
    app.add_systems(Startup, add_back_button)
        .add_systems(Update, back_button_system);
}

#[derive(Component)]
struct BackButton;

fn add_back_button(mut commands: Commands) {
    debug!("Setting up select");

    commands
        .spawn((Node {
            padding: UiRect::all(Val::Px(10.0)),
            align_items: AlignItems::End,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn((
                    BackButton,
                    Button,
                    Node {
                        width: Val::Auto,
                        height: Val::Auto,
                        border: UiRect::all(Val::Px(3.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                ))
                .with_child((
                    Text::new("Scene Select"),
                    TextFont {
                        font_size: 18.0,
                        ..Default::default()
                    },
                    TextColor::from(Color::BLACK),
                    TextLayout::new_with_justify(JustifyText::Center),
                ));
        });
}

#[allow(clippy::type_complexity)]
fn back_button_system(
    query: Query<(&Interaction, &mut BorderColor), (Changed<Interaction>, With<BackButton>)>,
    mut game_state: ResMut<NextState<GameScene>>,
) {
    if button_is_pressed(query) {
        game_state.set(GameScene::Select);
    }
}
