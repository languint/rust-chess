use chess::{ChessMove, Game, Board, Color, GameResult};

pub fn print_pgn(moves: &[ChessMove], game: &Game) {

    let result_tag = match game.result().unwrap_or(GameResult::DrawAccepted) {
        GameResult::WhiteCheckmates => "1-0",
        GameResult::BlackCheckmates => "0-1",
        GameResult::Stalemate
        | GameResult::DrawAccepted
        | _                         => "1/2-1/2",
    };
    
    println!("[Event \"Engine Self-Play\"]");
    println!("[Site \"?\"]");
    println!("[Date \"2025.04.25\"]");
    println!("[Round \"?\"]");
    println!("[White \"Engine\"]");
    println!("[Black \"Engine\"]");
    println!("[Result \"{}\"]\n", result_tag);
    
    let mut pgn_body = String::new();
    for (i, mv) in moves.iter().enumerate() {
        if i % 2 == 0 {
            let move_num = (i / 2) + 1;
            pgn_body.push_str(&format!("{}.", move_num));
        }
        
        pgn_body.push_str(&format!(" {}", mv));

        if i % 2 == 1 {
            pgn_body.push('\n');
        }
    }
    
    if moves.len() % 2 == 1 {
        pgn_body.push('\n');
    }

    pgn_body.push_str(&format!("{}", result_tag));

    println!("{}", pgn_body);
}
