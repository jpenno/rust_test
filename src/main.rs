mod board;
mod point;
mod turn;

use board::Board;
use point::Point;
use std::io;
use turn::Turn;

fn main() {
    let mut board = Board::new();
    let mut turn = Turn::new();
    println!("Game started");
    board.print();

    loop {
        println!("[{}]: turn", turn.char);
        println!("Enter your move: ");
        loop {
            if board.place_char(player_input(), turn.char) {
                break;
            }
        }
        board.print();
        if board.check_win(turn.char) {
            println!("Player [{}] won", turn.char);
            break;
        }
        turn.swap();
    }
}

fn player_input() -> Point {
    loop {
        let mut player_input = String::new();
        io::stdin()
            .read_line(&mut player_input)
            .expect("Failed to read line");
        let chords: Vec<&str> = player_input.trim().split(',').collect();

        let mut vaild_input = false;
        if chords[0].to_string().trim().parse::<usize>().is_ok()
            && chords[1].to_string().trim().parse::<usize>().is_ok()
        {
            vaild_input = true;
        }

        if chords.len() == 2 && vaild_input {
            return Point {
                x: chords[0]
                    .to_string()
                    .trim()
                    .parse::<usize>()
                    .expect("Bad x cord"),
                y: chords[1]
                    .to_string()
                    .trim()
                    .parse::<usize>()
                    .expect("Bad y cord"),
            };
        } else {
            println!("Input wrong try something like this [1,1]");
        }
    }
}
