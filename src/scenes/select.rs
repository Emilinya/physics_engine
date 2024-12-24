use bevy::prelude::*;

use super::{despawn_scene, GameScene};

use crate::utils::button::button_is_pressed;

#[derive(Component)]
struct SelectEntity;

pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScene::Select), select_setup)
            .add_systems(Update, button_system.run_if(in_state(GameScene::Select)))
            .add_systems(OnExit(GameScene::Select), despawn_scene::<SelectEntity>);
    }
}

#[derive(Component)]
struct SpringPendulumButton;

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
            parent
                .spawn((
                    SpringPendulumButton,
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
                    Text::new("Spring Pendulum"),
                    TextFont {
                        font_size: 20.0,
                        ..Default::default()
                    },
                    TextColor::from(Color::BLACK),
                    TextLayout::new_with_justify(JustifyText::Center),
                ));
        });
}

#[allow(clippy::type_complexity)]
fn button_system(
    query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<SpringPendulumButton>),
    >,
    mut game_state: ResMut<NextState<GameScene>>,
) {
    if button_is_pressed(query) {
        game_state.set(GameScene::SpringPendulum);
    }
}
