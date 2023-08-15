use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Cross,
    Circle,
}

impl Player {
    pub fn switch(&mut self) -> Player {
        // i made it so it returns itself after changing idk if it changes anything
        *self = match self {
            Self::Cross => Self::Circle,
            Self::Circle => Self::Cross,
        };
        *self
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::Cross
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cross => write!(f, "X"),
            Self::Circle => write!(f, "O"),
        }
    }
}

impl From<Player> for char {
    fn from(player: Player) -> Self {
        match player {
            Player::Cross => 'X',
            Player::Circle => 'O',
        }
    }
}

impl From<Player> for bool {
    fn from(player: Player) -> Self {
        match player {
            Player::Cross => true,
            Player::Circle => false,
        }
    }
}
