use bevy::prelude::*;
use strum::IntoEnumIterator;

use super::{GameScene, despawn_scene};

use crate::scenes::SceneButton;

#[derive(Component)]
struct SelectEntity;

pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScene::Select), select_setup)
            .add_systems(OnExit(GameScene::Select), despawn_scene::<SelectEntity>);
    }
}

fn select_setup(mut commands: Commands) {
    debug!("Setting up select");

    commands
        .spawn((
            Node {
                padding: UiRect::all(Val::Px(10.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            SelectEntity,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Select Scene"),
                TextFont {
                    font_size: 26.0,
                    ..Default::default()
                },
                TextColor::from(Color::BLACK),
            ));

            for scene in GameScene::iter() {
                if matches!(scene, GameScene::Select) {
                    continue;
                }

                parent
                    .spawn((
                        SceneButton(scene),
                        Button,
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(4.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor(Color::BLACK),
                        BorderRadius::MAX,
                    ))
                    .with_child((
                        Text::new(scene.to_string()),
                        TextFont {
                            font_size: 20.0,
                            ..Default::default()
                        },
                        TextColor::from(Color::BLACK),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
            }
        });
}
