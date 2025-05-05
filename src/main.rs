use crate::util::STARTING_FEN;
use chess::{Board, ChessMove, Game};
use std::process::exit;
use std::str::FromStr;

use crate::evaluate::Evaluator;
use crate::util::util::{clear_screen, pretty_print_board, print_banner, print_error, print_label};
use clap::{Parser, Subcommand};

mod evaluate;
mod pgn;
mod util;

#[derive(Parser, Debug)]
#[command(name = "Stahrfish")]
#[command(about = "A chess engine CLI", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    PlaySelf {
        #[arg(short, long, default_value = "5")]
        depth: usize,
        #[arg(default_value = STARTING_FEN)]
        position: String,
    },
}

fn main() {
    let args = Args::parse();

    clear_screen();
    print_banner();

    let result = command(args);

    match result {
        Ok(_) => (),
        Err(e) => {
            print_error(e.as_str(), 0);
        }
    }
}

fn command(args: Args) -> Result<(), String> {
    match args.command {
        Command::PlaySelf { depth, position } => {
            if depth == 0 {
                print_error("Depth argument must be greater than 0!", 0);
                exit(1);
            }

            print_label("Depth", &depth.to_string(), 0);
            print_label("Position", &position.to_string(), 0);

            let board = Board::from_str(position.as_str())
                .map_err(|e| format!("Invalid FEN Position {}", position));

            // Clone the board since ? consumes it.
            let mut game = Game::new_with_board(board.clone()?);

            return selfplay_loop(&mut game, board.unwrap(), depth);
        }
    }
}

fn selfplay_loop(game: &mut Game, mut board: Board, depth: usize) -> Result<(), String> {
    let evaluator = Evaluator::new();
    let mut history: Vec<ChessMove> = Vec::new();

    loop {
        let (score, best_move) = evaluator.negamax(
            &board,
            depth,
            f32::NEG_INFINITY,
            f32::INFINITY,
            board.side_to_move(),
        );

        let mv = best_move.ok_or("No legal moves available!")?;
        history.push(mv);

        // Apply move
        game.make_move(mv);
        board = board.make_move_new(mv);

        // Display
        clear_screen();
        pretty_print_board(&board, Option::from(mv));
        println!("Move {}: {} (Score: {})", history.len(), mv, score);
    }
}
