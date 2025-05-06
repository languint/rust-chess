use chess::{Board, ChessMove, MoveGen};

use crate::evaluations::material_evaluations::base_piece_value_in_pawns;

// Orders moves based on MVV/LVA, and then MVV/LVA for captures.
pub fn order_moves(board: &Board) -> Vec<ChessMove> {
    let mut captures: Vec<(ChessMove, f32)> = Vec::new();
    let mut checks: Vec<ChessMove> = Vec::new();
    let mut non_caps: Vec<ChessMove> = Vec::new();

    for mv in MoveGen::new_legal(board) {
        if let Some(victim) = board.piece_on(mv.get_dest()) {
            let attacker = board.piece_on(mv.get_source()).unwrap();

            let victim_value = base_piece_value_in_pawns(victim);
            let attacker_value = base_piece_value_in_pawns(attacker);

            let score = victim_value * 10.0 - attacker_value;
            captures.push((mv, score));
        } else {
            let next = board.make_move_new(mv);
            if next.checkers().to_size(0) == 0 {
                non_caps.push(mv);
            } else {
                checks.push(mv);
            }
        }
    }

    captures.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let mut ordered = captures.into_iter().map(|(mv, _)| mv).collect::<Vec<_>>();

    ordered.extend(checks);
    ordered.extend(non_caps);

    ordered
}
