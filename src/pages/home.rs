use bevy::prelude::*;

use super::GameState;
use crate::ui::button::InteractiveButton;
use crate::ui::widgets::*;

#[derive(Component, PartialEq, Eq)]
pub enum Button {
    Play,
    Quit,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    w_root((), &asset_server, &mut commands, (), |p| {
        w_text("Chess", (), (), (), p);
        w_text_button("Play", (), (), (Button::Play, InteractiveButton), p);
        w_text_button("Quit", (), (), (Button::Quit, InteractiveButton), p);
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
