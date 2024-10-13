use bevy::prelude::*;
use cursor::CursorPosition;

use super::*;
use crate::ui::palette::ColorPalette;

pub fn spawn_board_system(
    mut commands: Commands,
    palette: Res<ColorPalette>,
    query: Query<
        (
            Entity,
            &ChessBoardSize,
            &ChessBoardOffset,
            &ChessBoardOrientation,
            &ChessBoardPosition,
        ),
        Added<ChessBoard>,
    >,
) {
    for (entity, size, offset, orientation, position) in query.iter() {
        let tile_size = size.0 / 8.;

        let spatial = SpatialBundle {
            transform: Transform::from_xyz(offset.0.x, offset.0.y, 0.),
            ..default()
        };

        commands.entity(entity).insert(spatial);

        for i in 0..64 {
            let col = i % 8;
            let row = i / 8;

            let x = (col as f32 - 3.5) * tile_size;
            let y = (row as f32 - 3.5) * tile_size;

            let square_index = if orientation.0 == PieceColor::White {
                ((7 - row) * 8) + col
            } else {
                (row * 8) + (7 - col)
            };

            let color = if (row + col) % 2 == 0 {
                palette.blue
            } else {
                palette.white
            };

            let piece = position.pieces[square_index as usize];
            let square = ChessBoardSquare {
                index: square_index,
                piece,
            };

            let sprite = SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::splat(tile_size).into(),
                    color,
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.),
                ..default()
            };

            commands.spawn(square).insert(sprite).set_parent(entity);
        }
    }
}

pub fn spawn_labels_system(
    mut commands: Commands,
    palette: Res<ColorPalette>,
    asset_server: Res<AssetServer>,
    query: Query<
        (Entity, &ChessBoardSize, &ChessBoardOrientation),
        (With<ChessBoard>, Added<ChessBoardLabel>),
    >,
) {
    let font = asset_server.load("Kosefont.ttf");
    for (entity, size, orientation) in query.iter() {
        let tile_size = size.0 / 8.;

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

                commands.spawn(text2d_file).set_parent(entity);
                commands.spawn(text2d_rank).set_parent(entity);
            }
        }
    }
}

pub fn update_square_system(
    mut commands: Commands,
    query: Query<(Entity, &Parent, &ChessBoardSquare), Changed<ChessBoardSquare>>,
    parents: Query<
        (&Handle<Image>, &Handle<TextureAtlasLayout>, &ChessBoardSize),
        With<ChessBoard>,
    >,
) {
    for (entity, parent, square) in query.iter() {
        commands.entity(entity).despawn_descendants();

        let Ok((texture, layout, size)) = parents.get(parent.get()) else {
            continue;
        };

        if square.piece.is_none() {
            continue;
        }

        let tile_size = size.0 / 8.;
        let board_size = tile_size.clamp(100. / 8., 400. / 8.);
        let sprite_size = (board_size - (100. / 8.)) / (300. / 8.) * 0.7 + 0.4;

        let sprite = SpriteBundle {
            texture: texture.clone(),
            transform: Transform::from_xyz(0., 0., 1.0).with_scale(Vec3::splat(sprite_size)),
            ..default()
        };

        let texture_atlas = TextureAtlas {
            layout: layout.clone(),
            index: square.piece.unwrap().into(),
        };

        commands
            .spawn(sprite)
            .insert(texture_atlas)
            .set_parent(entity);
    }
}

pub fn interaction_system(
    mut commands: Commands,
    squares: Query<&ChessBoardSquare>,
    positions: Query<(&ChessBoardPosition, &Children), Changed<ChessBoardPosition>>,
    interactable_squares: Query<Entity, With<InteractableSquare>>,
) {
    let Ok((position, children)) = positions.get_single() else {
        return;
    };

    for square in interactable_squares.iter() {
        commands.entity(square).remove::<InteractableSquare>();
    }

    for &child in children.iter() {
        let Ok(square) = squares.get(child) else {
            continue;
        };

        let Some(piece) = square.piece else {
            continue;
        };

        if piece.color == position.turn {
            commands.entity(child).insert(InteractableSquare);
        }
    }
}

