use ggez::{
    event::{self, MouseButton},
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
    holding: HoldingPiece,
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
enum HoldingPiece{
    False,
    True(usize, usize), //x,y
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

                let rect = Rect::new(x+100.0, y+100.0, TILE_SIZE, TILE_SIZE);
                mesh_builder.rectangle(DrawMode::fill(), rect, color)?;
            }
        }
        let mesh: Mesh = Mesh::from_data(ctx, mesh_builder.build());
        let state = MainState { chessboard: board, board: mesh, holding: HoldingPiece::False, king_w: king_w,queen_w: queen_w,rook_w: rook_w,knight_w: knight_w,bishop_w: bishop_w,pawn_w: pawn_w,king_b: king_b,queen_b: queen_b,rook_b: rook_b,knight_b: knight_b,bishop_b: bishop_b,pawn_b: pawn_b};
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
                match &self.holding{
                    HoldingPiece::True(x_pos, y_pos) => {if x_pos == &col && y_pos == &row{
                        let pos = ctx.mouse.position();
                        match &self.chessboard[*x_pos][*y_pos] {
                            Tile::Something(piece) => draw_piece(piece, pos.x, pos.y, self, &mut canvas,true),
                            _ => {},
                        }
                        
                        continue;
                    }},
                    _ => {}
                }
                match &self.chessboard[col][row] {
                    Tile::Something(piece) => draw_piece(piece, x, y, self, &mut canvas,false),
                    _ => {},
                }
                
            }
        }
        canvas.finish(ctx)?;
        Ok(())
    }
    fn mouse_button_down_event(&mut self, ctx: &mut Context, btn: MouseButton, x: f32, y: f32) -> GameResult{
        match btn {
            MouseButton::Left => {
                check_if_touching_piece(x,y,self);
            }

            _ => (),
        }
        Ok(())
    }
    fn mouse_button_up_event(&mut self, ctx: &mut Context, btn: MouseButton, x: f32, y: f32) -> GameResult{
        match btn {
            MouseButton::Left => {
                check_if_can_place_piece(x,y,self);
            }

            _ => (),
        }
        Ok(())
    }
}
fn draw_piece(piece: &Piece, x:f32, y:f32, state:&MainState,canvas: &mut Canvas, is_mouse_coords: bool){
    const OFFSET_X: f32 = 100.0 + 15.0;
    const OFFSET_Y: f32 = 100.0 + 10.0; //screen offset plus offset to make the images align with the tiles
    let draw_param = if !is_mouse_coords{DrawParam::default().dest([x+OFFSET_X, y+OFFSET_Y])} else {DrawParam::default().dest([x-35.0, y-35.0])};
    match piece {
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::King,
            ..
        } => canvas.draw(&state.king_b,draw_param),
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::Pawn,..
        } => canvas.draw(&state.pawn_b,draw_param),
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::Queen,..
        } => canvas.draw(&state.queen_b,draw_param),
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::Knight,..
        } => canvas.draw(&state.knight_b,draw_param),
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::Rook,..
        } => canvas.draw(&state.rook_b,draw_param),
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::Bishop,..
        } => canvas.draw(&state.bishop_b,draw_param),
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::King,..
        } => canvas.draw(&state.king_w,draw_param),
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::Pawn,..
        } => canvas.draw(&state.pawn_w,draw_param),
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::Queen,..
        } => canvas.draw(&state.queen_w,draw_param),
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::Knight,..
        } => canvas.draw(&state.knight_w,draw_param),
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::Rook,..
        } => canvas.draw(&state.rook_w,draw_param),
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::Bishop,..
        } => canvas.draw(&state.bishop_w,draw_param),
    }
}
fn check_if_touching_piece(x: f32,y: f32,state: &mut MainState){
    //TODO - make sure it is checking if it is the right color
    if x >= 115.0 && x <= 885.0 && y >= 105.0 && y <= 885.0{
        //in bounds of the board
        let x = x / 100.0;
        let y = y / 100.0;
        if x % 1.0 >= 0.15 &&  x % 1.0 <= 0.8 &&  y % 1.0 >= 0.1 &&  y % 1.0 <= 0.85{
            //In bounds of a square
            let x = x.floor() as usize - 1;
            let y = y.floor() as usize - 1;
            match state.chessboard[x][y] {
                Tile::Something(_) => {state.holding = HoldingPiece::True(x, y)},
                _ => {},
            }
        }
    }
}
fn check_if_can_place_piece(x: f32,y: f32,state: &mut MainState){
    //check if holding, if so assign pos1. Then check if mouse coords lands on a tile, if so pass it into a check function.
    //that check function has to check first if it is either opposite color or empty tile. Then, it does all the valid checks. If it is valid, replace the new tile with it and get rid of the old one.
    let mut pos1: (usize,usize) = (0,0);
    match state.holding{
        HoldingPiece::True(x_pos, y_pos) => {pos1 = (x_pos,y_pos);},
        _ => {}
    }
    let (x1,y1)=pos1;
    println!("{},{}",x1,y1);
    if x >= 115.0 && x <= 885.0 && y >= 105.0 && y <= 885.0{
        //in bounds of the board
        let x = x / 100.0;
        let y = y / 100.0;
        if x % 1.0 >= 0.25 &&  x % 1.0 <= 0.7 &&  y % 1.0 >= 0.2 &&  y % 1.0 <= 0.75{
            //In bounds of a square
            let x = x.floor() as usize - 1;
            let y = y.floor() as usize - 1;
            let pos2 = (x,y);
            check_if_valid_move(pos1,pos2,state);
        }
    }
    state.holding = HoldingPiece::False;
}
fn check_if_valid_move(pos1: (usize,usize),pos2: (usize,usize),state: &mut MainState){
    let (col1,row1) = pos1;
    let (col2,row2) = pos2;
    println!("col1: {}, row1: {}, col2: {}, row2: {}",col1,row1,col2,row2);
    let tile1 = state.chessboard[col1][row1];
    let tile2 = state.chessboard[col2][row2];
    if row1 == row2 && col1 == col2 {
        println!("Same tile");
        return;
    }
    if !is_opposite_color_or_none(&tile1, &tile2){
        return;
    }
    state.chessboard[col2][row2] = tile1;
    state.chessboard[col1][row1] = Tile::Nothing;
}
fn is_opposite_color_or_none(tile1: &Tile, tile2: &Tile) -> bool{
    match tile1 {
        Tile::Nothing => {false}, // Empty tile
        Tile::Something(piece) => {
            let color1 = piece.color; // Opposite color of the first
            match tile2 {
                Tile::Nothing => return true, // Empty tile
                Tile::Something(piece) => {
                    return !(piece.color == color1); // Opposite color of the first
                }
            }
        }
    }
}
pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");
    let window_setup = WindowSetup::default().title("Chessboard");
    let window_mode = WindowMode::default().dimensions(1000.0, 1000.0);
    let cb = ggez::ContextBuilder::new("Chess", "Jax Bulbrook").window_setup(window_setup).window_mode(window_mode);
    let cb = cb.add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}