use chess::{self, Board, BoardStatus, Color, MoveGen, Piece, ALL_COLORS, ALL_PIECES};
use std::{
    cmp::{max, min},
    str::FromStr,
};

fn main() {
    println!("Welcome to the Craken chess engine!");
    let board = Board::from_str("1nK1Q3/8/8/8/8/4k3/1r6/8 b - - 0 2").unwrap();
    println!(
        "Craken Engine evaluates this position at {}.",
        minimax(board, 5, Color::White)
    );
}

fn minimax(board: Board, depth: usize, maximizing_player: Color) -> i32 {
    if depth == 0 || board.status() != BoardStatus::Ongoing {
        return evaluate_board(&board, maximizing_player);
    }

    if board.side_to_move() == maximizing_player {
        let mut best_value = -9000;

        for legal_move in MoveGen::new_legal(&board) {
            best_value = max(
                best_value,
                minimax(
                    board.make_move_new(legal_move),
                    depth - 1,
                    maximizing_player,
                ),
            );
        }

        return best_value;
    } else {
        let mut worst_value = 9000;

        for legal_move in MoveGen::new_legal(&board) {
            worst_value = min(
                worst_value,
                minimax(
                    board.make_move_new(legal_move),
                    depth - 1,
                    maximizing_player,
                ),
            );
        }

        return worst_value;
    }
}

fn evaluate_board(board: &Board, maximizing_player: Color) -> i32 {
    match board.status() {
        BoardStatus::Checkmate => {
            if board.side_to_move() != maximizing_player {
                9000
            } else {
                -9000
            }
        }
        BoardStatus::Stalemate => 0,
        BoardStatus::Ongoing => {
            let mut pieces_value = 0;
            for piece in ALL_PIECES {
                for color in ALL_COLORS {
                    let number_of_pieces =
                        (board.color_combined(color) & board.pieces(piece)).popcnt();
                    let piece_value = get_piece_value(&piece) * number_of_pieces as i32;
                    pieces_value += if color == maximizing_player {
                        piece_value
                    } else {
                        -piece_value
                    }
                }
            }
            return pieces_value;
        }
    }
}

fn get_piece_value(piece: &Piece) -> i32 {
    match piece {
        Piece::King => 0,
        Piece::Queen => 900,
        Piece::Rook => 500,
        Piece::Knight => 300,
        Piece::Bishop => 300,
        Piece::Pawn => 100,
    }
}

// fn get_opposite_color(color: &Color) -> Color {
//     match color {
//         Color::Black => Color::White,
//         Color::White => Color::Black,
//     }
// }
