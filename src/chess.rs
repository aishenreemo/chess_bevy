use std::fmt::Display;

use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component, Clone)]
pub struct ChessBoard {
    pub orientation: PieceColor,
    pub position: Vec2,
    pub size: f32,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum PieceColor {
    White = 0,
    Black = 1,
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

#[derive(Component, Debug)]
pub struct ChessPosition {
    pub pieces: [Option<ChessPiece>; 64],
    pub _turn: PieceColor,
}

impl ChessPosition {
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

        ChessPosition {
            pieces,
            _turn: turn,
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

impl Into<PieceColor> for char {
    fn into(self) -> PieceColor {
        match self.to_ascii_lowercase() {
            'w' => PieceColor::White,
            'b' => PieceColor::Black,
            _ => panic!("Invalid FEN character: {}", self),
        }
    }
}

impl Display for ChessPosition {
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

impl Into<usize> for ChessPiece {
    fn into(self) -> usize {
        use PieceVariant::*;
        use PieceColor::*;

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
