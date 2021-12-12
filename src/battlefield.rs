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
}

/// The main map of the battle
pub struct BattleField {
    /// Current map
    map: Vec<Vec<Cell>>,
}

impl BattleField {
    pub fn new(rows: usize, cols: usize, _bombs: u16) -> Self {
        let mut map = Vec::with_capacity(rows);
        let mut unique_id = 0;
        let mut rng = rand::thread_rng();

        let random_bomb_row = rng.gen_range(0..rows);
        let random_bomb_col = rng.gen_range(0..cols);

        for i in 0..rows {
            map.push(Vec::with_capacity(cols));

            for j in 0..cols {
                // Place the bomb in random place
                let ctype = if i == random_bomb_row && j == random_bomb_col {
                    CellType::Mine
                } else {
                    // Create random value for each cell
                    CellType::Empty(rng.gen_range(0..8))
                };

                map[i].push(Cell {
                    id: unique_id,
                    ctype,
                    status: CellStatus::Hidden,
                });

                unique_id += 1;
            }
        }

        Self { map }
    }

    /// Returns a link to the cell by provided `id`
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

    /// Returns all matrix map
    pub fn get_all(&self) -> &Vec<Vec<Cell>> {
        &self.map
    }
}
