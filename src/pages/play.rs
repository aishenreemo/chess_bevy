use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::Mesh2dHandle;

use super::GameState;
use crate::chess::ChessBoard;
use crate::ui::button::InteractiveButton;
use crate::ui::classes::*;
use crate::ui::widgets::*;

#[derive(Component, PartialEq, Eq)]
pub enum Button {
    GoBack,
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(450., 450.))),
        material: materials.add(Color::WHITE),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    };

    commands.spawn((mesh, ChessBoard));

    w_root(c_root_part, &asset_server, &mut commands, (), |p| {
        w_text_button(
            "Go back",
            c_button_small,
            c_text_normal,
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
