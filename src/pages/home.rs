use bevy::prelude::*;
use bevy_ui_dsl::*;

use super::GameState;
use crate::ui::button::InteractiveButton;
use crate::ui::classes::*;

#[derive(Component, PartialEq, Eq)]
pub enum Button {
    Play,
    Quit,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    root(c_root, &asset_server, &mut commands, |p| {
        text("Chess", c_text, (c_text_white, c_text_kose, c_text_h1), p);
        text_buttoni(
            "Play",
            c_button,
            (c_text_black, c_text_kose, c_text_h2),
            (Button::Play, InteractiveButton),
            p,
        );
        text_buttoni(
            "Quit",
            c_button,
            (c_text_black, c_text_kose, c_text_h2),
            (Button::Quit, InteractiveButton),
            p,
        );
    });
}

pub fn button_system(
    mut buttons: Query<(&Button, &Interaction), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (id, interaction) in &mut buttons {
        match interaction {
            Interaction::Pressed if id == &Button::Quit => {
                exit.send(AppExit::Success);
            }
            Interaction::Pressed if id == &Button::Play => {
                next_state.set(GameState::Play);
            }
            _ => {}
        }
    }
}
