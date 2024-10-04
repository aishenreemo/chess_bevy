use bevy::prelude::*;

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
) {
    for (interaction, mut bg_color, mut border_color, children) in &mut buttons {
        let mut text = texts.get_mut(children[0]).unwrap();
        match interaction {
            Interaction::Pressed => {
                text.sections[0].style.color = Color::srgb(0., 9., 0.);
                border_color.0 = Color::srgb(0., 9., 0.);
            }
            Interaction::Hovered => {
                text.sections[0].style.color = Color::WHITE;
                *bg_color = Color::BLACK.into();
                border_color.0 = Color::WHITE.into();
            }
            Interaction::None => {
                text.sections[0].style.color = Color::BLACK;
                border_color.0 = Color::NONE.into();
                *bg_color = Color::WHITE.into();
            }
        }
    }
}
