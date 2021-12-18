use rand::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::console;

pub type CellId = u32;

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
struct Position {
    pub x: i16,
    pub y: i16,
}

#[wasm_bindgen]
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
    position: Position,
}

/// Battlefield map represents the field
///  when the first vector is a `x` axis or `cols`
///  and the second vector is a `y` axis or `rows`
type BattlefieldMap = Vec<Vec<Cell>>;

/// The main map of the battle
pub struct BattleField {
    /// Current map
    map: BattlefieldMap,
}

impl BattleField {
    /// Creates an empty battlefield map with
    ///  no mines and without any text messages
    ///
    /// Notes
    ///  But it should place bombs and text messages recording to the bombs
    pub fn new(rows: usize, cols: usize, bombs: u16) -> Self {
        let mut battlefield_map = Vec::with_capacity(cols);
        let mut unique_id = 0;
        let mut rng = rand::thread_rng();

        let mut bombs_map = Vec::<Position>::with_capacity(bombs as usize);
        for _ in 0..bombs {
            let random_bomb_col = rng.gen_range(0..cols);
            let random_bomb_row = rng.gen_range(0..rows);

            bombs_map.push(Position {
                x: random_bomb_col as i16,
                y: random_bomb_row as i16,
            });
        }

        for col_index in 0..cols {
            battlefield_map.push(Vec::with_capacity(rows));

            for row_index in 0..rows {
                let mut ctype = CellType::Empty(0);
                for bomb_position in bombs_map.iter() {
                    if bomb_position.y == row_index as i16 && bomb_position.x == col_index as i16 {
                        ctype = CellType::Mine;
                    }
                }

                battlefield_map[col_index].push(Cell {
                    id: unique_id,
                    ctype,
                    state: CellState::Hidden,
                    position: Position {
                        x: col_index as i16,
                        y: row_index as i16,
                    },
                });

                unique_id += 1;
            }
        }

        for bomb_position in bombs_map.iter() {
            Self::update_empty_cells_count(&mut battlefield_map, bomb_position);
        }

        Self {
            map: battlefield_map,
        }
    }

    /// Find and returns Vec of `Cells` by `position` and returns flood_fill
    ///  of nearby elements from top-left to bottom-right
    fn update_empty_cells_count<'a>(map: &'a mut BattlefieldMap, cell_position: &'a Position) {
        for col in -1..2 {
            for row in -1..2 {
                let option_cell = Self::get_by_position(
                    map,
                    Position {
                        x: cell_position.x + col,
                        y: cell_position.y + row,
                    },
                );

                if let Some(cell) = option_cell {
                    let new_type = match cell.ctype {
                        CellType::Mine => CellType::Mine,
                        CellType::Empty(count) => CellType::Empty(count + 1),
                    };

                    cell.ctype = new_type;
                }
            }
        }
    }

    /// Returns a mutable link to the cell by provided `id`
    pub fn get_mut(&mut self, id: CellId) -> &mut Cell {
        for row in &mut self.map {
            for col in row {
                if col.id == id {
                    return col;
                }
            }
        }

        panic!("Cell didn't find in battlefield by provided id: {}", id);
    }

    /// Returns a link to the cell by provided `id`
    fn get_by_id(map: &BattlefieldMap, id: CellId) -> &Cell {
        for row in map {
            for col in row {
                if col.id == id {
                    return col;
                }
            }
        }

        panic!("Cell didn't find in battlefield by provided id: {}", id);
    }

    /// Returns a link to the cell by provided `id`
    fn get_by_position(map: &mut BattlefieldMap, position: Position) -> Option<&mut Cell> {
        if let Some(item) = map.get_mut(position.x as usize) {
            if let Some(mut val) = item.get_mut(position.y as usize) {
                Some(val)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns all matrix map
    pub fn get_all(&self) -> &Vec<Vec<Cell>> {
        &self.map
    }
}

#[cfg(test)]
mod battlefield_test {
    use crate::battlefield::{BattleField, Cell, CellState, CellType, Position};

    #[test]
    fn should_return_cell_by_specify_position() {
        let mut battlefield = BattleField::new(10, 10, 0);
        let cell = BattleField::get_by_position(&mut battlefield.map, Position { x: 5, y: 5 });

        if let Some(c) = cell {
            println!("Should be here!");
            assert_eq!(
                *c,
                Cell {
                    id: 55,
                    position: Position { x: 5, y: 5 },
                    state: CellState::Hidden,
                    ctype: CellType::Empty(0)
                }
            )
        }
    }

    #[test]
    fn should_not_return_cell_by_specify_position_because_it_is_out_of_bounce() {
        let mut battlefield = BattleField::new(10, 10, 0);
        let cell = BattleField::get_by_position(&mut battlefield.map, Position { x: -1, y: -1 });

        if let Some(_) = cell {
            panic!("Cell mustn't be Some in that particular scenario");
        } else {
            assert_eq!(cell.is_none(), true);
        }
    }
}
