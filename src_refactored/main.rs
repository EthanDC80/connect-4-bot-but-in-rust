mod board;
mod consts;
mod player;

use std::io;
use board::{Board, SimpleBoard, Error};
use player::Player;

struct Connect4<B: Board<Error>> {
    board: B,
    current_player: Player,
    game_over: bool,
}

impl<B: Board<Error>> Connect4<B> {
    fn new() -> Self {
        Self {
            board: B::new(),
            current_player: Player::default(),
            game_over: false,
        }
    }

    fn play(&mut self) {
        while !self.game_over {
            println!("{}", self.board);
            println!("{}, (0-6):", self.current_player);

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read input");

            let col: usize = match input.trim().parse() {
                Ok(col) => col,
                Err(_) => {
                    println!("thats not a number dumbass");
                    continue;
                }
            };

            match self.board.make_move(col, self.current_player) {
                Ok(_) => {
                    if self.board.check_winner() {
                        println!("{}", self.board);
                        println!("{} wins", self.current_player);
                        self.game_over = true;
                    }
                    else {
                        self.current_player.switch();
                    }
                }
                Err(msg) => eprintln!("{}", msg),
            }
        }
    }
}

fn main() {
    let mut connect4 = Connect4::<SimpleBoard>::new();
    connect4.play()
}
