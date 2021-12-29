use wasm_bindgen::prelude::*;

use crate::engine::{BattleField, Cell, CellId, CellState, CellType};
use crate::wasm::wasm_types::*;

#[wasm_bindgen]
/// The main Minesweeper engine which contain
///  - rows
///  - cols
pub struct MineSweeperEngine {
    battlefield: BattleField,
}

#[wasm_bindgen]
impl MineSweeperEngine {
    /// Creates the engine and matrix battlefield by providing
    ///  rows and columns
    pub fn create(rows: u16, cols: u16, bombs: u16) -> Self {
        let battlefield = BattleField::new(rows as usize, cols as usize, bombs);

        Self { battlefield }
    }

    /// Reveals the cell by providing id
    pub fn reveal(&mut self, cell_id: CellId) -> js_sys::Array {
        let revealed = self.battlefield.reveal(cell_id);

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
            status: match cell.state {
                CellState::Hidden => WasmCellState::Hidden,
                CellState::Revealed => WasmCellState::Revealed,
            },
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