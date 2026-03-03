use super::player::Player;

use core::fmt;

#[derive(Clone, Copy, PartialEq)]
pub(super) struct Cell (pub Option<Player>);

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(player) => write!(f, "{}", player),
            None => write!(f, " "),
        }
    }
}

impl Cell {
    pub(super) fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub(super) fn put(&mut self, player: Player) {
        self.0 = Some(player);
    }

    pub(super) fn get(&self) -> Option<Player> {
        self.0
    }
}