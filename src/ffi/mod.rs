pub mod error;
pub mod gamesettings;
pub mod gamestate;
pub mod observe;

#[cfg(test)]
mod tests {
    use crate::ffi::gamestate::GameState;
    use crate::settings::GameSettings;
    use crate::observe::StateFunctionType;

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

    #[test]
    fn can_observe_game_state() {
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
        
        // Test that we can observe the game state
        let observed = game_state.observe();
        
        // Basic sanity checks - newly created game state will have initial values
        assert_eq!(observed.seed(), 12345);
        // Current player is -1 for newly created game (not started yet)
        assert_eq!(observed.current_player(), -1);
        // Current state should be Error for uninitialized game
        assert_eq!(observed.current_state(), StateFunctionType::Error);
        
        // Check that we have correct array sizes
        assert_eq!(observed.hands().len(), 4);
        assert_eq!(observed.scores().len(), 4);
        assert_eq!(observed.points().len(), 4);
        assert_eq!(observed.has_ronned().len(), 4);
    }
}
