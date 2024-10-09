use bevy::prelude::*;

use super::palette::ColorPalette;

pub struct ButtonInteractivePlugin;

impl Plugin for ButtonInteractivePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system);
    }
}

#[derive(Component)]
pub struct InteractiveButton;

fn button_system(
    mut buttons: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<InteractiveButton>),
    >,
    mut texts: Query<&mut Text>,
    palette: Res<ColorPalette>,
) {
    for (interaction, mut bg_color, mut border_color, children) in &mut buttons {
        let mut text = texts.get_mut(children[0]).unwrap();
        match interaction {
            Interaction::Pressed => {
                text.sections[0].style.color = palette.orange.into();
                border_color.0 = palette.orange.into();
            }
            Interaction::Hovered => {
                text.sections[0].style.color = palette.white.into();
                *bg_color = palette.black.into();
                border_color.0 = palette.white.into();
            }
            Interaction::None => {
                text.sections[0].style.color = palette.black.into();
                border_color.0 = Color::NONE.into();
                *bg_color = palette.white.into();
            }
        }
    }
}
