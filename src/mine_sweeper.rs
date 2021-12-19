use wasm_bindgen::prelude::*;

use crate::battlefield::{BattleField, Cell, CellId, CellState, CellType};

#[wasm_bindgen]
/// The main Minesweeper engine which contain
///  - rows
///  - cols
pub struct MineSweeperEngine {
    battlefield: BattleField,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum WasmCTypeName {
    Mine,
    Empty,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct WasmCType {
    pub name: WasmCTypeName,
    pub value: u8,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct WasmCell {
    pub id: CellId,
    pub ctype: WasmCType,
    pub status: CellState,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct WasmPosition {
    pub x: i16,
    pub y: i16,
}

#[wasm_bindgen]
impl MineSweeperEngine {
    /// Creates the engine and matrix battlefield by providing
    ///  rows and columns
    pub fn create(rows: u16, cols: u16, bombs: u16) -> Self {
        let battlefield = BattleField::new(rows as usize, cols as usize, bombs);

        Self { battlefield }
    }

    /// Reveal and mutate the cell by providing id
    pub fn reveal(&mut self, cell_id: CellId) -> js_sys::Array {
        let revealed = self.battlefield.reveal(cell_id);
        // cell.state = CellState::Revealed;
        //
        // match cell.ctype {
        //     CellType::Mine => {}
        //     CellType::Empty(0) => {
        //         // Flood fill algorithm which have to uncover all near cells
        //     }
        //     CellType::Empty(_) => {}
        // }

        // Returns a vector of changed cells
        revealed
            .into_iter()
            .map(|ref cell| self.convert_cell_into_wasm(cell))
            .collect()
    }

    /// Returns map to the client
    #[wasm_bindgen(js_name = getField)]
    pub fn get_field(&self) -> js_sys::Array {
        self.battlefield
            .get_all()
            .iter()
            .map(|cell_vec| {
                cell_vec
                    .clone()
                    .into_iter()
                    .map(|ref cell| self.convert_cell_into_wasm(cell))
                    .collect::<js_sys::Array>()
            })
            .collect()
    }

    /// Converts Battlefield Cell into WasmCell structure
    fn convert_cell_into_wasm(&self, cell: &Cell) -> JsValue {
        let wasm_cell = WasmCell {
            id: cell.id,
            status: cell.state,
            ctype: WasmCType {
                name: match cell.ctype {
                    CellType::Mine => WasmCTypeName::Mine,
                    CellType::Empty(_) => WasmCTypeName::Empty,
                },
                value: match cell.ctype {
                    CellType::Mine => 0,
                    CellType::Empty(value) => value,
                },
            },
        };

        JsValue::from(wasm_cell)
    }
}
