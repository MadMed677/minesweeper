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

/// The ordinary `Cell` but only for Web Assembly
///
/// Current implementation uses id as an integer. But if we
///  would like to make them String we have to:
///  - Implement all fields as `getter` and in the struct
///     we are not published the fields with `pub` keyword
///
/// We have to do this because wasm have to derive `copy` and `clone` traits
///  when we expose the public fields. Because `String` [couldn't be copy](https://doc.rust-lang.org/std/marker/trait.Copy.html)
///  we have to explicitly say that the structure themselves has no public fields
///  and provide via `getters` all these fields
///
/// - [Why we couldn't use `String` as public in structures for Wasm](https://github.com/rustwasm/wasm-bindgen/issues/1775#issuecomment-533761425)
/// - [Getters and Setters](https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/getter-and-setter.html)
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
    Flagged,
}
