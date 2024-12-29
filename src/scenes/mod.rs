mod bouncy_castle;
mod select;
mod shapes;
mod spring_pendulum;

use bouncy_castle::BouncyCastlePlugin;
use select::SelectPlugin;
use shapes::ShapesPlugin;
use spring_pendulum::SpringPendulumPlugin;

use std::fmt;

use bevy::prelude::*;
use strum::{EnumIter, IntoStaticStr};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, EnumIter, IntoStaticStr)]
pub enum GameScene {
    #[default]
    Select,
    SpringPendulum,
    BouncyCastle,
    Shapes,
}

impl fmt::Display for GameScene {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Select => f.write_str("Select"),
            Self::SpringPendulum => f.write_str("Spring Pendulum"),
            Self::BouncyCastle => f.write_str("Bouncy Castle"),
            Self::Shapes => f.write_str("Shapes"),
        }
    }
}

#[derive(Component)]
struct SceneButton(GameScene);

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            back_plugin,
            SelectPlugin,
            SpringPendulumPlugin,
            BouncyCastlePlugin,
            ShapesPlugin,
        ));
    }
}

pub fn despawn_scene<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn back_plugin(app: &mut App) {
    app.add_systems(Startup, add_back_button)
        .add_systems(Update, scene_button_system);
}

fn add_back_button(mut commands: Commands) {
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
                    SceneButton(GameScene::Select),
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

fn scene_button_system(
    mut query: Query<(&Interaction, &SceneButton, &mut BorderColor), Changed<Interaction>>,
    mut game_scene: ResMut<NextState<GameScene>>,
) {
    for (interaction, scene, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                *color = Color::srgb_u8(100, 100, 200).into();
                game_scene.set(scene.0);
            }
            Interaction::Hovered => {
                *color = Color::srgb_u8(150, 150, 150).into();
            }
            Interaction::None => {
                *color = Color::BLACK.into();
            }
        }
    }
}
