
use chess::{ALL_SQUARES, Board, Color, Piece, Square};

pub fn base_piece_value_in_pawns(piece: Piece) -> f32 {
    match piece {
        Piece::Pawn => 1.0,
        Piece::Knight => 3.0,
        Piece::Bishop => 3.2,
        Piece::Rook => 5.0,
        Piece::Queen => 9.0,
        Piece::King => 0.0,
    }
}

fn get_board_material_for_color(board: &Board) -> (f32, f32) {
    let mut white_material = 0.0;
    let mut black_material = 0.0;

    for sq in ALL_SQUARES {
        if let Some(piece) = board.piece_on(sq) {
            if board.color_on(sq).is_some() && board.color_on(sq).unwrap() == Color::White {
                white_material += base_piece_value_in_pawns(piece) + get_map_bonus(piece, sq);
            } else {
                black_material += base_piece_value_in_pawns(piece) + get_map_bonus(piece, sq);
            }
        }
    }

    (white_material, black_material)
}

pub fn get_color_material_advantage(board: &Board) -> f32 {
    let (white_material, black_material) = get_board_material_for_color(board);
    white_material - black_material
}

pub const PAWN_MAP: [[f32; 8]; 8] = [
    [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0],
    [0.0, 0.0, 0.1, 0.2, 0.2, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.2, 0.2, 0.1, 0.0, 0.0],
    [0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
];

pub const KNIGHT_MAP: [[f32; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.25, 0.0, 0.25, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.25, 0.0, 0.25, 0.0, 0.25, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

pub const BISHOP_MAP: [[f32; 8]; 8] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.5, 0.0, 0.0, 0.5, 0.0, 0.0],
    [0.0, 0.0, 0.5, 0.0, 0.0, 0.5, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

pub const KING_MAP: [[f32; 8]; 8] = [
    [0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.5, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.5, 0.0],
];

pub fn get_map_bonus(piece: Piece, sq: Square) -> f32 {
    let r = sq.get_rank().to_index();
    let f = sq.get_file().to_index();

    match piece {
        Piece::Pawn => PAWN_MAP[r][f],
        Piece::Knight => KNIGHT_MAP[r][f],
        Piece::Bishop => BISHOP_MAP[r][f],
        Piece::King => KING_MAP[r][f],
        _ => 0.0f32,
    }
}
