use js_sys;
use wasm_bindgen::prelude::*;

use crate::battlefield::BattleField;

// pub fn matrix_array(rows: usize, cols: usize) -> Vec<Vec<Cell>> {
//     let mut arr = Vec::with_capacity(rows);
//
//     for i in 0..rows {
//         arr.push(Vec::with_capacity(cols));
//
//         for j in 0..cols {
//             arr[i].push(Cell {
//                 id: (i + j) as u32,
//                 ctype: CellType::Empty(0),
//             });
//         }
//     }
//
//     arr
// }

#[wasm_bindgen]
/// The main Minesweeper engine which contain
///  - rows
///  - cols
pub struct MineSweeperEngine {
    battlefield: BattleField,
}

#[wasm_bindgen]
impl MineSweeperEngine {
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

    pub fn uncover(&self) {}

    /// Returns map to the client
    #[wasm_bindgen(js_name = getField)]
    pub fn get_field(&self) -> js_sys::Array {
        self.battlefield
            .get_all()
            .clone()
            .into_iter()
            .map(|cell_vec| {
                cell_vec
                    .clone()
                    .into_iter()
                    .map(JsValue::from)
                    .collect::<js_sys::Array>()
            })
            .collect()
    }
}
