use core::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn switch(player: &mut Player) {
        *player = match player {
            Player::O => Player::X,
            Player::X => Player::O
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}
