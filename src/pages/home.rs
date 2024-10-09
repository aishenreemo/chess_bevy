use bevy::prelude::*;

use super::GameState;
use crate::ui::button::InteractiveButton;
use crate::ui::palette::ColorPalette;

#[derive(Component, PartialEq, Eq)]
pub enum Button {
    Play,
    Quit,
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

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, palette: Res<ColorPalette>) {
    let root = NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(25.),
            ..default()
        },
        ..default()
    };

    let chess_text = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Chess".to_owned(),
                style: TextStyle {
                    color: palette.white,
                    font: asset_server.load("Kosefont.ttf").into(),
                    font_size: 128.,
                },
            }],
            ..default()
        },
        ..default()
    };

    let play_button = ButtonBundle {
        style: Style {
            width: Val::Px(300.),
            height: Val::Px(80.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        background_color: palette.white.into(),
        border_color: Color::NONE.into(),
        ..default()
    };
    let play_text = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Play".to_owned(),
                style: TextStyle {
                    color: palette.black,
                    font: asset_server.load("Kosefont.ttf").into(),
                    font_size: 64.,
                },
            }],
            ..default()
        },
        ..default()
    };

    let quit_button = ButtonBundle {
        style: Style {
            width: Val::Px(300.),
            height: Val::Px(80.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        background_color: palette.white.into(),
        border_color: Color::NONE.into(),
        ..default()
    };
    let quit_text = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Quit".to_owned(),
                style: TextStyle {
                    color: palette.black,
                    font: asset_server.load("Kosefont.ttf").into(),
                    font_size: 64.,
                },
            }],
            ..default()
        },
        ..default()
    };

    commands.spawn(root).with_children(|p| {
        p.spawn(chess_text);
        p.spawn(play_button)
            .insert(Button::Play)
            .insert(InteractiveButton)
            .with_children(|p| {
                p.spawn(play_text);
            });

        p.spawn(quit_button)
            .insert(Button::Quit)
            .insert(InteractiveButton)
            .with_children(|p| {
                p.spawn(quit_text);
            });
    });
}

