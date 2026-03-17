use super::player::Player;

use core::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Cell (pub Option<super::Player>);

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(player) => write!(f, "{}", player),
            None => write!(f, " "),
        }
    }
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn put(&mut self, player: Player) {
        self.0 = Some(player);
    }

    pub fn get(&self) -> Option<Player> {
        self.0
    }
}