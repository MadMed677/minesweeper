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
                });

                unique_id += 1;
            }
        }

        Self {
            map: battlefield_map,
        }
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
