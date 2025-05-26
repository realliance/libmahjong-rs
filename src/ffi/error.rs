#[derive(Debug, thiserror::Error)]
/// Error type for Mahjong FFI operations
pub enum MahjongFFIError {
    #[error("Failed to create CString from Rust string")]
    FailedToCreateCString,
    #[error("Failed to allocate a game with given settings")]
    FailedToAllocateGameState,
}
