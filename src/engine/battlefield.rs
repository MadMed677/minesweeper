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

    /// How many flags user may set
    ///  it's the same value as bombs
    pub flags_left: u16,

    /// How many bombs has been placed on the map
    bombs: u16,
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
                }
            }
        }

        Self {
            map: battlefield_map,
            flags_left: bombs,
            bombs,
        }
    }

    /// Creates a battlefield with provided map
    ///  works only for `test` scenario
    #[cfg(test)]
    fn with_map(map: BattlefieldMap) -> Self {
        let bombs_count = map.iter().fold(0, |outer_acc, row| {
            outer_acc
                + row.iter().fold(0, |inner_acc, cell| {
                    if cell.ctype == CellType::Mine {
                        inner_acc + 1
                    } else {
                        inner_acc
                    }
                })
        });

        Self {
            map,
            flags_left: bombs_count,
            bombs: bombs_count,
        }
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

            let flags_left = self.flags_left();
            self.flags_left = flags_left;

            Reveal {
                game_is_over: true,
                cells: revealed_cells_accumulator,
            }
        } else {
            let flags_left = self.flags_left();
            self.flags_left = flags_left;

            Reveal {
                game_is_over: false,
                cells: revealed_cells_accumulator,
            }
        }
    }

    /// Flag the cell by provided `CellId` and
    ///  returns the Cell
    pub fn flag(&mut self, cell_id: CellId) -> &Cell {
        let cell = self.get(cell_id);
        let is_flagged = cell.state == CellState::Flagged;

        // User wants to unflag the cell
        //  we may do that without any restrictions
        if is_flagged {
            self.flags_left += 1;

            let cell = self.get_mut(cell_id);
            cell.flag();

            cell
        } else {
            // User wants to flag the cell
            //  we have to check if is it possible or not

            // We can't flag the cell
            if self.flags_left == 0 {
                self.get_mut(cell_id)
            } else {
                self.flags_left -= 1;

                let cell = self.get_mut(cell_id);
                cell.flag();

                cell
            }
        }
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

    /// Returns immutable link to the cell by provided `id`
    fn get(&self, id: CellId) -> &Cell {
        for row in &self.map {
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

    /// Returns count of how many flags are left
    ///
    /// Note: It's very consumable method which
    ///  iterate through all cells on the battlefield
    ///  and count how many cells aren't flagged
    fn flags_left(&self) -> u16 {
        let mut flags_left = self.bombs;

        for row in &self.map {
            for cell in row {
                if cell.state == CellState::Flagged {
                    flags_left -= 1;
                }
            }
        }

        flags_left
    }
}

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
        // |0, 0, 0|
        // |0, 0, 0|
        // |0, 0, 0|
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
            .iter()
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

    #[test]
    fn should_reveal_and_return_only_unrevealed_elements() {
        // |0, 0, 1|
        // |0, 0, 1|
        // |1, 1, 1|
        let map = vec![
            vec![
                Cell::new(0, CellType::Empty(0), CellPosition { x: 0, y: 0 }),
                Cell::new(1, CellType::Empty(0), CellPosition { x: 0, y: 1 }),
                Cell {
                    id: 2,
                    state: CellState::Revealed,
                    ctype: CellType::Empty(0),
                    position: CellPosition { x: 0, y: 2 },
                },
            ],
            vec![
                Cell::new(3, CellType::Empty(0), CellPosition { x: 1, y: 0 }),
                Cell::new(4, CellType::Empty(0), CellPosition { x: 1, y: 1 }),
                Cell {
                    id: 5,
                    state: CellState::Revealed,
                    ctype: CellType::Empty(0),
                    position: CellPosition { x: 1, y: 2 },
                },
            ],
            vec![
                Cell {
                    id: 6,
                    state: CellState::Revealed,
                    ctype: CellType::Empty(0),
                    position: CellPosition { x: 2, y: 0 },
                },
                Cell {
                    id: 7,
                    state: CellState::Revealed,
                    ctype: CellType::Empty(0),
                    position: CellPosition { x: 2, y: 1 },
                },
                Cell {
                    id: 8,
                    state: CellState::Revealed,
                    ctype: CellType::Empty(0),
                    position: CellPosition { x: 2, y: 2 },
                },
            ],
        ];
        let mut battlefield = BattleField::with_map(map);

        // Start reveal from the top-left cell
        //  it should affect all cells and at
        //  the end all 9 cells must be revealed
        let revealed = battlefield.reveal(0);
        let revealed_cells_state = revealed
            .cells
            .iter()
            .map(|cell| cell.state)
            .collect::<Vec<CellState>>();

        let revealed_cells_id = revealed
            .cells
            .iter()
            .map(|cell| cell.id)
            .collect::<Vec<CellId>>();

        // Only 4 should have `Revealed` state
        assert_eq!(
            revealed_cells_state,
            vec![
                CellState::Revealed,
                CellState::Revealed,
                CellState::Revealed,
                CellState::Revealed,
            ]
        );

        // Only specific 4 cell should be uncovered
        assert_eq!(revealed_cells_id, vec![0, 1, 3, 4]);
    }

    #[test]
    fn should_reveal_cells_and_not_take_the_bomb() {
        // |0, 0, 0|
        // |0, 1, 1|
        // |0, 1, b|
        let map = vec![
            vec![
                Cell {
                    id: 0,
                    state: CellState::Hidden,
                    ctype: CellType::Empty(0),
                    position: CellPosition { x: 0, y: 0 },
                },
                Cell {
                    id: 1,
                    state: CellState::Hidden,
                    ctype: CellType::Empty(0),
                    position: CellPosition { x: 0, y: 1 },
                },
                Cell {
                    id: 2,
                    state: CellState::Hidden,
                    ctype: CellType::Empty(1),
                    position: CellPosition { x: 0, y: 2 },
                },
            ],
            vec![
                Cell {
                    id: 3,
                    state: CellState::Hidden,
                    ctype: CellType::Empty(0),
                    position: CellPosition { x: 1, y: 0 },
                },
                Cell {
                    id: 4,
                    state: CellState::Hidden,
                    ctype: CellType::Empty(0),
                    position: CellPosition { x: 1, y: 1 },
                },
                Cell {
                    id: 5,
                    state: CellState::Hidden,
                    ctype: CellType::Empty(1),
                    position: CellPosition { x: 1, y: 2 },
                },
            ],
            vec![
                Cell {
                    id: 6,
                    state: CellState::Hidden,
                    ctype: CellType::Empty(0),
                    position: CellPosition { x: 2, y: 0 },
                },
                Cell {
                    id: 7,
                    state: CellState::Hidden,
                    ctype: CellType::Empty(0),
                    position: CellPosition { x: 2, y: 1 },
                },
                Cell {
                    id: 8,
                    state: CellState::Hidden,
                    ctype: CellType::Mine,
                    position: CellPosition { x: 2, y: 2 },
                },
            ],
        ];
        let mut battlefield = BattleField::with_map(map);

        // Start reveal from the top-left cell
        //  it should affect all cells and at
        //  the end all 9 cells must be revealed
        let revealed = battlefield.reveal(0);
        let revealed_cells_state = revealed
            .cells
            .iter()
            .map(|cell| cell.state)
            .collect::<Vec<CellState>>();

        let revealed_cells_id = revealed
            .cells
            .iter()
            .map(|cell| cell.id)
            .collect::<Vec<CellId>>();

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
            ]
        );
        assert_eq!(revealed_cells_id, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn should_reveal_all_elements_if_user_select_the_bomb() {
        // |0, 0, 0|
        // |0, 1, 1|
        // |0, 1, b|
        let map = vec![
            vec![
                Cell {
                    id: 0,
                    state: CellState::Hidden,
                    ctype: CellType::Empty(0),
                    position: CellPosition { x: 0, y: 0 },
                },
                Cell {
                    id: 1,
                    state: CellState::Hidden,
                    ctype: CellType::Empty(0),
                    position: CellPosition { x: 0, y: 1 },
                },
                Cell {
                    id: 2,
                    state: CellState::Hidden,
                    ctype: CellType::Empty(1),
                    position: CellPosition { x: 0, y: 2 },
                },
            ],
            vec![
                Cell {
                    id: 3,
                    state: CellState::Hidden,
                    ctype: CellType::Empty(0),
                    position: CellPosition { x: 1, y: 0 },
                },
                Cell {
                    id: 4,
                    state: CellState::Hidden,
                    ctype: CellType::Empty(0),
                    position: CellPosition { x: 1, y: 1 },
                },
                Cell {
                    id: 5,
                    state: CellState::Hidden,
                    ctype: CellType::Empty(1),
                    position: CellPosition { x: 1, y: 2 },
                },
            ],
            vec![
                Cell {
                    id: 6,
                    state: CellState::Hidden,
                    ctype: CellType::Empty(0),
                    position: CellPosition { x: 2, y: 0 },
                },
                Cell {
                    id: 7,
                    state: CellState::Hidden,
                    ctype: CellType::Empty(0),
                    position: CellPosition { x: 2, y: 1 },
                },
                Cell {
                    id: 8,
                    state: CellState::Hidden,
                    ctype: CellType::Mine,
                    position: CellPosition { x: 2, y: 2 },
                },
            ],
        ];
        let mut battlefield = BattleField::with_map(map);

        // Start reveal the bomb's id
        let revealed = battlefield.reveal(8);
        let revealed_cells_state = revealed
            .cells
            .iter()
            .map(|cell| cell.state)
            .collect::<Vec<CellState>>();

        let mut revealed_cells_id = revealed
            .cells
            .iter()
            .map(|cell| cell.id)
            .collect::<Vec<CellId>>();

        revealed_cells_id.sort_unstable();

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
                CellState::Revealed,
            ]
        );
        assert_eq!(revealed_cells_id, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn should_flag_the_cell() {
        let map = vec![vec![
            Cell {
                id: 0,
                state: CellState::Hidden,
                ctype: CellType::Empty(0),
                position: CellPosition { x: 0, y: 0 },
            },
            Cell {
                id: 1,
                state: CellState::Hidden,
                ctype: CellType::Mine,
                position: CellPosition { x: 0, y: 0 },
            },
        ]];
        let mut battlefield = BattleField::with_map(map);

        let cell = battlefield.flag(0);

        assert_eq!(
            cell,
            &Cell {
                id: 0,
                ctype: CellType::Empty(0),
                state: CellState::Flagged,
                position: CellPosition { x: 0, y: 0 }
            }
        );
    }

    #[test]
    fn should_not_flag_the_cell_if_it_is_revealed() {
        let map = vec![vec![Cell {
            id: 0,
            state: CellState::Revealed,
            ctype: CellType::Empty(0),
            position: CellPosition { x: 0, y: 0 },
        }]];
        let mut battlefield = BattleField::with_map(map);

        let cell = battlefield.flag(0);

        assert_eq!(
            cell,
            &Cell {
                id: 0,
                state: CellState::Revealed,
                ctype: CellType::Empty(0),
                position: CellPosition { x: 0, y: 0 }
            }
        );
    }

    #[test]
    fn should_unflag_the_cell_if_it_is_flagged() {
        let map = vec![vec![Cell {
            id: 0,
            state: CellState::Flagged,
            ctype: CellType::Empty(0),
            position: CellPosition { x: 0, y: 0 },
        }]];
        let mut battlefield = BattleField::with_map(map);

        let cell = battlefield.flag(0);

        assert_eq!(
            cell,
            &Cell {
                id: 0,
                state: CellState::Hidden,
                ctype: CellType::Empty(0),
                position: CellPosition { x: 0, y: 0 }
            }
        );
    }

    #[test]
    fn should_build_correct_max_flag_values_counter() {
        let map = vec![vec![
            Cell {
                id: 0,
                state: CellState::Hidden,
                ctype: CellType::Empty(0),
                position: CellPosition { x: 0, y: 0 },
            },
            Cell {
                id: 1,
                state: CellState::Hidden,
                ctype: CellType::Mine,
                position: CellPosition { x: 0, y: 0 },
            },
            Cell {
                id: 2,
                state: CellState::Hidden,
                ctype: CellType::Mine,
                position: CellPosition { x: 0, y: 0 },
            },
        ]];
        let mut battlefield = BattleField::with_map(map);

        assert_eq!(battlefield.flags_left, 2);

        battlefield.flag(0);

        assert_eq!(battlefield.flags_left, 1);
    }

    #[test]
    fn should_not_set_more_flags_that_is_possible() {
        let map = vec![vec![
            Cell {
                id: 0,
                state: CellState::Hidden,
                ctype: CellType::Empty(0),
                position: CellPosition { x: 0, y: 0 },
            },
            Cell {
                id: 1,
                state: CellState::Hidden,
                ctype: CellType::Mine,
                position: CellPosition { x: 0, y: 0 },
            },
            Cell {
                id: 2,
                state: CellState::Hidden,
                ctype: CellType::Mine,
                position: CellPosition { x: 0, y: 0 },
            },
        ]];
        let mut battlefield = BattleField::with_map(map);

        let cell0 = battlefield.flag(0);
        assert_eq!(cell0.state, CellState::Flagged);

        let cell1 = battlefield.flag(1);
        assert_eq!(cell1.state, CellState::Flagged);

        let cell2 = battlefield.flag(2);
        assert_eq!(cell2.state, CellState::Hidden);

        assert_eq!(battlefield.flags_left, 0);
    }

    #[test]
    fn should_unflag_already_flagged_cells() {
        let map = vec![vec![
            Cell {
                id: 0,
                state: CellState::Hidden,
                ctype: CellType::Empty(0),
                position: CellPosition { x: 0, y: 0 },
            },
            Cell {
                id: 1,
                state: CellState::Hidden,
                ctype: CellType::Mine,
                position: CellPosition { x: 0, y: 0 },
            },
            Cell {
                id: 2,
                state: CellState::Hidden,
                ctype: CellType::Mine,
                position: CellPosition { x: 0, y: 0 },
            },
        ]];
        let mut battlefield = BattleField::with_map(map);

        assert_eq!(battlefield.flags_left, 2);
        let cell0 = battlefield.flag(0);
        assert_eq!(cell0.state, CellState::Flagged);
        assert_eq!(battlefield.flags_left, 1);

        let cell1 = battlefield.flag(1);
        assert_eq!(cell1.state, CellState::Flagged);
        assert_eq!(battlefield.flags_left, 0);

        // Should NOT flag the cell by `cell_id: 2`
        let cell2 = battlefield.flag(2);
        assert_eq!(cell2.state, CellState::Hidden);
        assert_eq!(battlefield.flags_left, 0);

        // Should unflag the cell by `cell_id: 0`
        let cell0 = battlefield.flag(0);
        assert_eq!(cell0.state, CellState::Hidden);
        assert_eq!(battlefield.flags_left, 1);

        // Should unflag the cell by `cell_id: 1`
        let cell1 = battlefield.flag(1);
        assert_eq!(cell1.state, CellState::Hidden);
        assert_eq!(battlefield.flags_left, 2);

        // Should flag the cell by `cell_id: 2`
        let cell2 = battlefield.flag(2);
        assert_eq!(cell2.state, CellState::Flagged);
        assert_eq!(battlefield.flags_left, 1);
    }

    #[test]
    fn should_have_actual_state_value_when_reveal_flagged_cell() {
        let map = vec![vec![
            Cell {
                id: 0,
                state: CellState::Hidden,
                ctype: CellType::Empty(0),
                position: CellPosition { x: 0, y: 0 },
            },
            Cell {
                id: 1,
                state: CellState::Hidden,
                ctype: CellType::Mine,
                position: CellPosition { x: 0, y: 1 },
            },
            Cell {
                id: 2,
                state: CellState::Hidden,
                ctype: CellType::Mine,
                position: CellPosition { x: 0, y: 2 },
            },
            Cell {
                id: 3,
                state: CellState::Hidden,
                ctype: CellType::Mine,
                position: CellPosition { x: 0, y: 3 },
            },
        ]];

        let mut battlefield = BattleField::with_map(map);

        assert_eq!(battlefield.flags_left, 3);

        // Flag two cells
        battlefield.flag(0);
        battlefield.flag(1);

        assert_eq!(battlefield.flags_left, 1);

        battlefield.reveal(0);

        // After revealing we have to have one empty flag left
        assert_eq!(battlefield.flags_left, 2);
    }
}
