use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::GameState;
use crate::chess::ChessBoard;
use crate::chess::ChessBoardBundle;
use crate::chess::ChessBoardSize;
use crate::chess::PieceColor;
use crate::ui::button::InteractiveButton;
use crate::ui::palette::ColorPalette;

#[derive(Component, PartialEq, Eq)]
pub enum Button {
    Back,
}

#[derive(Component, Debug)]
pub struct ChessBoardSquare;

#[derive(Component)]
pub struct ChessBoardPiece;

#[derive(Component)]
pub struct MovingPiece;

#[derive(Component)]
pub struct ActivePiece;

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

pub fn pick_system(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    chessboards: Query<&Children, With<ChessBoard>>,
    squares: Query<(&Transform, &Sprite, &Children), With<ChessBoardSquare>>,
    pieces: Query<Entity, With<ChessBoardPiece>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(mut cursor_position) = windows.single().cursor_position() else {
        return;
    };
    cursor_position -= windows.single().resolution.size() / 2.;
    cursor_position.y = -cursor_position.y;

    let Ok(chessboard_squares) = chessboards.get_single() else {
        return;
    };

    for &child in chessboard_squares.iter() {
        let Ok((transform, sprite, children)) = squares.get(child) else {
            continue;
        };

        let position = transform.translation.truncate();
        let size = sprite.custom_size.unwrap();
        let rect = Rect::from_center_size(position, size);

        if !rect.contains(cursor_position) {
            continue;
        }

        let Some(&child) = children.iter().next() else {
            break;
        };

        let Ok(piece) = pieces.get(child) else {
            continue;
        };

        commands.entity(piece).insert(MovingPiece).insert(ActivePiece);

        break;
    }
}

pub fn place_system(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    squares: Query<(Entity, &Transform, &Sprite), With<ChessBoardSquare>>,
    mut active_pieces: Query<
        (Entity, &mut Transform, &Parent),
        (With<ActivePiece>, Without<ChessBoardSquare>),
    >,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(mut cursor_position) = windows.single().cursor_position() else {
        return;
    };
    cursor_position -= windows.single().resolution.size() / 2.;
    cursor_position.y = -cursor_position.y;

    let Ok((entity, mut transform, parent)) = active_pieces.get_single_mut() else {
        println!("Missing");
        return;
    };

    transform.translation = Vec3::new(0., 0., 1.);

    for (square_entity, square_transform, sprite) in squares.iter() {
        let position = square_transform.translation.truncate();
        let size = sprite.custom_size.unwrap();
        let rect = Rect::from_center_size(position, size);

        if !rect.contains(cursor_position) {
            continue;
        }

        let mut piece_entity = commands.entity(entity);

        piece_entity.remove::<ActivePiece>();

        if square_entity != parent.get() {
            piece_entity.set_parent(square_entity);
        }

        break;
    }
}

pub fn drag_system(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    squares: Query<&Transform, With<ChessBoardSquare>>,
    mut dragging_pieces: Query<
        (&mut Transform, &Parent),
        (With<MovingPiece>, Without<ChessBoardSquare>),
    >,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    let Some(mut cursor_position) = windows.single().cursor_position() else {
        return;
    };
    cursor_position -= windows.single().resolution.size() / 2.;
    cursor_position.y = -cursor_position.y;

    let Ok((mut transform, parent)) = dragging_pieces.get_single_mut() else {
        return;
    };

    let Ok(parent_transform) = squares.get(parent.get()) else {
        return;
    };

    let parent_position = parent_transform.translation.truncate();

    transform.translation.x = cursor_position.x - parent_position.x;
    transform.translation.y = cursor_position.y - parent_position.y;
}

pub fn drop_system(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    squares: Query<(Entity, &Transform, &Sprite), With<ChessBoardSquare>>,
    mut dragging_pieces: Query<
        (Entity, &mut Transform, &Parent),
        (With<MovingPiece>, Without<ChessBoardSquare>),
    >,
) {
    if !buttons.just_released(MouseButton::Left) {
        return;
    }

    let Some(mut cursor_position) = windows.single().cursor_position() else {
        return;
    };
    cursor_position -= windows.single().resolution.size() / 2.;
    cursor_position.y = -cursor_position.y;

    let Ok((entity, mut transform, parent)) = dragging_pieces.get_single_mut() else {
        return;
    };

    transform.translation = Vec3::new(0., 0., 1.);

    for (square_entity, square_transform, sprite) in squares.iter() {
        let position = square_transform.translation.truncate();
        let size = sprite.custom_size.unwrap();
        let rect = Rect::from_center_size(position, size);

        if !rect.contains(cursor_position) {
            continue;
        }

        let mut piece_entity = commands.entity(entity);

        piece_entity.remove::<MovingPiece>();

        if square_entity != parent.get() {
            piece_entity.set_parent(square_entity);
            piece_entity.remove::<ActivePiece>();
        }

        break;
    }
}

