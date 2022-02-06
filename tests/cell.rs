use minesweeper_engine::engine::{Cell, CellPosition, CellState, CellType};

#[test]
fn should_create_cell_via_constructor() {
    let cell = Cell::new(0, CellType::Empty(0), CellPosition { x: 0, y: 0 });

    assert_eq!(
        cell,
        Cell {
            id: 0,
            state: CellState::Hidden,
            ctype: CellType::Empty(0),
            position: CellPosition { x: 0, y: 0 },
        }
    );
}

mod revealing {
    use minesweeper_engine::engine::*;

    #[test]
    fn should_reveal_the_cell() {
        let mut cell = Cell {
            id: 0,
            state: CellState::Hidden,
            ctype: CellType::Empty(0),
            position: CellPosition { x: 0, y: 0 },
        };

        cell.reveal();

        assert_eq!(
            cell,
            Cell {
                id: 0,
                state: CellState::Revealed,
                ctype: CellType::Empty(0),
                position: CellPosition { x: 0, y: 0 },
            }
        );
    }
}

mod flagging {
    use minesweeper_engine::engine::*;

    #[test]
    fn should_flag_the_cell() {
        let mut cell = Cell {
            id: 0,
            state: CellState::Hidden,
            ctype: CellType::Empty(0),
            position: CellPosition { x: 0, y: 0 },
        };

        let is_flagged = cell.flag();

        assert_eq!(
            cell,
            Cell {
                id: 0,
                state: CellState::Flagged,
                ctype: CellType::Empty(0),
                position: CellPosition { x: 0, y: 0 },
            }
        );

        assert!(is_flagged);
    }

    #[test]
    fn should_unflag_the_cell() {
        let mut cell = Cell {
            id: 0,
            state: CellState::Hidden,
            ctype: CellType::Empty(0),
            position: CellPosition { x: 0, y: 0 },
        };

        // Should flag the cell
        cell.flag();

        // Should unflag the cell
        cell.flag();

        assert_eq!(
            cell,
            Cell {
                id: 0,
                state: CellState::Hidden,
                ctype: CellType::Empty(0),
                position: CellPosition { x: 0, y: 0 },
            }
        );
    }

    #[test]
    fn should_not_flag_the_cell_if_it_is_revealed() {
        let mut cell = Cell {
            id: 0,
            state: CellState::Revealed,
            ctype: CellType::Empty(0),
            position: CellPosition { x: 0, y: 0 },
        };

        // Should NOT flag the cell
        let is_flagged = cell.flag();

        assert_eq!(
            cell,
            Cell {
                id: 0,
                state: CellState::Revealed,
                ctype: CellType::Empty(0),
                position: CellPosition { x: 0, y: 0 },
            }
        );

        assert!(!is_flagged);
    }
}
