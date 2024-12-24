use bevy::prelude::*;

#[allow(clippy::type_complexity)]
pub fn button_is_pressed<C: Component>(
    mut query: Query<(&Interaction, &mut BorderColor), (Changed<Interaction>, With<C>)>,
) -> bool {
    let Ok((interaction, mut color)) = query.get_single_mut() else {
        return false;
    };

    match interaction {
        Interaction::Pressed => {
            *color = Color::srgb_u8(100, 100, 200).into();
        }
        Interaction::Hovered => {
            *color = Color::srgb_u8(150, 150, 150).into();
        }
        Interaction::None => {
            *color = Color::BLACK.into();
        }
    }

    matches!(interaction, Interaction::Pressed)
}
