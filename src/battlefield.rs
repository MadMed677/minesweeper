use wasm_bindgen::prelude::*;

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

/// Cell represent each tile on the board
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Cell {
    /// Cell index
    id: u32,

    /// Cell type
    ctype: CellType,
}

#[wasm_bindgen]
impl Cell {
    #[wasm_bindgen(js_name = getId)]
    pub fn id(&self) -> u32 {
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
                map[i].push(Cell {
                    id: unique_id,
                    ctype: CellType::Empty(0),
                });

                unique_id += 1;
            }
        }

        Self { map }
    }

    /// Returns all matrix map
    pub fn get_all(&self) -> &Vec<Vec<Cell>> {
        &self.map
    }
}
