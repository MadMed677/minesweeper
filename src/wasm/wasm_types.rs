use wasm_bindgen::prelude::*;

use crate::engine::CellId;

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
pub struct WasmCell {
    pub id: CellId,
    pub ctype: WasmCType,
    pub status: WasmCellState,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WasmCellState {
    Hidden,
    Revealed,
}
