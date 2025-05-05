use chess::{Board, BoardStatus, ChessMove, Color, MoveGen};

pub struct Evaluator;

pub type EvaluationResult = (f32, Option<ChessMove>);

impl Evaluator {
    pub fn new() -> Self {
        Evaluator
    }

    pub fn heuristic(&self, board: &Board) -> f32 {
        // TODO "Improve heuristics with material, king safety, center, etc."

        let mut score = 0.0;

        score
    }

    pub fn order_moves(&self, board: &Board) -> Vec<ChessMove> {
        let moves = MoveGen::new_legal(board).collect();

        moves
    }

    /// Negamax with alpha-beta pruning.
    /// https://en.wikipedia.org/wiki/Negamax
    pub fn negamax(
        &self,
        board: &Board,
        depth: usize,
        mut alpha: f32,
        beta: f32,
        color: Color,
    ) -> EvaluationResult {
        // Terminal node or maximum depth reached
        match board.status() {
            BoardStatus::Ongoing if depth > 0 => {}
            BoardStatus::Stalemate => return (0.0, None),
            BoardStatus::Checkmate => {
                let mate_score = f32::NEG_INFINITY;
                return (mate_score, None);
            }
            BoardStatus::Ongoing => {
                // depth == 0
                let val = self.heuristic(board);
                return (if color == Color::White { val } else { -val }, None);
            }
        }

        let mut best_score = f32::NEG_INFINITY;
        let mut best_move = None;

        // Generate and order moves
        let moves: Vec<ChessMove> = MoveGen::new_legal(board).collect();

        for mv in moves {
            let new_board = board.make_move_new(mv);
            // Flip color perspective
            let (score, _) = self.negamax(&new_board, depth - 1, -beta, -alpha, !color);
            let score = -score;

            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }

            alpha = alpha.max(score);
            if alpha >= beta {
                break; // beta cutoff
            }
        }

        (best_score, best_move)
    }
}
