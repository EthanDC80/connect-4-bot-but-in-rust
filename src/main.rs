use std::io;

const ROWS: usize = 6;
const COLS: usize = 7;

struct Connect4 {
    board: [[char; COLS]; ROWS],
    current_player: char,
    game_over: bool,
}

impl Connect4 {
    fn new() -> Connect4 {
        Connect4 {
            board: [['□'; COLS]; ROWS],
            current_player: 'X',
            game_over: false,
        }
    }

    fn print_board(&self) {
        for row in &self.board {
            for cell in row {
                print!("{} ", cell);
            }
            println!();
        }
        println!();
    }

    fn make_move(&mut self, col: usize) -> Result<(), &str> {
        if col >= COLS {
            return Err("invalid column");
        }

        if self.board[0][col] != '□' {
            return Err("full column");
        }

        for row in (0..ROWS).rev() {
            if self.board[row][col] == '□' {
                self.board[row][col] = self.current_player;
                break;
            }
        }

        Ok(())
    }

    fn check_winner(&self) -> bool {
        for row in 0..ROWS {
            for col in 0..COLS - 3 {
                let token = self.board[row][col];
                if token != '□' && self.board[row][col + 1] == token && self.board[row][col + 2] == token && self.board[row][col + 3] == token {
                    return true;
                }
            }
        }
    
        for row in 0..ROWS - 3 {
            for col in 0..COLS {
                let token = self.board[row][col];
                if token != '□' && self.board[row + 1][col] == token && self.board[row + 2][col] == token && self.board[row + 3][col] == token {
                    return true;
                }
            }
        }
    
        for row in 0..ROWS - 3 {
            for col in 0..COLS - 3 {
                let token = self.board[row][col];
                if token != '□' && self.board[row + 1][col + 1] == token && self.board[row + 2][col + 2] == token && self.board[row + 3][col + 3] == token {
                    return true;
                }
            }
        }

        for row in 0..ROWS - 3 {
            for col in 3..COLS {
                let token = self.board[row][col];
                if token != '□' && self.board[row + 1][col - 1] == token && self.board[row + 2][col - 2] == token && self.board[row + 3][col - 3] == token {
                    return true;
                }
            }
        }
    
        false
    }
    

    fn switch_player(&mut self) {
        if self.current_player == 'X' {
            self.current_player = 'O';
        }
        else {
            self.current_player = 'X';
        }
    }

    fn play(&mut self) {
        while !self.game_over {
            self.print_board();
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

            match self.make_move(col) {
                Ok(_) => {
                    if self.check_winner() {
                        self.print_board();
                        println!("{} wins", self.current_player);
                        self.game_over = true;
                    }
                    else {
                        self.switch_player();
                    }
                }
                Err(msg) => println!("{}", msg),
            }
        }
    }
}

fn main() {
    let mut connect4 = Connect4::new();
    connect4.play();
}
