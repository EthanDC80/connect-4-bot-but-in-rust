mod board;
mod consts;
mod minimax;
mod player;

use board::{Board, Error, SimpleBoard};
use minimax::{minimax};
use player::Player;
use std::io;

struct Connect4<B: Board<Error>> {
    board: B,
    current_player: Player,
    bot_player: Player,
    game_over: bool,
}

impl<B: Board<Error>> Connect4<B> {
    fn new() -> Self {
        Self {
            board: B::new(),
            current_player: Player::default(),
            bot_player: Player::Circle,
            game_over: false,
        }
    }

    fn find_best_move(&mut self) -> Option<usize> {
        minimax(&mut self.board, 7, self.bot_player).1
    }

    fn play(&mut self) {
        while !self.game_over {
            println!("{}", self.board);

            if self.current_player == self.bot_player {
                let bot_move = self.find_best_move();
                if let Some(col) = bot_move {
                    println!("Bot's move: {}", col + 1);
                    match self.board.make_move(col, self.current_player) {
                        Ok(_) => {
                            if self.board.check_winner() {
                                println!("{}", self.board);
                                println!("{} wins", self.current_player);
                                self.game_over = true;
                            } else {
                                self.current_player.switch();
                            }
                        }
                        Err(msg) => eprintln!("{}", msg),
                    }
                } else {
                    println!("Bot's move: No valid move available");
                }
            }
            else {
                println!("{}, (1-7):", self.current_player);

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("failed to read input");

                let col: usize = match input.trim().parse(){
                    Ok(col) => col,
                    Err(_) => {
                        println!("thats not a number dumbass");
                        continue;
                    }
                };

                match self.board.make_move(col - 1, self.current_player) {
                    Ok(_) => {
                        if self.board.check_winner() {
                            println!("{}", self.board);
                            println!("{} wins", self.current_player);
                            self.game_over = true;
                        } else {
                            self.current_player.switch();
                        }
                    }
                    Err(msg) => eprintln!("{}", msg),
                }
            }
        }
    }
}

fn main() {
    let mut connect4 = Connect4::<SimpleBoard>::new();
    connect4.play()
}
