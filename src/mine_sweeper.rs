use js_sys;
use wasm_bindgen::prelude::*;

use crate::battlefield::{BattleField, Cell, CellId, CellStatus, CellType};

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
    pub status: CellStatus,
}

#[wasm_bindgen]
impl MineSweeperEngine {
    /// Creates the engine and matrix battlefield by providing
    ///  rows and columns
    pub fn create(rows: Option<u16>, cols: Option<u16>) -> Self {
        let battle_field_rows = rows.unwrap_or(10);
        let battle_field_cols = cols.unwrap_or(10);
        let bombs = 5;

        let battlefield = BattleField::new(
            battle_field_rows as usize,
            battle_field_cols as usize,
            bombs,
        );

        Self { battlefield }
    }

    /// Uncover and mutate the cell by providing id
    pub fn uncover(&mut self, cell_id: CellId) -> js_sys::Array {
        let cell = self.battlefield.get_mut(cell_id);

        match cell.ctype {
            CellType::Mine => {
                cell.status = CellStatus::Uncovered;
            }
            CellType::Empty(_count) => {
                cell.status = CellStatus::Uncovered;
            }
        }

        // Returns a vector of changed cells
        vec![cell.clone()]
            .into_iter()
            .map(|ref cell| self.convert_cell_into_wasm(cell))
            .collect()
    }

    /// Returns map to the client
    #[wasm_bindgen(js_name = getField)]
    pub fn get_field(&self) -> js_sys::Array {
        self.battlefield
            .get_all()
            .into_iter()
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
            status: cell.status,
            ctype: WasmCType {
                name: match cell.ctype {
                    CellType::Mine => WasmCTypeName::Mine,
                    CellType::Empty(_) => WasmCTypeName::Empty,
                },
                value: 0,
            },
        };

        JsValue::from(wasm_cell)
    }
}
