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

pub struct Reveal {
    pub game_is_over: bool,
    pub cells: Vec<Cell>,
}

impl BattleField {
    /// Creates an empty battlefield map with
    ///  no mines and without any text messages
    ///
    /// Notes
    ///  But it should place bombs and text messages recording to the bombs
    pub fn new(rows: usize, cols: usize, bombs: u16) -> Self {
        let mut battlefield_map = Vec::with_capacity(cols);

        // Calculates bomb positions in random place on the map
        let mut bombs_map = Vec::<CellPosition>::with_capacity(bombs as usize);
        for _ in 0..bombs {
            let mut bomb_position = Self::get_random_bomb_location(cols, rows);

            // If we have the same position which already has
            //  in `bombs_map` vector we have to re-create bomb location
            //  to avoid duplications
            for bomb_pos in bombs_map.iter() {
                if bomb_pos == &bomb_position {
                    bomb_position = Self::get_random_bomb_location(cols, rows);
                }
            }

            bombs_map.push(bomb_position);
        }

        let mut unique_id = 0;
        for col_index in 0..cols {
            battlefield_map.push(Vec::with_capacity(rows));

            for row_index in 0..rows {
                let mut ctype = CellType::Empty(0);
                for bomb_position in bombs_map.iter() {
                    if bomb_position.y == row_index as i16 && bomb_position.x == col_index as i16 {
                        ctype = CellType::Mine;
                    }
                }

                battlefield_map[col_index].push(Cell::new(
                    unique_id,
                    ctype,
                    CellPosition {
                        x: col_index as i16,
                        y: row_index as i16,
                    },
                ));

                unique_id += 1;
            }
        }

        // Update counts for each cell which are nearby bombs
        for bomb_position in bombs_map.iter() {
            for col in -1..2 {
                for row in -1..2 {
                    let position_x = bomb_position.x + col;
                    let position_y = bomb_position.y + row;

                    // Check if position for current cell inside the map
                    //  not less than 0
                    //  and not more than number of cols and rows
                    let is_position_inside_map = position_x >= 0
                        && position_y >= 0
                        && position_x < cols as i16
                        && position_y < rows as i16;

                    if is_position_inside_map {
                        let x = position_x as usize;
                        let y = position_y as usize;

                        let cell = &mut battlefield_map[x][y];

                        let new_type = match cell.ctype {
                            CellType::Mine => CellType::Mine,
                            CellType::Empty(count) => CellType::Empty(count + 1),
                        };

                        cell.ctype = new_type;
                    }

                    // let option_cell = Self::get_by_position(
                    //     &mut battlefield_map,
                    //     CellPosition {
                    //         x: bomb_position.x + col,
                    //         y: bomb_position.y + row,
                    //     },
                    // );
                    //
                    // if let Some(cell) = option_cell {
                    //     let new_type = match cell.ctype {
                    //         CellType::Mine => CellType::Mine,
                    //         CellType::Empty(count) => CellType::Empty(count + 1),
                    //     };
                    //
                    //     cell.ctype = new_type;
                    // }
                }
            }
        }

        Self {
            map: battlefield_map,
        }
    }

    /// Creates a battlefield with provided map
    fn with_map(map: BattlefieldMap) -> Self {
        Self { map }
    }

    /// Generates random bomb location by giving `cols` and `rows`
    fn get_random_bomb_location(cols: usize, rows: usize) -> CellPosition {
        let mut rng = rand::thread_rng();

        let random_bomb_col = rng.gen_range(0..cols);
        let random_bomb_row = rng.gen_range(0..rows);

        CellPosition {
            x: random_bomb_col as i16,
            y: random_bomb_row as i16,
        }
    }

    /// Reveals the cell by provided `id`
    /// Returns a vector of cells which were revealed
    ///  based on internal logic when we have to
    ///  reveal all cells which have `0` value
    pub fn reveal(&mut self, cell_id: CellId) -> Reveal {
        // Create accumulator to save all revealed Cells
        let mut revealed_cells_accumulator = vec![];
        self.reveal_recursively(cell_id, &mut revealed_cells_accumulator);

        let option_bomb = revealed_cells_accumulator
            .iter()
            .find(|cell| cell.ctype == CellType::Mine);

        // If we found a bomb we have to move through all
        //  cells, reveal it and return the actual data into
        //  the client
        if option_bomb.is_some() {
            // let mut result = Vec::new();
            for col in self.map.iter_mut() {
                for cell in col {
                    // Do not reveal already revealed cells
                    if cell.state != CellState::Revealed {
                        // Update cell state
                        cell.reveal();
                        revealed_cells_accumulator.push(*cell);
                    }
                }
            }

            Reveal {
                game_is_over: true,
                cells: revealed_cells_accumulator,
            }
        } else {
            Reveal {
                game_is_over: false,
                cells: revealed_cells_accumulator,
            }
        }
    }

    /// Flag the cell by provided `CellId` and
    ///  returns the Cell
    pub fn flag(&mut self, cell_id: CellId) -> Cell {
        let cell = self.get_mut(cell_id);
        cell.flag();

        *cell
    }

