
pub mod gamestate;
pub mod gamesettings;
pub mod error;

#[cfg(test)]
mod tests {
    use crate::settings::GameSettings;
    use crate::ffi::gamestate::GameState;

    #[test]
    fn can_create_game() {

        let settings = GameSettings {
            seed: 12345,
            seat_controllers: [
                "AlphabeticalBot".to_string(),
                "AlphabeticalBot".to_string(),
                "AlphabeticalBot".to_string(),
                "AlphabeticalBot".to_string(),
            ],
        };

        let game_state = GameState::new(settings).unwrap();

        assert!(!game_state.as_ptr().is_null(), "Game state pointer should not be null");
    }

    #[test]
    fn can_run_game() {
        let settings = GameSettings {
            seed: 12345,
            seat_controllers: [
                "AlphabeticalBot".to_string(),
                "AlphabeticalBot".to_string(),
                "AlphabeticalBot".to_string(),
                "AlphabeticalBot".to_string(),
            ],
        };

        let mut game_state = GameState::new(settings).unwrap();

        let mut counter = 0;
        loop {
            if counter < 5 {
                counter += 1;
                game_state = game_state.advance()
                    .expect("Failed to advance game state");
            } else {
                break;
            }
        };

        assert!(!game_state.as_ptr().is_null(), "Game state pointer shouldnt be null");
    }
}