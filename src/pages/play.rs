use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::GameState;
use crate::chess::ChessBoard;
use crate::chess::ChessPosition;
use crate::chess::PieceColor;
use crate::ui::button::InteractiveButton;
use crate::ui::palette::ColorPalette;

#[derive(Component, PartialEq, Eq)]
pub enum Button {
    Back,
}

#[derive(Component, Default)]
pub struct Draggable {
    dragging: bool,
}

#[derive(Component)]
pub struct Target;

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

pub fn drag_system(
    mut commands: Commands,
    mut pieces: Query<(Entity, &Parent, &mut Transform, &mut Draggable), Without<Target>>,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    targets: Query<(Entity, &Transform, &Sprite), With<Target>>,
) {
    let Some(mut cursor_position) = windows.single().cursor_position() else {
        return;
    };

    cursor_position -= windows.single().resolution.size() / 2.;
    cursor_position.y = -cursor_position.y;

    if buttons.just_pressed(MouseButton::Left) {
        for (_, parent, _, mut draggable) in pieces.iter_mut() {
            let Ok((_, transform, sprite)) = targets.get(parent.get()) else {
                continue;
            };

            let position = transform.translation.truncate();
            let size = sprite.custom_size.unwrap();
            let rect = Rect::from_center_size(position, size);

            if rect.contains(cursor_position) {
                draggable.dragging = true;
                break;
            }
        }
    } else if buttons.pressed(MouseButton::Left) {
        for (_, parent, mut transform, draggable) in pieces.iter_mut() {
            let Ok((_, parent_transform, _)) = targets.get(parent.get()) else {
                continue;
            };

            let parent_position = parent_transform.translation.truncate();

            if draggable.dragging {
                transform.translation = Vec3::new(
                    cursor_position.x - parent_position.x,
                    cursor_position.y - parent_position.y,
                    transform.translation.z,
                );
            }
        }
    } else if buttons.just_released(MouseButton::Left) {
        'outer: for (entity, _, mut transform, mut draggable) in pieces.iter_mut() {
            if !draggable.dragging {
                continue;
            }

            for (parent, target_transform, sprite) in targets.iter() {
                let position = target_transform.translation.truncate();
                let size = sprite.custom_size.unwrap();
                let rect = Rect::from_center_size(position, size);

                if !rect.contains(cursor_position) {
                    continue;
                }

                commands.entity(entity).set_parent(parent);
                draggable.dragging = false;
                transform.translation = Vec3::new(0., 0., 1.);
                break 'outer;
            }
        }
    }
}

pub fn setup(
    mut commands: Commands,
    palette: Res<ColorPalette>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    spawn_chessboard(
        &mut commands,
        &palette,
        &asset_server,
        &mut texture_atlas_layouts,
        ChessBoard {
            position: Vec2::new(0., 0.),
            orientation: PieceColor::Black,
            size: 400.,
        },
        ChessPosition::from_fen(ChessPosition::DEFAULT_WHITE_FEN),
    );

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

fn spawn_chessboard(
    commands: &mut Commands,
    palette: &Res<ColorPalette>,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    entity: ChessBoard,
    position: ChessPosition,
) -> Entity {
    let pieces_texture: Handle<Image> = asset_server.load("pieces_spritesheet.png");
    let pieces_atlas_layout = TextureAtlasLayout::from_grid(UVec2::splat(45), 6, 2, None, None);
    let pieces_layout = texture_atlas_layouts.add(pieces_atlas_layout);

    let tile_size = entity.size / 8.;

    let chessboard = commands
        .spawn(entity.clone())
        .insert(SpatialBundle {
            transform: Transform::from_xyz(entity.position.x, entity.position.y, 0.),
            ..default()
        })
        .id();

    for row in 0..8 {
        for col in 0..8 {
            let x = (col as f32 - 3.5) * tile_size;
            let y = (row as f32 - 3.5) * tile_size;

            let color = if (row + col) % 2 == 0 {
                palette.blue
            } else {
                palette.white
            };

            let board_square = commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(tile_size, tile_size)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..Default::default()
                })
                .insert(Target)
                .set_parent(chessboard)
                .id();

            let board_size = tile_size.clamp(100. / 8., 400. / 8.);
            let sprite_size = (board_size - (100. / 8.)) / (300. / 8.) * 0.7 + 0.4;

            let index = if entity.orientation == PieceColor::White {
                ((7 - row) * 8) + col
            } else {
                (row * 8) + (7 - col)
            };

            if let Some(piece) = position.pieces[index] {
                commands
                    .spawn(SpriteBundle {
                        texture: pieces_texture.clone(),
                        transform: Transform::from_xyz(0., 0., 1.0)
                            .with_scale(Vec3::splat(sprite_size)),
                        ..default()
                    })
                    .insert(TextureAtlas {
                        layout: pieces_layout.clone(),
                        index: piece.into(),
                    })
                    .insert(Draggable::default())
                    .set_parent(board_square);
            }
        }
    }

    let mut files = ["A", "B", "C", "D", "E", "F", "G", "H"];
    let mut ranks = ["1", "2", "3", "4", "5", "6", "7", "8"];

    if entity.orientation == PieceColor::Black {
        files.reverse();
        ranks.reverse();
    }

    let board_size = entity.size.clamp(100., 400.);
    let font_size = (board_size - 100.) / 300. * 14. + 10.;

    for (i, (file, rank)) in files.iter().zip(ranks.iter()).enumerate() {
        let x = (i as f32 - 3.5) * tile_size;

        for y in [tile_size * 4.5, -tile_size * 4.5].into_iter() {
            commands
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        file.to_owned(),
                        TextStyle {
                            font: asset_server.load("Kosefont.ttf"),
                            color: palette.cyan,
                            font_size,
                        },
                    ),
                    transform: Transform::from_xyz(x, y, 0.),
                    ..default()
                })
                .set_parent(chessboard);

            commands
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        rank.to_owned(),
                        TextStyle {
                            font: asset_server.load("Kosefont.ttf"),
                            color: palette.cyan,
                            font_size,
                        },
                    ),
                    transform: Transform::from_xyz(y, x, 0.),
                    ..default()
                })
                .set_parent(chessboard);
        }
    }

    chessboard
}
