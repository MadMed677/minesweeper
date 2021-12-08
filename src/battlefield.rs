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
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Cell {
    /// Cell identificator
    id: CellId,

    #[wasm_bindgen(skip)]
    /// Cell type
    pub ctype: CellType,

    pub status: CellStatus,
}

#[wasm_bindgen]
impl Cell {
    #[wasm_bindgen(js_name = getId)]
    pub fn id(&self) -> CellId {
        self.id
    }
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

        for i in 0..rows {
            map.push(Vec::with_capacity(cols));

            for j in 0..cols {
                // Place the bomb only on the first element
                let ctype = if i == 0 && j == 0 {
                    CellType::Mine
                } else {
                    CellType::Empty(0)
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
