pub type CellId = u16;

/// Should contain 2 structure
///  1. Factory - to create Minesweeper engine
///  2. Engine which contain game state and provides
///      some methods to uncover the cells

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellType {
    /// Cell is a mine
    Mine,

    /// Cell is empty but it may be next to the bomb
    ///  if it's not it would contain `0`
    Empty(u8),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CellPosition {
    pub x: i16,
    pub y: i16,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellState {
    /// Default cell status
    Hidden,

    /// Cell was uncovered
    Revealed,

    /// Cell was flagged as a potential bomb
    Flagged,
}

/// Cell represent each tile on the board
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cell {
    /// Cell identificator
    pub id: CellId,

    /// Cell type
    pub ctype: CellType,

    pub state: CellState,

    /// Cell position
    ///  `position.x` - represents column
    ///  `position.y` - represents row
    pub position: CellPosition,
}

impl Cell {
    /// Creates new `Cell` by default `Hidden` state
    pub fn new(id: CellId, ctype: CellType, position: CellPosition) -> Self {
        Self {
            id,
            ctype,
            state: CellState::Hidden,
            position,
        }
    }

    /// Reveal the cell
    pub fn reveal(&mut self) {
        self.state = CellState::Revealed;
    }

    /// Mark the cell as a flag if it's not (returns `true`)
    ///  or unmark it as a flag if it's flagged (returns `false`)
    pub fn flag(&mut self) -> bool {
        if self.state == CellState::Flagged {
            self.state = CellState::Hidden;

            false
        } else if self.state == CellState::Hidden {
            self.state = CellState::Flagged;

            true
        } else {
            false
        }
    }
}
