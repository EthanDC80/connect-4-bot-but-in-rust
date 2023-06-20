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
pub(crate) trait Board<E>: Display + Copy + Clone {
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
        } else if self.get(0, col)?.is_none() {
            // Full Column
            return Ok(false)
        }

        // Procedural implementation:
        for slot in self.get_column(col)? {
            if slot.is_none() {
                return Ok(true)
            }
        }
        Ok(false)

        // Iterator implementation:
        // Ok(
        //  self.get_column(col)?
        //      .iter()
        //      .any(|tile| tile.is_none())
        // )
    }

    fn make_move(&mut self, col: usize, player: Player) -> Result<(), Error> {
        for slot in self.get_column_mut(col)? {
            if slot.is_none() {
                *slot = Some(player);
                return Ok(())
            }
        }
        Err(Error::FullColumn)
    }

    fn check_winner(&self) -> bool {
        let mut streak = 0;
        let mut prev_token = None;
        let mut token;
        
        for row in 0..ROWS {
            for col in 0..COLS - 3 {
                token = self.get(row, col).expect("Invalid board access");
                if token.is_some() && token == prev_token { streak += 1 } 
                else { streak = 0 }
                if streak == 3 { return true }
                prev_token = token
            }
        }

        todo!("Implement other checks");
        #[allow(unreachable_code)]
        false
    }
}

impl Display for SimpleBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Example:
        // 
        // let board = Board::new()
        // println!("{}", board)
        write!(f, "Implement this however you would like, this is what gets called when you do println!() on a board")
    }
}