pub fn setup(
    mut commands: Commands,
    palette: Res<ColorPalette>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_atlas_layout = TextureAtlasLayout::from_grid(UVec2::splat(45), 6, 2, None, None);
    let chessboard = ChessBoardBundle {
        texture: asset_server.load("pieces_spritesheet.png"),
        font: asset_server.load("Kosefont.ttf"),
        layout: texture_atlas_layouts.add(texture_atlas_layout),
        size: ChessBoardSize(400.),
        spatial: SpatialBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        ..default()
    };

    spawn_chessboard(&mut commands, &palette, chessboard);

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
    palette: &ColorPalette,
    bundle: ChessBoardBundle,
) -> Entity {
    let ChessBoardBundle {
        marker,
        position,
        orientation,
        spatial,
        size,
        texture,
        layout,
        font,
    } = bundle;

    let chessboard = commands.spawn(marker).insert(spatial).id();
    let tile_size = size.0 / 8.;

    for row in 0..8 {
        for col in 0..8 {
            let x = (col as f32 - 3.5) * tile_size;
            let y = (row as f32 - 3.5) * tile_size;

            let color = if (row + col) % 2 == 0 {
                palette.blue
            } else {
                palette.white
            };

            let index = if orientation.0 == PieceColor::White {
                ((7 - row) * 8) + col
            } else {
                (row * 8) + (7 - col)
            };

            let sprite = SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(tile_size)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            };

            let board_square = commands
                .spawn(sprite)
                .insert(ChessBoardSquare)
                .set_parent(chessboard)
                .id();

            let board_size = tile_size.clamp(100. / 8., 400. / 8.);
            let sprite_size = (board_size - (100. / 8.)) / (300. / 8.) * 0.7 + 0.4;

            let Some(piece) = position.pieces[index as usize] else {
                continue;
            };

            let sprite = SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_xyz(0., 0., 1.0).with_scale(Vec3::splat(sprite_size)),
                ..default()
            };
            let texture_atlas = TextureAtlas {
                layout: layout.clone(),
                index: piece.into(),
            };

            commands
                .spawn(sprite)
                .insert(texture_atlas)
                .insert(ChessBoardPiece)
                .set_parent(board_square);
        }
    }

    let mut files = ["A", "B", "C", "D", "E", "F", "G", "H"];
    let mut ranks = ["1", "2", "3", "4", "5", "6", "7", "8"];

    if orientation.0 == PieceColor::Black {
        files.reverse();
        ranks.reverse();
    }

    let board_size = size.0.clamp(100., 400.);
    let font_size = (board_size - 100.) / 300. * 14. + 10.;

    for (i, (file, rank)) in files.iter().zip(ranks.iter()).enumerate() {
        let x = (i as f32 - 3.5) * tile_size;

        for y in [tile_size * 4.5, -tile_size * 4.5].into_iter() {
            let text_style = TextStyle {
                font: font.clone(),
                color: palette.cyan,
                font_size,
            };

            let text2d_file = Text2dBundle {
                text: Text::from_section(file.to_owned(), text_style.clone()),
                transform: Transform::from_xyz(x, y, 0.),
                ..default()
            };
            let text2d_rank = Text2dBundle {
                text: Text::from_section(rank.to_owned(), text_style.clone()),
                transform: Transform::from_xyz(y, x, 0.),
                ..default()
            };

            commands.spawn(text2d_file).set_parent(chessboard);
            commands.spawn(text2d_rank).set_parent(chessboard);
        }
    }

    commands
        .entity(chessboard)
        .insert(position)
        .insert(orientation)
        .insert(size)
        .insert(texture)
        .insert(layout)
        .insert(font);

    chessboard
}
