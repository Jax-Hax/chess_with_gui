#[derive(Clone, Copy,PartialEq)]
pub enum PieceColor {
    White,
    Black,
}
#[derive(Clone, Copy)]
pub enum PieceType {
    Bishop,
    Rook,
    Knight,
    Queen,
    King,
    Pawn,
}
#[derive(Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    pub num_times_moved: usize,
}
#[derive(Clone, Copy)]
pub enum Tile {
    Nothing,
    Something(Piece),
}


pub fn init_board(fen_string: String) -> [[Tile; 8]; 8]{
    let mut chessboard: [[Tile; 8]; 8] = [[Tile::Nothing; 8]; 8];
    let mut row = 7;
    let mut col = 0;
    for c in fen_string.chars() {
        if c == '/' {
            row -= 1;
            col = 0;
        } else if c.is_digit(10) {
            let empty_count: usize = c.to_digit(10).unwrap() as usize;
            col += empty_count;
        } else {
            // Handle the piece placement
            let tile = match c {
                'k' => Tile::Something(Piece {
                    color: PieceColor::Black,
                    piece_type: PieceType::King,
                    num_times_moved: 0,
                }),
                'q' => Tile::Something(Piece {
                    color: PieceColor::Black,
                    piece_type: PieceType::Queen,
                    num_times_moved: 0,
                }),
                'r' => Tile::Something(Piece {
                    color: PieceColor::Black,
                    piece_type: PieceType::Rook,
                    num_times_moved: 0,
                }),
                'b' => Tile::Something(Piece {
                    color: PieceColor::Black,
                    piece_type: PieceType::Bishop,
                    num_times_moved: 0,
                }),
                'n' => Tile::Something(Piece {
                    color: PieceColor::Black,
                    piece_type: PieceType::Knight,
                    num_times_moved: 0,
                }),
                'p' => Tile::Something(Piece {
                    color: PieceColor::Black,
                    piece_type: PieceType::Pawn,
                    num_times_moved: 0,
                }),
                'K' => Tile::Something(Piece {
                    color: PieceColor::White,
                    piece_type: PieceType::King,
                    num_times_moved: 0,
                }),
                'Q' => Tile::Something(Piece {
                    color: PieceColor::White,
                    piece_type: PieceType::Queen,
                    num_times_moved: 0,
                }),
                'R' => Tile::Something(Piece {
                    color: PieceColor::White,
                    piece_type: PieceType::Rook,
                    num_times_moved: 0,
                }),
                'B' => Tile::Something(Piece {
                    color: PieceColor::White,
                    piece_type: PieceType::Bishop,
                    num_times_moved: 0,
                }),
                'N' => Tile::Something(Piece {
                    color: PieceColor::White,
                    piece_type: PieceType::Knight,
                    num_times_moved: 0,
                }),
                'P' => Tile::Something(Piece {
                    color: PieceColor::White,
                    piece_type: PieceType::Pawn,
                    num_times_moved: 0,
                }),
                _ => Tile::Nothing,
            };

            if col < 8 {
                chessboard[col][7 - row] = tile;
                col += 1;
            }
        }
    }
    chessboard
}