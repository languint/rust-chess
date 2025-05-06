use crate::util::STARTING_FEN;
use chess::{Board, BoardStatus, ChessMove, Game};
use pgn::print_pgn;
use std::process::exit;
use std::str::FromStr;

use crate::evaluate::Evaluator;
use crate::util::util::{clear_screen, pretty_print_board, print_banner, print_error, print_label};
use clap::{Parser, Subcommand};

mod evaluate;
mod evaluations;
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
                .map_err(|_| format!("Invalid FEN Position {}", position));

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
        let (score, best_move, leaf_counter) = evaluator.negamax(
            &board,
            0,
            depth,
            f32::NEG_INFINITY,
            f32::INFINITY,
            board.side_to_move(),
        );

        // If there are no legal moves, we’ll fall through and detect it below
        let mv = match best_move {
            Some(m) => m,
            None => {
                break;
            }
        };

        game.make_move(mv);
        board = board.make_move_new(mv);

        history.push(mv);

        match board.status() {
            BoardStatus::Ongoing => {
                if game.can_declare_draw() {
                    game.declare_draw();
                    game.accept_draw();
                    break;
                }
            }
            BoardStatus::Checkmate => {
                break;
            }
            _ => {
                break;
            }
        }

        clear_screen();
        pretty_print_board(&board, Some(mv));
        print_label("Move", &mv.to_string(), 0);
        print_label("Score", score.to_string().as_str(), 0);
        print_label("Evaluated Leafs", &leaf_counter.to_string(), 0);
    }

    println!("Game is over, engine has no legal moves!");
    print_pgn(&history, game);
    Ok(())
}
