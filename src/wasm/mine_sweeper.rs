use wasm_bindgen::prelude::*;

use crate::engine::{BattleField, Cell, CellId, CellState, CellType};
use crate::wasm::wasm_types::*;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum GameState {
    /// Signals to the player that the game is going
    Play,

    /// Signals to the player that the game has ended and the user is lost
    Lose,

    /// Signals to the player that the game has ended and the user is won
    Won,
}

#[wasm_bindgen]
/// The main Minesweeper engine which contain
///  - rows
///  - cols
pub struct MineSweeperEngine {
    /// The main battlefield of mine sweeper
    battlefield: BattleField,

    /// Returns the game state of the game
    game_state: GameState,

    /// How many cells should we reveal to win the game
    elements_to_win_the_game: i16,

    /// How many cells already revealed
    revealed_elements: i16,
}

#[wasm_bindgen]
impl MineSweeperEngine {
    /// Creates the engine and matrix battlefield by providing
    ///  rows and columns
    pub fn create(rows: u16, cols: u16, bombs: u16) -> Self {
        let battlefield = BattleField::new(rows as usize, cols as usize, bombs);

        Self {
            battlefield,
            game_state: GameState::Play,
            elements_to_win_the_game: (rows * cols - bombs) as i16,
            revealed_elements: 0,
        }
    }

    /// Reveals the cell by providing id
    pub fn reveal(&mut self, cell_id: CellId) -> js_sys::Array {
        let reveal = self.battlefield.reveal(cell_id);

        let mut revealed_elements = 0;
        for col in self.battlefield.map.iter() {
            for cell in col {
                if cell.state == CellState::Revealed {
                    revealed_elements += 1;
                }
            }
        }

        self.revealed_elements = revealed_elements;

        // Returns a vector of changed cells
        let cells = reveal
            .cells
            .into_iter()
            .map(|ref cell| self.convert_cell_into_wasm(cell))
            .collect();

        // Updates `game_is_over` flag to set the actual game state
        if reveal.game_is_over {
            self.game_state = GameState::Lose;
        }

        if self.elements_to_win_the_game - self.revealed_elements == 0 {
            self.game_state = GameState::Won;
        }

        cells
    }

    pub fn flag(&mut self, cell_id: CellId) -> JsValue {
        let cell = self.battlefield.flag(cell_id);

        self.convert_cell_into_wasm(&cell)
    }

    /// Returns a game state of the game
    #[wasm_bindgen(js_name = getGameState)]
    pub fn game_state(&self) -> GameState {
        self.game_state
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
                CellState::Flagged => WasmCellState::Flagged,
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
