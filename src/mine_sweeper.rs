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

    /// We provide `position` only for debugging
    ///  We have to remove it later
    pub position: WasmPosition,
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
                value: match cell.ctype {
                    CellType::Mine => 0,
                    CellType::Empty(value) => value,
                },
            },
            position: WasmPosition {
                x: cell.position.x,
                y: cell.position.y,
            },
        };

        JsValue::from(wasm_cell)
    }
}
