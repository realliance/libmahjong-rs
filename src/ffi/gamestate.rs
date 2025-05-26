use std::ffi::c_int;

use super::{error::MahjongFFIError, gamesettings::CGameSettings};

/// Opaque type representing a GameState
#[repr(C)]
pub struct RawGameState {
    _data: [u8; 0],
    _marker: std::marker::PhantomData<(*mut u8, std::marker::PhantomPinned)>,
}

#[link(name = "mahjong")]
extern "C" {
    /// Start a game with the given settings
    pub fn StartGame(settings: *const CGameSettings, async_mode: bool) -> c_int;

    /// Exit a game by its ID
    pub fn ExitGame(game: c_int);

    /// Initialize a new game state
    pub fn InitGameState(settings: *const CGameSettings) -> *mut RawGameState;

    /// Advance the game state to the next state
    pub fn AdvanceGameState(state: *mut RawGameState) -> *mut RawGameState;

    /// Free a game state
    pub fn FreeGameState(state: *mut RawGameState);
}

/// Safe wrapper for GameState
pub struct GameState {
    ptr: *mut RawGameState,
}

impl GameState {
    /// Create a new game state from settings
    pub fn new<S: TryInto<CGameSettings, Error = MahjongFFIError>>(
        settings: S,
    ) -> Result<Self, MahjongFFIError> {
        let ptr = unsafe {
            let settings: CGameSettings = settings.try_into()?;
            Ok(InitGameState(&settings))
        }?;
        if ptr.is_null() {
            Err(MahjongFFIError::FailedToAllocateGameState)
        } else {
            Ok(Self { ptr })
        }
    }

    /// Advance the game state
    pub fn advance(self) -> Option<Self> {
        let new_ptr = unsafe { AdvanceGameState(self.ptr) };
        std::mem::forget(self); // Prevent double-free since C++ takes ownership
        if new_ptr.is_null() {
            None
        } else {
            Some(Self { ptr: new_ptr })
        }
    }

    /// Get the raw pointer
    pub fn as_ptr(&self) -> *mut RawGameState {
        self.ptr
    }
}

impl Drop for GameState {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { FreeGameState(self.ptr) };
        }
    }
}

/// Safe wrapper for game operations
pub fn start_game(settings: &CGameSettings, async_mode: bool) -> c_int {
    unsafe { StartGame(settings as *const CGameSettings, async_mode) }
}

pub fn exit_game(game_id: c_int) {
    unsafe { ExitGame(game_id) }
}
