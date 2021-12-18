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
    x: i16,
    y: i16,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellStatus {
    /// Default cell status
    Hidden,

    /// Cell was uncovered
    Uncovered,
}

/// Cell represent each tile on the board
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cell {
    /// Cell identificator
    pub id: CellId,

    /// Cell type
    pub ctype: CellType,

    pub status: CellStatus,

    position: Position,
}

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
        let mut battlefield_map = Vec::with_capacity(rows);
        let mut unique_id = 0;
        let mut rng = rand::thread_rng();

        let mut bombs_map = Vec::<Position>::with_capacity(bombs as usize);
        for _ in 0..bombs {
            let random_bomb_row = rng.gen_range(0..rows);
            let random_bomb_col = rng.gen_range(0..cols);

            bombs_map.push(Position {
                x: random_bomb_col as i16,
                y: random_bomb_row as i16,
            });
        }

        for row_index in 0..rows {
            battlefield_map.push(Vec::with_capacity(cols));

            for col_index in 0..cols {
                let mut ctype = CellType::Empty(0);
                for bomb_position in bombs_map.iter() {
                    if bomb_position.y == row_index as i16 && bomb_position.x == col_index as i16 {
                        ctype = CellType::Mine;
                    }
                }

                battlefield_map[row_index].push(Cell {
                    id: unique_id,
                    ctype,
                    status: CellStatus::Hidden,
                    position: Position {
                        x: col_index as i16,
                        y: row_index as i16,
                    },
                });

                unique_id += 1;
            }
        }

        for bomb_position in bombs_map.iter() {
            let mut nearby_cells =
                Self::get_matrix_of_nearby_cells(&battlefield_map, bomb_position);

            for cell in nearby_cells.iter_mut() {
                let new_type = match cell.ctype {
                    CellType::Mine => CellType::Mine,
                    CellType::Empty(count) => CellType::Empty(count + 1),
                };
                cell.ctype = new_type;
            }
        }

        Self {
            map: battlefield_map,
        }
    }

    /// Find and returns `Cell` by `id` and returns matrix
    ///  of nearby elements from top-left to bottom-right
    fn get_matrix_of_nearby_cells<'a>(
        map: &'a BattlefieldMap,
        cell_position: &'a Position,
    ) -> Vec<&'a mut Cell> {
        let mut result = Vec::<&mut Cell>::new();
        for row in -1..1 {
            for col in -1..1 {
                let cell = Self::get_by_position(
                    map,
                    Position {
                        x: cell_position.x + row,
                        y: cell_position.y + col,
                    },
                );

                if let Some(mut c) = cell {
                    result.push(c);
                }
            }
        }

        result

        // Calculate top cells
        // let top_left_cell = Self::get_by_position(
        //     map,
        //     Position {
        //         x: cell_position.x - 1,
        //         y: cell_position.y - 1,
        //     },
        // );
        // let top_middle_cell = Self::get_by_position(
        //     map,
        //     Position {
        //         x: cell_position.x,
        //         y: cell_position.y - 1,
        //     },
        // );
        // let top_right_cell = Self::get_by_position(
        //     map,
        //     Position {
        //         x: cell_position.x + 1,
        //         y: cell_position.y - 1,
        //     },
        // );
        //
        // // Calculate center cells
        // let center_left_cell = Self::get_by_position(
        //     map,
        //     Position {
        //         x: cell_position.x - 1,
        //         y: cell_position.y,
        //     },
        // );
        // let center_right_cell = Self::get_by_position(
        //     map,
        //     Position {
        //         x: cell_position.x + 1,
        //         y: cell_position.y,
        //     },
        // );
        //
        // // Calculate bottom cells
        // let bottom_left_cell = Self::get_by_position(
        //     map,
        //     Position {
        //         x: cell_position.x - 1,
        //         y: cell_position.y + 1,
        //     },
        // );
        // let bottom_middle_cell = Self::get_by_position(
        //     map,
        //     Position {
        //         x: cell_position.x,
        //         y: cell_position.y + 1,
        //     },
        // );
        // let bottom_right_cell = Self::get_by_position(
        //     map,
        //     Position {
        //         x: cell_position.x + 1,
        //         y: cell_position.y + 1,
        //     },
        // );
        //
        // vec![
        //     top_left_cell.unwrap(),
        //     top_middle_cell.unwrap(),
        //     top_right_cell.unwrap(),
        //     center_left_cell.unwrap(),
        //     center_right_cell.unwrap(),
        //     bottom_left_cell.unwrap(),
        //     bottom_middle_cell.unwrap(),
        //     bottom_right_cell.unwrap(),
        // ]
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
    fn get_by_position(map: &BattlefieldMap, position: Position) -> Option<&Cell> {
        if let Some(item) = map.get(position.y as usize) {
            if let Some(val) = item.get(position.x as usize) {
                Some(val)
            } else {
                None
            }
        } else {
            None
        }
        // let cell = map[position.y][position.x];

        // cell
    }

    /// Returns all matrix map
    pub fn get_all(&self) -> &Vec<Vec<Cell>> {
        &self.map
    }
}

#[cfg(test)]
mod battlefield_test {
    use crate::battlefield::{BattleField, Cell, CellStatus, CellType, Position};

    #[test]
    fn should_return_cell_by_specify_position() {
        let battlefield = BattleField::new(10, 10, 0);
        let cell = BattleField::get_by_position(&battlefield.map, Position { x: 5, y: 5 });

        if let Some(c) = cell {
            println!("Should be here!");
            assert_eq!(
                *c,
                Cell {
                    id: 55,
                    position: Position { x: 5, y: 5 },
                    status: CellStatus::Hidden,
                    ctype: CellType::Empty(0)
                }
            )
        }
    }

    #[test]
    fn should_not_return_cell_by_specify_position_because_it_is_out_of_bounce() {
        let battlefield = BattleField::new(10, 10, 0);
        let cell = BattleField::get_by_position(&battlefield.map, Position { x: -1, y: -1 });

        if let Some(_) = cell {
            panic!("Cell mustn't be Some in that particular scenario");
        } else {
            assert_eq!(cell.is_none(), true);
        }
    }
}
