use bevy::prelude::*;

use super::GameState;
use crate::ui::button::InteractiveButton;
use crate::ui::widgets::*;

#[derive(Component, PartialEq, Eq)]
pub enum Button {
    GoBack,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    w_root((), &asset_server, &mut commands, (), |p| {
        w_text_button("Go back", (), (), (Button::GoBack, InteractiveButton), p);
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
