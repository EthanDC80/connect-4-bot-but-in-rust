use crate::player::Player;
use crate::consts::{ROWS, COLS};
use std::fmt::Display;


/// Board Errors
#[derive(Debug)]
pub enum Error {
    InvalidColumn,
    InvalidRow,
    FullColumn
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidColumn => write!(f, "Invalid column"),
            Error::InvalidRow => write!(f, "Invalid row"),
            Error::FullColumn => write!(f, "Full column")
        }
    }
}

impl std::error::Error for Error {}


/// Board properties, useful for multiples implementations of board
/// (Board is something that is often changed a lot during optimization, useful to have a trait)
pub trait Board<E>: Display + Copy + Clone {
    fn new() -> Self;
    fn get(&self, row: usize, col: usize) -> Result<Option<Player>, E>;
    fn get_column(&self, col: usize) -> Result<&[Option<Player>], E>;
    fn get_column_mut(&mut self, col: usize) -> Result<&mut [Option<Player>], E>;
    fn is_valid(&self, col: usize) -> Result<bool, E>;
    fn make_move(&mut self, col: usize, player: Player) -> Result<(), E>;
    // TODO: Add function to unmake move (For minimax)
    fn check_winner(&self) -> bool;
}



/// An actual implementation of a Board
#[derive(Clone, Copy)]
pub struct SimpleBoard {
    board: [[Option<Player>; COLS]; ROWS]
}

impl Board<Error> for SimpleBoard {
    fn new() -> Self {
        Self {
            board: [[None; COLS]; ROWS]
        }
    }

    fn get(&self, row: usize, col: usize) -> Result<Option<Player>, Error> {
        // row and col are signed so we don't need to check for less than 0
        if row >= ROWS {
            return Err(Error::InvalidRow);
        } else if col >= COLS {
            return Err(Error::InvalidColumn);
        }
        Ok(self.board[row][col])
    }

    fn get_column(&self, col: usize) -> Result<&[Option<Player>], Error> {
        if col >= COLS {
            return Err(Error::InvalidColumn);
        }
        Ok(&self.board[..][col])
    }

    fn get_column_mut(&mut self, col: usize) -> Result<&mut [Option<Player>], Error> {
        if col >= COLS {
            return Err(Error::InvalidColumn);
        }
        Ok(&mut self.board[..][col])
    }

    fn is_valid(&self, col: usize) -> Result<bool, Error> {
        if col >= COLS {
            return Err(Error::InvalidColumn);
        }
    
        for row in 0..ROWS {
            if self.get(row, col)? == None {
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    fn make_move(&mut self, col: usize, player: Player) -> Result<(), Error> {
        if col >= COLS {
            return Err(Error::InvalidColumn);
        }

        for row in (0..ROWS).rev() {
            if self.board[row][col].is_none() {
                self.board[row][col] = Some(player);
                return Ok(());
            }
        }
        Err(Error::FullColumn)
    }

    fn check_winner(&self) -> bool {
        // let mut streak = 0;
        // let mut prev_token = None;
        // let mut token;
        
        for row in 0..ROWS {
            for col in 0..COLS - 3 {
                let token = self.board[row][col];
                if let Some(player) = token {
                    if self.board[row][col + 1] == Some(player)
                        && self.board[row][col + 2] == Some(player)
                        && self.board[row][col + 3] == Some(player)
                    {
                        return true;
                    }
                }
            }
        }
    
        // Vertical check
        for col in 0..COLS {
            for row in 0..ROWS - 3 {
                let token = self.board[row][col];
                if let Some(player) = token {
                    if self.board[row + 1][col] == Some(player)
                        && self.board[row + 2][col] == Some(player)
                        && self.board[row + 3][col] == Some(player)
                    {
                        return true;
                    }
                }
            }
        }
    
        // Diagonal check (bottom-left to top-right)
        for row in 0..ROWS - 3 {
            for col in 0..COLS - 3 {
                let token = self.board[row][col];
                if let Some(player) = token {
                    if self.board[row + 1][col + 1] == Some(player)
                        && self.board[row + 2][col + 2] == Some(player)
                        && self.board[row + 3][col + 3] == Some(player)
                    {
                        return true;
                    }
                }
            }
        }
    
        // Diagonal check (top-left to bottom-right)
        for row in 3..ROWS {
            for col in 0..COLS - 3 {
                let token = self.board[row][col];
                if let Some(player) = token {
                    if self.board[row - 1][col + 1] == Some(player)
                        && self.board[row - 2][col + 2] == Some(player)
                        && self.board[row - 3][col + 3] == Some(player)
                    {
                        return true;
                    }
                }
            }
        }
    
        false
    }
}

impl Display for SimpleBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..ROWS {
            for col in 0..COLS {
                match self.get(row, col) {
                    Ok(Some(player)) => write!(f, "{} ", player)?,
                    Ok(None) => write!(f, "□ ")?,
                    Err(_) => write!(f, "ඞ ")?, // Handle error case
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
