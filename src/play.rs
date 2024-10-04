use bevy::prelude::*;
use bevy_ui_dsl::*;

use super::ui_classes::*;
use super::GameState;

#[derive(Component)]
pub struct GoBackButton;

pub fn setup_play(mut commands: Commands, asset_server: Res<AssetServer>) {
    root(c_root, &asset_server, &mut commands, |p| {
        text_buttoni("Go back", c_button, (c_text_black, c_text_h2), GoBackButton, p);
    });
}

pub fn play_button_system(
    mut ui_entities: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (With<GoBackButton>, Changed<Interaction>),
    >,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut bg_color, mut border_color, children) in &mut ui_entities {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match interaction {
            Interaction::Pressed => {
                next_state.set(GameState::Home);
            }
            Interaction::Hovered => {
                text.sections[0].style.color = Color::WHITE;
                *bg_color = Color::BLACK.into();
                border_color.0 = Color::WHITE.into();
            }
            _ => {
                text.sections[0].style.color = Color::BLACK;
                border_color.0 = Color::NONE.into();
                *bg_color = Color::WHITE.into();
            }
        }
    }
}
