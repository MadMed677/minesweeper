use wasm_bindgen::prelude::*;

use crate::engine::{BattleField, Cell, CellId, CellState, CellType};
use crate::wasm::wasm_types::*;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct GameState {
    pub status: GameStatus,
    pub flags: u16,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum GameStatus {
    /// Signals to the player that the game is going
    Played,

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

    /// A public subsciption that we have to trigger
    ///  if the client want to know if game state
    ///  has been changed
    on_change: Option<js_sys::Function>,
}

#[wasm_bindgen]
impl MineSweeperEngine {
    /// Creates the engine and matrix battlefield by providing
    ///  rows and columns
    pub fn create(rows: u16, cols: u16, bombs: u16) -> Self {
        let battlefield = BattleField::new(rows as usize, cols as usize, bombs);
        let flgs = battlefield.flags_left;

        Self {
            battlefield,
            game_state: GameState {
                status: GameStatus::Played,
                flags: flgs,
            },
            elements_to_win_the_game: (rows * cols - bombs) as i16,
            revealed_elements: 0,
            on_change: None,
        }
    }

    /// Reveals the cell by providing id
    pub fn reveal(&mut self, cell_id: CellId) -> js_sys::Array {
        let reveal = self.battlefield.reveal(cell_id);

        // Add into `revealed_elements` all new revealed cells count
        self.revealed_elements += reveal.cells.len() as i16;

        // Returns a vector of changed cells
        let cells = reveal
            .cells
            .into_iter()
            .map(|ref cell| self.convert_cell_into_wasm(cell))
            .collect();

        // Updates `game_is_over` flag to set the actual game state
        if reveal.game_is_over {
            self.game_state.status = GameStatus::Lose;
        } else if self.elements_to_win_the_game - self.revealed_elements == 0 {
            self.game_state.status = GameStatus::Won;
        }

        self.game_state.flags = self.battlefield.flags_left;
        self.on_game_changed(&self.game_state);

        cells
    }

    pub fn flag(&mut self, cell_id: CellId) -> JsValue {
        let cell = self.battlefield.flag(cell_id);
        let cp_cell = *cell;

        self.game_state.flags = self.battlefield.flags_left;
        self.on_game_changed(&self.game_state);

        self.convert_cell_into_wasm(&cp_cell)
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

    /// Method that can be subscribed to in the Public API
    ///  that signals that the state of the application
    ///  has been changed
    ///
    /// The solution was based on the [StackOverflow example](https://stackoverflow.com/a/53685611)
    #[wasm_bindgen(js_name = onChange)]
    pub fn on_change(&mut self, callback: js_sys::Function) {
        self.on_change = Some(callback);
    }

    /// Fires when game was changed
    /// Internal method which called public `on_change` event
    fn on_game_changed(&self, game_state: &GameState) {
        if let Some(callback) = &self.on_change {
            let this = JsValue::null();
            let wasm_data = JsValue::from(*game_state);

            let _ = callback.call1(&this, &wasm_data);
        }
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
