use minesweeper_engine::MineSweeperEngine;

#[test]
fn should_return_initial_flags_value() {
    let engine = MineSweeperEngine::create(10, 10, 10);
    let game_state = engine.game_state();

    assert_eq!(game_state.flags, 10);
}
