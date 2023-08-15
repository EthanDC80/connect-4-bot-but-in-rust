use std::io;

//! Consider abstracting consts into seperate module, becomes more useful as consts increase. 
//! (Completely optional, up to preference)
const ROWS: usize = 6;
const COLS: usize = 7;

struct Connect4 {
    //! Use a struct to represent the board, as game logic grows it will become important to 
    //! perform actions on the board. (e.g. printing, making moves, checking for winners, etc.)
    //! Will make future testing, debugging, refactoring and optimization easier.
    //! This is an example of the Single Responsibility Principle
    board: [[char; COLS]; ROWS],

    //! Use an enum to represent player, there are only 2 possible values for the player
    //! however there are 149,186 possible chars, this presents an easy way to introduce bugs,
    //! using an enum will take full advantage of rusts rich type system to prevent bugs.
    //! Alternatively, you could use a bool, but this is less readable and as functionality grows 
    //! using an enum will become more useful.
    current_player: char,

    //! Use of bool here is very good, however, it is not necessary to initialize it to false.
    //! Consider adding a comment and looking into default values in rust. 
    //! WARNING: Default values can be a mess sometimes and are not always the best solution.
    game_over: bool,
}

//! Good, use of impl block to group methods together.
impl Connect4 {
    
    //! Good, use of associated function to create a new Connect4 struct.
    //! This is the idiomatic way to create a new struct in rust.
    fn new() -> Connect4 {
        //! Sometimes instead of the name of the struct, you will see Self used.
        //! the two are equivalent and at the end of the day it is a matter of preference.
        //! In my experience, binaries tend to use struct name more and libraries use Self more.
        Connect4 {
            //! Use of a board struct would make this more concrete.
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
            //! Use new line characters ("\n"), println!() flushes the stdout buffer, system calls.
            println!();
        }
        println!();
    }

    //! Good, use of Result to handle errors.
    //! Consider using a custom error type with an enum instead of &str, this will make it easier to add more
    //! errors in the future and will make it easier to handle errors in the future.
    //! (Completely optional, up to preference)
    //! Consider adding a comment explaining why you are returning a Result.
    //! Why is this function fallible? What are the possible errors?
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
        //! Iterators and itertools might be useful here but also for nested loops it can get messy quickly.
        //! This implementation is fine but consider using a counter instead of comparing the next 4, this is 
        //! a very expensive computation which will need to be run a lot for minimax and other algorithms.
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
    
        //! A lot of beginners will use return false here, this is not necessary.
        //! The last statement in a function is the return value, so if you reach this point
        //! good use of the fact that bool is the last statement in the function.
        false
    }
    
    //! This function is very error prone, use enum with custom functionality or bool
    //! Char is a decent abstraction in very small examples but is not a good way to 
    //! handle state as projects grow.
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

            //! Unfortunately this is the best way to do user input aside from using 
            //! an external crate.
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read input");

            //! Very good use of methods here, very clean and is the idiomatic way to do this.
            //! Also great use of match statements, very clean and idiomatic.
            let col: usize = match input.trim().parse() {
                Ok(col) => col,
                Err(_) => {
                    //! I like the word choice ;)
                    println!("thats not a number dumbass");
                    continue;
                }
            };

            match self.make_move(col) {
                //! Great use of _, removes an unnececary variable assignment and makes code more readable
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
