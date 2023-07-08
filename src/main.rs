use ggez::{
    event,
    glam::*,
    graphics::{self, Color,MeshBuilder, DrawMode, Rect, Mesh, DrawParam,Image, Canvas},
    conf::{WindowMode, WindowSetup},
    Context, GameResult,
};
use std::{path};
mod board;
use board::{init_board,Tile,PieceColor,PieceType,Piece};

struct MainState {
    chessboard: [[Tile; 8]; 8],
    board: Mesh,
    king_w: Image,
    queen_w: Image,
    rook_w: Image,
    knight_w: Image,
    bishop_w: Image,
    pawn_w: Image,
    king_b: Image,
    queen_b: Image,
    rook_b: Image,
    knight_b: Image,
    bishop_b: Image,
    pawn_b: Image,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let board = init_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());
        let king_w = Image::from_path(ctx, "/king_w.png")?;
        let queen_w = Image::from_path(ctx, "/queen_w.png")?;
        let rook_w = Image::from_path(ctx, "/rook_w.png")?;
        let knight_w = Image::from_path(ctx, "/knight_w.png")?;
        let bishop_w = Image::from_path(ctx, "/bishop_w.png")?;
        let pawn_w = Image::from_path(ctx, "/pawn_w.png")?;
        let king_b = Image::from_path(ctx, "/king_b.png")?;
        let queen_b = Image::from_path(ctx, "/queen_b.png")?;
        let rook_b = Image::from_path(ctx, "/rook_b.png")?;
        let knight_b = Image::from_path(ctx, "/knight_b.png")?;
        let bishop_b = Image::from_path(ctx, "/bishop_b.png")?;
        let pawn_b = Image::from_path(ctx, "/pawn_b.png")?;
        const TILE_SIZE: f32 = 100.0;
        let mut mesh_builder = MeshBuilder::new();
        for row in 0..8 {
            for col in 0..8 {
                let x = col as f32 * TILE_SIZE;
                let y = row as f32 * TILE_SIZE;

                let color = if (row + col) % 2 == 0 {
                    Color::from_rgb(240, 217, 181) // Light tile color
                } else {
                    Color::from_rgb(181, 136, 99) // Dark tile color
                };

                let rect = Rect::new(x, y, TILE_SIZE, TILE_SIZE);
                mesh_builder.rectangle(DrawMode::fill(), rect, color)?;
            }
        }
        let mesh: Mesh = Mesh::from_data(ctx, mesh_builder.build());
        let state = MainState { chessboard: board, board: mesh, king_w: king_w,queen_w: queen_w,rook_w: rook_w,knight_w: knight_w,bishop_w: bishop_w,pawn_w: pawn_w,king_b: king_b,queen_b: queen_b,rook_b: rook_b,knight_b: knight_b,bishop_b: bishop_b,pawn_b: pawn_b};
        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        canvas.draw( &self.board, DrawParam::default());
        const TILE_SIZE: f32 = 100.0;
        for row in 0..8 {
            for col in 0..8 {
                let x = col as f32 * TILE_SIZE;
                let y = row as f32 * TILE_SIZE;
                match &self.chessboard[col][row] {
                    Tile::Something(piece) => draw_piece(piece, x, y, self, &mut canvas),
                    _ => {},
                }
                
            }
        }
        canvas.finish(ctx)?;
        Ok(())
    }
}
fn draw_piece(piece: &Piece, x:f32, y:f32, state:&MainState,canvas: &mut Canvas){
    let draw_param = DrawParam::default().dest([x, y]);
    match piece {
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::King,
        } => canvas.draw(&state.king_b,draw_param),
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::Pawn,
        } => canvas.draw(&state.pawn_b,draw_param),
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::Queen,
        } => canvas.draw(&state.queen_b,draw_param),
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::Knight,
        } => canvas.draw(&state.knight_b,draw_param),
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::Rook,
        } => canvas.draw(&state.rook_b,draw_param),
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::Bishop,
        } => canvas.draw(&state.bishop_b,draw_param),
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::King,
        } => canvas.draw(&state.king_w,draw_param),
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::Pawn,
        } => canvas.draw(&state.pawn_w,draw_param),
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::Queen,
        } => canvas.draw(&state.queen_w,draw_param),
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::Knight,
        } => canvas.draw(&state.knight_w,draw_param),
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::Rook,
        } => canvas.draw(&state.rook_w,draw_param),
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::Bishop,
        } => canvas.draw(&state.bishop_w,draw_param),
    }
}
pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");
    let window_setup = WindowSetup::default().title("Chessboard");
    let window_mode = WindowMode::default().fullscreen_type(ggez::conf::FullscreenType::Desktop);
    let cb = ggez::ContextBuilder::new("Chess", "Jax Bulbrook").window_setup(window_setup).window_mode(window_mode);
    let cb = cb.add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}