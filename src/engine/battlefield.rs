use crate::engine::cell::*;
use rand::prelude::*;

/// Battlefield map represents the field
///  when the first vector is a `x` axis or `cols`
///  and the second vector is a `y` axis or `rows`
type BattlefieldMap = Vec<Vec<Cell>>;

/// The main map of the battle
pub struct BattleField {
    /// Current map
    map: BattlefieldMap,
}

impl BattleField {
    /// Creates an empty battlefield map with
    ///  no mines and without any text messages
    ///
    /// Notes
    ///  But it should place bombs and text messages recording to the bombs
    pub fn new(rows: usize, cols: usize, bombs: u16) -> Self {
        let mut battlefield_map = Vec::with_capacity(cols);
        let mut unique_id = 0;
        let mut rng = rand::thread_rng();

        let mut bombs_map = Vec::<Position>::with_capacity(bombs as usize);
        for _ in 0..bombs {
            let random_bomb_col = rng.gen_range(0..cols);
            let random_bomb_row = rng.gen_range(0..rows);

            bombs_map.push(Position {
                x: random_bomb_col as i16,
                y: random_bomb_row as i16,
            });
        }

        for col_index in 0..cols {
            battlefield_map.push(Vec::with_capacity(rows));

            for row_index in 0..rows {
                let mut ctype = CellType::Empty(0);
                for bomb_position in bombs_map.iter() {
                    if bomb_position.y == row_index as i16 && bomb_position.x == col_index as i16 {
                        ctype = CellType::Mine;
                    }
                }

                battlefield_map[col_index].push(Cell {
                    id: unique_id,
                    ctype,
                    state: CellState::Hidden,
                    position: Position {
                        x: col_index as i16,
                        y: row_index as i16,
                    },
                });

                unique_id += 1;
            }
        }

        for bomb_position in bombs_map.iter() {
            Self::update_empty_cells_count(&mut battlefield_map, bomb_position);
        }

        Self {
            map: battlefield_map,
        }
    }

    /// Returns cols count based on `map`
    fn cols_count(&self) -> u16 {
        self.map.len() as u16
    }

    /// Returns rows count based on `map`
    /// Assumes that all cells have the same `cols` and `rows`
    fn rows_count(&self) -> u16 {
        self.map[0].len() as u16
    }

    pub fn reveal(&mut self, cell_id: CellId) -> Vec<Cell> {
        let cell = self.get_mut(cell_id);
        cell.state = CellState::Revealed;

        // Create accumulator to save all revealed Cells
        // let mut accumulator = Vec::new();
        let mut accumulator = vec![*cell];
        self.reveal_priv(cell_id, &mut accumulator);

        accumulator
    }

    fn reveal_priv(&mut self, cell_id: CellId, accumulator: &mut Vec<Cell>) {
        let cell = self.get_mut(cell_id);
        cell.state = CellState::Revealed;

        if cell.ctype == CellType::Empty(0) {
            let position = cell.position;

            // console::log_1(&JsValue::from(
            //     "Cell type is `0` we should start flood fill algorithm",
            // ));
            self.flood_fill(position, accumulator);
        }
    }

    fn flood_fill(&mut self, cell_position: Position, accumulator: &mut Vec<Cell>) {
        for col in -1..2 {
            for row in -1..2 {
                // let message1 = format!(
                //     "cell_position: {} / {}, col/row: {} / {}",
                //     cell_position.x, cell_position.y, col, row
                // );
                // console::log_1(&JsValue::from(message1));

                // Check if we out of bounce
                let position_x = cell_position.x + col;
                let position_y = cell_position.y + row;

                // Check if position for current cell inside the map
                //  not less than 0
                //  and not more than number of cols and rows
                let is_position_in_map = position_x >= 0
                    && position_y >= 0
                    && position_x < self.cols_count() as i16
                    && position_y < self.rows_count() as i16;

                if is_position_in_map {
                    let x = position_x as usize;
                    let y = position_y as usize;

                    let cell = self.map[x][y];

                    if cell.ctype != CellType::Mine && cell.state != CellState::Revealed {
                        accumulator.push(cell);

                        // If cell is not a mine, and it's not revealed
                        // we have to call `reveal` method again
                        self.reveal_priv(cell.id, accumulator);
                    }
                }
            }
        }
    }

    /// Find and returns Vec of `Cells` by `position` and returns flood_fill
    ///  of nearby elements from top-left to bottom-right
    fn update_empty_cells_count<'a>(map: &'a mut BattlefieldMap, cell_position: &'a Position) {
        for col in -1..2 {
            for row in -1..2 {
                let option_cell = Self::get_by_position(
                    map,
                    Position {
                        x: cell_position.x + col,
                        y: cell_position.y + row,
                    },
                );

                if let Some(cell) = option_cell {
                    let new_type = match cell.ctype {
                        CellType::Mine => CellType::Mine,
                        CellType::Empty(count) => CellType::Empty(count + 1),
                    };

                    cell.ctype = new_type;
                }
            }
        }
    }

    /// Returns a mutable link to the cell by provided `id`
    pub fn get_mut(&mut self, id: CellId) -> &mut Cell {
        for row in &mut self.map {
            for cell in row {
                if cell.id == id {
                    return cell;
                }
            }
        }

        panic!("Cell didn't find in battlefield by provided id: {}", id);
    }

    /// Returns a link to the cell by provided `id`
    fn get_by_id(map: &BattlefieldMap, id: CellId) -> &Cell {
        for row in map {
            for col in row {
                if col.id == id {
                    return col;
                }
            }
        }

        panic!("Cell didn't find in battlefield by provided id: {}", id);
    }

    /// Returns a link to the cell by provided `id`
    fn get_by_position(map: &mut BattlefieldMap, position: Position) -> Option<&mut Cell> {
        if let Some(item) = map.get_mut(position.x as usize) {
            if let Some(val) = item.get_mut(position.y as usize) {
                Some(val)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns all matrix map
    pub fn get_all(&self) -> &Vec<Vec<Cell>> {
        &self.map
    }
}

#[cfg(test)]
mod battlefield_test {
    use crate::engine::{BattleField, Cell, CellState, CellType, Position};

    #[test]
    fn should_return_cell_by_specify_position() {
        let mut battlefield = BattleField::new(10, 10, 0);
        let cell = BattleField::get_by_position(&mut battlefield.map, Position { x: 5, y: 5 });

        if let Some(c) = cell {
            println!("Should be here!");
            assert_eq!(
                *c,
                Cell {
                    id: 55,
                    position: Position { x: 5, y: 5 },
                    state: CellState::Hidden,
                    ctype: CellType::Empty(0)
                }
            )
        }
    }

    #[test]
    fn should_not_return_cell_by_specify_position_because_it_is_out_of_bounce() {
        let mut battlefield = BattleField::new(10, 10, 0);
        let cell = BattleField::get_by_position(&mut battlefield.map, Position { x: -1, y: -1 });

        if let Some(_) = cell {
            panic!("Cell mustn't be Some in that particular scenario");
        } else {
            assert_eq!(cell.is_none(), true);
        }
    }
}