pub fn pick_system(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    active_squares: Query<Entity, With<ActiveSquare>>,
    cursor_position: Res<CursorPosition>,
    squares: Query<
        (Entity, &Children, &Transform, &Sprite),
        (With<ChessBoardSquare>, With<InteractableSquare>),
    >,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    for (square, children, transform, sprite) in squares.iter() {
        let size = sprite.custom_size.unwrap();
        let origin = transform.translation.truncate();
        let rect = Rect::from_center_size(origin, size);

        if !rect.contains(cursor_position.0) {
            continue;
        }

        if let Ok(active_entity) = active_squares.get_single() {
            commands.entity(active_entity).remove::<ActiveSquare>();
        }

        let &child = children.iter().next().unwrap();
        commands.entity(square).insert(ActiveSquare);
        commands.entity(child).insert(MovingPiece);

        return;
    }
}

pub fn drag_system(
    buttons: Res<ButtonInput<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    active_squares: Query<&Transform, With<ActiveSquare>>,
    mut moving_pieces: Query<&mut Transform, (With<MovingPiece>, Without<ActiveSquare>)>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    let Ok(mut transform) = moving_pieces.get_single_mut() else {
        return;
    };

    let Ok(square_transform) = active_squares.get_single() else {
        return;
    };

    let parent_position = square_transform.translation.truncate();

    transform.translation.x = cursor_position.0.x - parent_position.x;
    transform.translation.y = cursor_position.0.y - parent_position.y;
}

pub fn place_system(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    active_squares: Query<(), With<ActiveSquare>>,
    capturable_squares: Query<
        (Entity, &Transform, &Sprite),
        (Without<InteractableSquare>, With<ChessBoardSquare>),
    >,
    mut moving_pieces: Query<
        (Entity, &mut Transform),
        (With<MovingPiece>, Without<ChessBoardSquare>),
    >,
) {
    if !buttons.just_pressed(MouseButton::Left) && !buttons.just_released(MouseButton::Left) {
        return;
    }

    if active_squares.get_single().is_err() {
        return;
    }

    if buttons.just_released(MouseButton::Left) {
        for (piece, mut transform) in moving_pieces.iter_mut() {
            transform.translation.x = 0.;
            transform.translation.y = 0.;
            commands.entity(piece).remove::<MovingPiece>();
        }
    }

    for (square_entity, square_transform, sprite) in capturable_squares.iter() {
        let position = square_transform.translation.truncate();
        let size = sprite.custom_size.unwrap();
        let rect = Rect::from_center_size(position, size);

        if !rect.contains(cursor_position.0) {
            continue;
        }

        commands.entity(square_entity).insert(CapturedSquare);
        return;
    }
}

pub fn move_system(
    mut commands: Commands,
    mut positions: Query<&mut ChessBoardPosition>,
    mut active: Query<
        (Entity, &mut ChessBoardSquare),
        (With<ActiveSquare>, Without<CapturedSquare>),
    >,
    mut captured: Query<
        (Entity, &mut ChessBoardSquare),
        (Added<CapturedSquare>, Without<ActiveSquare>),
    >,
) {
    let Ok(mut position) = positions.get_single_mut() else {
        return;
    };

    let Ok((captured_entity, mut captured_square)) = captured.get_single_mut() else {
        return;
    };

    let Ok((active_entity, mut active_square)) = active.get_single_mut() else {
        return;
    };

    commands.entity(active_entity).remove::<ActiveSquare>();
    commands.entity(captured_entity).remove::<CapturedSquare>();
    captured_square.piece = active_square.piece.take();
    position.pieces[captured_square.index as usize] = captured_square.piece;
    position.pieces[active_square.index as usize] = active_square.piece;
    position.change_turn();
}

#[cfg(feature = "square-inspector")]
pub fn update_square_color(
    mut squares: Query<(&ChessBoardSquare, &mut Sprite)>,
    palette: Res<ColorPalette>,
) {
    for (square, mut sprite) in squares.iter_mut() {
        let col = square.index % 8;
        let row = square.index / 8;
        sprite.color = if (col + row) % 2 == 0 {
            palette.white
        } else {
            palette.blue
        }
    }
}

#[cfg(feature = "square-inspector")]
pub fn update_interactable_square_color(
    mut interactable_squares: Query<&mut Sprite, With<InteractableSquare>>,
    palette: Res<ColorPalette>,
) {
    for mut sprite in interactable_squares.iter_mut() {
        sprite.color = palette.red;
    }
}

#[cfg(feature = "square-inspector")]
pub fn update_active_square_color(
    mut active_squares: Query<&mut Sprite, With<ActiveSquare>>,
    palette: Res<ColorPalette>,
) {
    for mut sprite in active_squares.iter_mut() {
        sprite.color = palette.yellow;
    }
}
