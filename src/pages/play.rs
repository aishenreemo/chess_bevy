use bevy::prelude::*;

use super::GameState;
use crate::chess::ChessBoardAssetBundle;
use crate::chess::ChessBoardBundle;
use crate::chess::ChessBoardLabel;
use crate::chess::ChessBoardOffset;
use crate::chess::ChessBoardOrientation;
use crate::chess::ChessBoardPosition;
use crate::chess::ChessBoardSize;
use crate::chess::PieceColor;
use crate::ui::button::InteractiveButton;
use crate::ui::palette::ColorPalette;

#[derive(Component, PartialEq, Eq)]
pub enum Button {
    Back,
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

pub fn setup(
    mut commands: Commands,
    palette: Res<ColorPalette>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands
        .spawn(ChessBoardBundle {
            size: ChessBoardSize(400.),
            offset: ChessBoardOffset((0., 0.).into()),
            orientation: ChessBoardOrientation(PieceColor::White),
            position: ChessBoardPosition::from_fen(ChessBoardPosition::DEFAULT_WHITE_FEN),
            ..default()
        })
        .insert(ChessBoardLabel)
        .insert(ChessBoardAssetBundle::new(
            &asset_server,
            &mut texture_atlas_layouts,
        ));

    // UI
    commands
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(10.),
                width: Val::Percent(100.),
                padding: UiRect::all(Val::Px(10.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|p| {
            p.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(100.),
                    height: Val::Px(40.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: palette.white.into(),
                border_color: palette.blue.into(),
                ..default()
            })
            .with_children(|p| {
                p.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "Back".to_owned(),
                            style: TextStyle {
                                color: palette.black,
                                font: asset_server.load("Kosefont.ttf").into(),
                                font_size: 18.,
                            },
                        }],
                        ..default()
                    },
                    ..default()
                });
            })
            .insert(Button::Back)
            .insert(InteractiveButton);
        });
}
