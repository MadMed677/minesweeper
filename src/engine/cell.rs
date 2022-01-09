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

    /// Mark the cell as a flag if it's not
    ///  or unmark it as a flag if it's flagged
    pub fn flag(&mut self) {
        if self.state == CellState::Flagged {
            self.state = CellState::Hidden;
        } else {
            self.state = CellState::Flagged;
        }
    }
}

#[cfg(test)]
mod battlefield_cell {
    use crate::engine::{Cell, CellPosition, CellState, CellType};

    #[test]
    fn should_create_cell_via_constructor() {
        let cell = Cell::new(0, CellType::Empty(0), CellPosition { x: 0, y: 0 });

        assert_eq!(
            cell,
            Cell {
                id: 0,
                state: CellState::Hidden,
                ctype: CellType::Empty(0),
                position: CellPosition { x: 0, y: 0 },
            }
        );
    }

    #[test]
    fn should_reveal_the_cell() {
        let mut cell = Cell {
            id: 0,
            state: CellState::Hidden,
            ctype: CellType::Empty(0),
            position: CellPosition { x: 0, y: 0 },
        };

        cell.reveal();

        assert_eq!(
            cell,
            Cell {
                id: 0,
                state: CellState::Revealed,
                ctype: CellType::Empty(0),
                position: CellPosition { x: 0, y: 0 },
            }
        );
    }

    #[test]
    fn should_flag_the_cell() {
        let mut cell = Cell {
            id: 0,
            state: CellState::Hidden,
            ctype: CellType::Empty(0),
            position: CellPosition { x: 0, y: 0 },
        };

        cell.flag();

        assert_eq!(
            cell,
            Cell {
                id: 0,
                state: CellState::Flagged,
                ctype: CellType::Empty(0),
                position: CellPosition { x: 0, y: 0 },
            }
        );
    }

    #[test]
    fn should_unflag_the_cell() {
        let mut cell = Cell {
            id: 0,
            state: CellState::Hidden,
            ctype: CellType::Empty(0),
            position: CellPosition { x: 0, y: 0 },
        };

        // Should flag the cell
        cell.flag();

        // Should unflag the cell
        cell.flag();

        assert_eq!(
            cell,
            Cell {
                id: 0,
                state: CellState::Hidden,
                ctype: CellType::Empty(0),
                position: CellPosition { x: 0, y: 0 },
            }
        );
    }
}