    /// Reveals the cell and iteratively execute `flood_fill` method
    ///  to calculate all near cells and reveal them too if
    ///  they have an `Empty` status and the value of the
    ///  cell is `0`
    fn reveal_recursively(&mut self, cell_id: CellId, accumulator: &mut Vec<Cell>) {
        let cell = self.get_mut(cell_id);
        cell.reveal();
        accumulator.push(*cell);

        if cell.ctype == CellType::Empty(0) {
            let position = cell.position;

            self.flood_fill(position, accumulator);
        }
    }

    /// Calculates all near cells based on `cell_position` and call
    ///  `reveal_priv` to reveal these cells if they have `0` value
    ///
    /// @see https://en.wikipedia.org/wiki/Flood_fill
    fn flood_fill(&mut self, cell_position: CellPosition, accumulator: &mut Vec<Cell>) {
        for col in -1..2 {
            for row in -1..2 {
                let position = CellPosition {
                    x: cell_position.x + col,
                    y: cell_position.y + row,
                };
                let option_cell = self.get_by_position(position);

                if let Some(cell) = option_cell {
                    if cell.ctype != CellType::Mine && cell.state != CellState::Revealed {
                        let cell_id = cell.id;

                        // If cell is not a mine, and it's not revealed
                        // we have to call `reveal` method again
                        self.reveal_recursively(cell_id, accumulator);
                    }
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
    fn get_by_position(&self, position: CellPosition) -> Option<&Cell> {
        if let Some(column) = self.map.get(position.x as usize) {
            if let Some(cell) = column.get(position.y as usize) {
                Some(cell)
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
    use crate::engine::*;

    #[test]
    fn should_create_field_4_by_10() {
        let battlefield = BattleField::new(10, 4, 0);
        let field = battlefield.get_all();

        assert_eq!(field.len(), 4);
        assert_eq!(field[0].len(), 10);
    }

    #[test]
    fn field_should_contains_10_bombs() {
        let battlefield = BattleField::new(10, 10, 10);
        let field = battlefield.get_all();

        let mut bombs_count = 0;
        for col in field.iter() {
            for cell in col {
                if cell.ctype == CellType::Mine {
                    bombs_count += 1;
                }
            }
        }

        assert_eq!(bombs_count, 10);
    }

    #[test]
    fn should_return_cell_by_specify_position() {
        let battlefield = BattleField::new(10, 10, 0);
        let cell = battlefield.get_by_position(CellPosition { x: 1, y: 5 });

        if let Some(c) = cell {
            assert_eq!(
                *c,
                Cell::new(15, CellType::Empty(0), CellPosition { x: 1, y: 5 }),
            )
        }
    }

    #[test]
    fn should_not_return_cell_by_specify_position_because_it_is_out_of_bounce() {
        let battlefield = BattleField::new(10, 10, 0);
        let cell = battlefield.get_by_position(CellPosition { x: -3, y: -1 });

        if cell.is_some() {
            panic!("Cell mustn't be Some in that particular scenario");
        } else {
            assert!(cell.is_none());
        }
    }

    mod event_logic {
        use crate::engine::*;

        #[test]
        fn should_reveal_all_cells_if_none_bombs_were_found() {
            let map = vec![
                vec![
                    Cell::new(0, CellType::Empty(0), CellPosition { x: 0, y: 0 }),
                    Cell::new(1, CellType::Empty(0), CellPosition { x: 0, y: 1 }),
                    Cell::new(2, CellType::Empty(0), CellPosition { x: 0, y: 2 }),
                ],
                vec![
                    Cell::new(3, CellType::Empty(0), CellPosition { x: 1, y: 0 }),
                    Cell::new(4, CellType::Empty(0), CellPosition { x: 1, y: 1 }),
                    Cell::new(5, CellType::Empty(0), CellPosition { x: 1, y: 2 }),
                ],
                vec![
                    Cell::new(6, CellType::Empty(0), CellPosition { x: 2, y: 0 }),
                    Cell::new(7, CellType::Empty(0), CellPosition { x: 2, y: 1 }),
                    Cell::new(8, CellType::Empty(0), CellPosition { x: 2, y: 2 }),
                ],
            ];
            let mut battlefield = BattleField::with_map(map);

            // Start reveal from the top-left cell
            //  it should affect all cells and at
            //  the end all 9 cells must be revealed
            let revealed = battlefield.reveal(0);
            let revealed_cells_state = revealed
                .cells
                .into_iter()
                .map(|cell| cell.state)
                .collect::<Vec<CellState>>();

            assert_eq!(
                revealed_cells_state,
                vec![
                    CellState::Revealed,
                    CellState::Revealed,
                    CellState::Revealed,
                    CellState::Revealed,
                    CellState::Revealed,
                    CellState::Revealed,
                    CellState::Revealed,
                    CellState::Revealed,
                    CellState::Revealed
                ]
            );
        }
    }
}
