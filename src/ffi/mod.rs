pub mod error;
pub mod gamesettings;
pub mod gamestate;

#[cfg(test)]
mod tests {
    use crate::ffi::gamestate::GameState;
    use crate::settings::GameSettings;

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

        assert!(
            !game_state.as_ptr().is_null(),
            "Game state pointer should not be null"
        );
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
                game_state = game_state.advance().expect("Failed to advance game state");
            } else {
                break;
            }
        }

        assert!(
            !game_state.as_ptr().is_null(),
            "Game state pointer shouldnt be null"
        );
    }

    // TODO chris, we dont have enough insight into the running game to tell when its complete 
    // (such as the current state), so we technically dont know when to stop running the game.
    // We should expand the API to allow for this at least, possibly observing the current event
    // would be enough, or gleaming parts of the gamestate in some C-ABI safe way
}
