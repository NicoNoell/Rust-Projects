mod board;
use board::Board;
use board::Player;

mod input;

fn main() {
    println!("Welcome to this game of TicTacToe :)");
    println!("(Simply input a number from 1 to 9 corresponding to where you want to place you mark)\n");

    let mut board = Board::default();
    let mut turn = Player::X;

    while board.has_winner().is_none() && board.has_space() {
        println!("Player {}'s turn:\n", turn);
        println!("{}\n", board);
        if !board.put(input::get_index(), turn) {
            println!("Place already occupied\n");
            continue;
        }
        Player::switch(&mut turn);
    }

    if let Some(winner) = board.has_winner() {
        println!("{} has Won!", winner);
    } else {
        println!("Draw :)");
    }
}
