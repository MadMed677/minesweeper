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
pub struct Position {
    pub x: i16,
    pub y: i16,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellState {
    /// Default cell status
    Hidden,

    /// Cell was uncovered
    Revealed,
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
    pub position: Position,
}
