pub mod controller_manager;
pub mod event;
pub mod piece;
pub mod player_controller;
pub mod settings;
pub mod wind;

#[cfg(test)]
mod tests {
    use crate::{
        ffi::event::{Event, EventType},
        MahjongGame,
    };

    use super::{
        controller_manager,
        player_controller::{create_player_controller_ffi, PlayerController},
        settings::GameSettings,
    };
    use controller_manager::controller_manager;

    struct TestGameController {
        last_event: Event,
    }

    impl TestGameController {
        pub fn new() -> Self {
            Self {
                last_event: Event::decline(),
            }
        }
    }

    impl PlayerController for TestGameController {
        fn game_start(&mut self, player_id: i32) {
            println!("Game started for player {}", player_id);
        }

        fn round_start(
            &mut self,
            hand: Vec<super::piece::Piece>,
            _seat_wind: super::wind::Wind,
            _prevalent_wind: super::wind::Wind,
        ) {
            println!("Round started with hand: {:?}", hand);
        }

        fn receive_event(&mut self, event: super::event::Event) {
            println!("Received event: {:?}", event);
            self.last_event = event;
        }

        fn retrieve_decision(&mut self) -> super::event::Event {
            println!("Retrieving decision");
            if self.last_event.decision {
                Event::decline()
            } else {
                Event::action(EventType::Discard, self.last_event.piece)
            }
        }

        fn name(&self) -> String {
            "DiscardoBot".to_string()
        }
    }

    #[test]
    fn run_game() {
        controller_manager().lock().unwrap().register_controller(
            Box::new(|| Box::new(create_player_controller_ffi(TestGameController::new()))),
            "TestController",
        );

        let settings =
            unsafe { GameSettings::new(vec!["TestController".to_string(); 4], vec![], 1337) };

        let game = MahjongGame::new(&settings, false);
    }
}
