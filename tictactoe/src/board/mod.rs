mod cell;
mod player;

use cell::Cell;
pub use player::Player;

use core::fmt;

pub struct Board {
    cells: [Cell; 9],
}

impl Default for Board {
    fn default() -> Self {
        Board { cells: [Cell(None); 9] }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..3 {
            for column in 0..3 {
                let index: usize = row*3 + column;
                if self.cells[index].is_empty() {
                    write!(f, "{}", index+1)?;
                } else {
                    write!(f, "{}", self.cells[index])?;
                }

                if column != 2 { write!(f, " | ")?; }
            }
            if row != 2 { writeln!(f, "\n---------")?; }
        }

        Ok(())
    }
}

impl Board {
    /// Returns a boolean indicating success or failure
    pub fn put(&mut self, index: usize, player: Player) -> bool {
        if index > 8 { return false; }
        if !self.cells[index].is_empty() { return false; }
        self.cells[index].put(player);
        return true;
    }

    pub fn has_winner(&self) -> Option<Player> {
        for (f1, f2, f3) in [(0, 1, 2), (3, 4, 5), (6, 7, 8), (0, 3, 6), (1, 4, 7), (2, 5, 8), (0, 4, 8), (2, 4, 6)] {
            if self.cells[f1] == self.cells[f2] && self.cells[f2] == self.cells[f3] {
                if self.cells[f1].is_empty() {
                    continue;
                }
                return self.cells[f1].get();
            }
        }
        None
    }

    pub fn has_space(&self) -> bool {
        for cell in self.cells {
            if cell.is_empty() {return true;}
        }
        false
    }
}
