use battle_snake::game_state::GameState;


fn main() {
    let mut game_state = GameState::new(200, 80);
    game_state.run();
}
