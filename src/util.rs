pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub mod util {
    use colored::{Colorize};

    pub fn print_banner() {
        let font = FIGfont::from_file("resources/basic.flf").unwrap();
        let figure = font.convert("Stahrfish");

        assert!(figure.is_some(), "Failed to load font file!");

        println!("{}", figure.unwrap().to_string().bold().bright_blue());
    }

    pub fn print_label(label: &str, message: &str, depth: i32) {
        println!(
            "{}{} `{}`",
            "\n".repeat(depth as usize),
            label.bold().bright_green(),
            message.normal()
        );
    }

    pub fn print_warning(message: &str, depth: i32) {
        println!(
            "{}{} `{}`",
            "\n".repeat(depth as usize),
            "Warning".bold().bright_red(),
            message.normal()
        );
    }

    pub fn print_error(message: &str, depth: i32) {
        println!(
            "{}{} `{}`",
            "\n".repeat(depth as usize),
            "Error".bold().bright_red(),
            message.normal()
        );
    }

    pub fn clear_screen() {
        clearscreen::clear().expect("failed to clear screen");
    }

    use chess::{Board, ChessMove, Piece, Square};
    use figlet_rs::FIGfont;

    pub fn pretty_print_board(board: &Board, last_move: Option<ChessMove>) {
        fn piece_char(piece: Piece, color: chess::Color) -> char {
            let c = match piece {
                Piece::Pawn => 'P',
                Piece::Knight => 'N',
                Piece::Bishop => 'B',
                Piece::Rook => 'R',
                Piece::Queen => 'Q',
                Piece::King => 'K',
            };
            if color == chess::Color::White {
                c
            } else {
                c.to_ascii_lowercase()
            }
        }

        for rank in (0..8).rev() {
            print!("{}  ", rank + 1);
            for file in 0..8 {
                let sq = Square::make_square(
                    chess::Rank::from_index(rank),
                    chess::File::from_index(file),
                );
                let mut square_repr = String::from(".");

                if let Some(piece) = board.piece_on(sq) {
                    let color = board.color_on(sq).unwrap();
                    let symbol = piece_char(piece, color).to_string();
                    square_repr = symbol;
                    if color == chess::Color::White {
                        square_repr = square_repr.black().on_bright_white().to_string();
                    } else {
                        square_repr = square_repr.bright_white().to_string();
                    }
                }

                if let Some(mv) = last_move {
                    if sq == mv.get_dest() {
                        let ch = board
                            .piece_on(sq)
                            .map(|p| piece_char(p, board.color_on(sq).unwrap()).to_string())
                            .unwrap_or_else(|| ".".to_string());
                        square_repr = ch.bold().black().on_yellow().to_string();
                    }
                }

                print!("{} ", square_repr);
            }
            println!();
        }
        println!("\n   a b c d e f g h");
    }
}
