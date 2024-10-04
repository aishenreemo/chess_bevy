use bevy::prelude::*;
use bevy_ui_dsl::*;

use super::GameState;
use crate::ui::button::InteractiveButton;
use crate::ui::classes::*;

#[derive(Component, PartialEq, Eq)]
pub enum Button {
    GoBack,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    root(c_root, &asset_server, &mut commands, |p| {
        text_buttoni(
            "Go back",
            c_button,
            (c_text_black, c_text_h2),
            (Button::GoBack, InteractiveButton),
            p,
        );
    });
}

pub fn button_system(
    mut buttons: Query<&Interaction, (With<Button>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut buttons {
        match interaction {
            Interaction::Pressed => {
                next_state.set(GameState::Home);
            }
            _ => {}
        }
    }
}
