use rand::prelude::*;
use wasm_bindgen::prelude::*;

pub type CellId = u32;

/// Should contain 2 structure
///  1. Factory - to create Minesweeper engine
///  2. Engine which contain game state and provides
///      some methods to uncover the cells

#[derive(Copy, Clone)]
pub enum CellType {
    /// Cell is a mine
    Mine,

    /// Cell is empty but it may be next to the bomb
    ///  if it's not it would contain `0`
    Empty(u8),
}

#[derive(Copy, Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum CellStatus {
    /// Default cell status
    Hidden,

    /// Cell was uncovered
    Uncovered,
}

/// Cell represent each tile on the board
#[derive(Copy, Clone)]
pub struct Cell {
    /// Cell identificator
    pub id: CellId,

    /// Cell type
    pub ctype: CellType,

    pub status: CellStatus,

    position: Position,
}

/// The main map of the battle
pub struct BattleField {
    /// Current map
    map: Vec<Vec<Cell>>,
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

        let mut random_bombs = Vec::<Position>::with_capacity(bombs as usize);
        for _ in 0..bombs {
            let random_bomb_row = rng.gen_range(0..rows);
            let random_bomb_col = rng.gen_range(0..cols);

            random_bombs.push(Position {
                x: random_bomb_col,
                y: random_bomb_row,
            });
        }

        for row_index in 0..rows {
            battlefield_map.push(Vec::with_capacity(cols));

            for col_index in 0..cols {
                let mut ctype = CellType::Empty(rng.gen_range(0..8));
                for bomb_position in random_bombs.iter() {
                    if bomb_position.y == row_index && bomb_position.x == col_index {
                        ctype = CellType::Mine;
                    }
                }

                battlefield_map[row_index].push(Cell {
                    id: unique_id,
                    ctype,
                    status: CellStatus::Hidden,
                    position: Position {
                        x: col_index,
                        y: row_index,
                    },
                });

                unique_id += 1;
            }
        }

        Self {
            map: battlefield_map,
        }
    }

    /// Find and returns `Cell` by `id` and returns matrix
    ///  of nearby elements from top-left to bottom-right
    fn get_matrix_of_nearby_cells(&self, id: CellId) -> Vec<&Cell> {
        let cell = self.get_by_id(id);

        // Calculate top cells
        let top_left_cell = self.get_by_position(Position {
            x: cell.position.x - 1,
            y: cell.position.y - 1,
        });
        let top_middle_cell = self.get_by_position(Position {
            x: cell.position.x,
            y: cell.position.y - 1,
        });
        let top_right_cell = self.get_by_position(Position {
            x: cell.position.x + 1,
            y: cell.position.y - 1,
        });

        // Calculate center cells
        let center_left_cell = self.get_by_position(Position {
            x: cell.position.x - 1,
            y: cell.position.y,
        });
        let center_right_cell = self.get_by_position(Position {
            x: cell.position.x + 1,
            y: cell.position.y,
        });

        // Calculate bottom cells
        let bottom_left_cell = self.get_by_position(Position {
            x: cell.position.x - 1,
            y: cell.position.y + 1,
        });
        let bottom_middle_cell = self.get_by_position(Position {
            x: cell.position.x,
            y: cell.position.y + 1,
        });
        let bottom_right_cell = self.get_by_position(Position {
            x: cell.position.x + 1,
            y: cell.position.y + 1,
        });

        vec![
            top_left_cell,
            top_middle_cell,
            top_right_cell,
            center_left_cell,
            center_right_cell,
            bottom_left_cell,
            bottom_middle_cell,
            bottom_right_cell,
        ]
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
    fn get_by_id(&self, id: CellId) -> &Cell {
        for row in &self.map {
            for col in row {
                if col.id == id {
                    return col;
                }
            }
        }

        panic!("Cell didn't find in battlefield by provided id: {}", id);
    }

    /// Returns a link to the cell by provided `id`
    fn get_by_position(&self, position: Position) -> &Cell {
        for row in &self.map {
            for cell in row {
                if cell.position.x == position.x && cell.position.y == position.y {
                    return cell;
                }
            }
        }

        panic!(
            "Cell didn't find in battlefield by provided position: {:?}",
            position
        );
    }

    /// Returns all matrix map
    pub fn get_all(&self) -> &Vec<Vec<Cell>> {
        &self.map
    }
}
