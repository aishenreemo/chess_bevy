pub mod cursor;
pub mod systems;

use bevy::prelude::*;
use systems::*;

pub struct ChessPlugin<S: States + Copy>(pub S);

impl<S: States + Copy> Plugin for ChessPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_board_system, spawn_labels_system, interaction_system)
                .run_if(in_state(self.0))
                .chain(),
        );

        app.add_systems(
            Update,
            (
                place_system,
                pick_system,
                drag_system,
                move_system,
                update_square_system,
            )
                .run_if(in_state(self.0))
                .chain(),
        );

        #[cfg(feature = "square-inspector")]
        {
            app.add_systems(
                Update,
                (
                    update_square_color,
                    update_interactable_square_color,
                    update_active_square_color,
                )
                    .run_if(in_state(self.0))
                    .chain(),
            );
        }
    }
}

#[derive(Bundle, Default)]
pub struct ChessBoardBundle {
    pub marker: ChessBoard,
    pub size: ChessBoardSize,
    pub offset: ChessBoardOffset,
    pub orientation: ChessBoardOrientation,
    pub position: ChessBoardPosition,
}

#[derive(Bundle)]
pub struct ChessBoardAssetBundle {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

impl ChessBoardAssetBundle {
    pub fn new(
        asset_server: &AssetServer,
        texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    ) -> Self {
        let texture_atlas_layout =
            TextureAtlasLayout::from_grid(UVec2::splat(45), 6, 2, None, None);

        Self {
            texture: asset_server.load("pieces_spritesheet.png"),
            layout: texture_atlas_layouts.add(texture_atlas_layout),
        }
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct ChessBoard;

#[derive(Component)]
pub struct ChessBoardLabel;

#[derive(Component)]
pub struct InteractableSquare;

#[derive(Component)]
pub struct ActiveSquare;

#[derive(Component)]
pub struct CapturedSquare;

#[derive(Component)]
pub struct MovingPiece;

#[derive(Component, Default, Debug, Clone)]
pub struct ChessBoardSquare {
    pub piece: Option<ChessPiece>,
    pub index: u8,
}

#[derive(Component, Clone, Debug, Default)]
pub struct ChessBoardOrientation(pub PieceColor);

#[derive(Component, Default, Clone)]
pub struct ChessBoardSize(pub f32);

#[derive(Component, Default, Clone)]
pub struct ChessBoardOffset(pub Vec2);

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum PieceColor {
    #[default]
    White = 0,
    Black = 1,
}

impl Into<PieceColor> for char {
    fn into(self) -> PieceColor {
        match self.to_ascii_lowercase() {
            'w' => PieceColor::White,
            'b' => PieceColor::Black,
            _ => panic!("Invalid FEN character: {}", self),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum PieceVariant {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ChessPiece {
    pub color: PieceColor,
    pub variant: PieceVariant,
}

impl Into<usize> for ChessPiece {
    fn into(self) -> usize {
        use PieceColor::*;
        use PieceVariant::*;

        match (self.variant, self.color) {
            (King, White) => 0,
            (Queen, White) => 1,
            (Bishop, White) => 2,
            (Knight, White) => 3,
            (Rook, White) => 4,
            (Pawn, White) => 5,
            (King, Black) => 6,
            (Queen, Black) => 7,
            (Bishop, Black) => 8,
            (Knight, Black) => 9,
            (Rook, Black) => 10,
            (Pawn, Black) => 11,
        }
    }
}

impl Into<char> for ChessPiece {
    fn into(self) -> char {
        let ch = match self.variant {
            PieceVariant::King => 'k',
            PieceVariant::Queen => 'q',
            PieceVariant::Rook => 'r',
            PieceVariant::Bishop => 'b',
            PieceVariant::Knight => 'n',
            PieceVariant::Pawn => 'p',
        };

        if self.color == PieceColor::White {
            ch.to_uppercase().to_string().chars().next().unwrap()
        } else {
            ch
        }
    }
}

impl Into<ChessPiece> for char {
    fn into(self) -> ChessPiece {
        let color = if self.is_uppercase() {
            PieceColor::White
        } else {
            PieceColor::Black
        };

        let variant = match self.to_ascii_lowercase() {
            'k' => PieceVariant::King,
            'q' => PieceVariant::Queen,
            'r' => PieceVariant::Rook,
            'b' => PieceVariant::Bishop,
            'n' => PieceVariant::Knight,
            'p' => PieceVariant::Pawn,
            _ => panic!("Invalid FEN character: {}", self),
        };

        ChessPiece { color, variant }
    }
}

#[derive(Component, Clone)]
pub struct ChessBoardPosition {
    pub pieces: [Option<ChessPiece>; 64],
    pub turn: PieceColor,
}

impl Default for ChessBoardPosition {
    fn default() -> Self {
        ChessBoardPosition::from_fen(ChessBoardPosition::DEFAULT_WHITE_FEN)
    }
}

impl ChessBoardPosition {
    pub const DEFAULT_BLACK_FEN: &'static str =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
    pub const DEFAULT_WHITE_FEN: &'static str =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    pub fn from_fen(fen: &str) -> Self {
        let mut pieces = [None; 64];

        let mut iterator = fen.split_whitespace();
        let mut square_index: usize = 0;

        let fen_ranks = iterator.next().expect("Invalid FEN: Missing fen position.");

        for rank in fen_ranks.split("/") {
            for c in rank.chars() {
                if let Some(num) = c.to_digit(10) {
                    square_index += num as usize;
                } else {
                    let piece = c.into();
                    pieces[square_index] = Some(piece);
                    square_index += 1;
                }
            }
        }

        let turn = iterator.next().expect("Invalid FEN: Missing current turn.");
        let turn = turn.chars().next().unwrap().into();

        ChessBoardPosition { pieces, turn }
    }

    pub fn change_turn(&mut self) {
        self.turn = if self.turn == PieceColor::White {
            PieceColor::Black
        } else {
            PieceColor::White
        };
    }
}

impl std::fmt::Debug for ChessBoardPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+-+-+-+-+-+-+-+-+")?;
        for i in (0..8).rev() {
            for j in (0..8).rev() {
                let ch = match self.pieces[(i * 8) + j] {
                    Some(piece) => piece.into(),
                    None => ' ',
                };

                write!(f, "|{}", ch)?;
            }

            writeln!(f, "|")?;
            writeln!(f, "+-+-+-+-+-+-+-+-+")?;
        }

        Ok(())
    }
}
