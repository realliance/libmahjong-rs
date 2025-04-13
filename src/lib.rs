// FFI (Foreign Function Interface) for Mahjong game controller

pub mod ffi;

use ffi::settings::GameSettings;

#[link(name = "mahjong")]
extern "C" {
    pub fn mahjong_StartGame(settings: *const GameSettings, async_mode: bool) -> i32;
    pub fn mahjong_ExitGame(game: i32);
    pub fn mahjong_StateController(settings: *const GameSettings);
}

pub struct MahjongGame {
    id: i32,
    settings: GameSettings,
}

impl MahjongGame {
    pub fn new(settings: &GameSettings, async_mode: bool) -> Self {
        let id = unsafe { mahjong_StartGame(settings, async_mode) };
        MahjongGame { id, settings: settings.clone() }
    }
    
    pub fn exit(&self) {
        unsafe { mahjong_ExitGame(self.id) }
    }

    pub fn run_state_controller(&self) {
        unsafe { mahjong_StateController(&self.settings) }
    }
}
