use super::piece::Piece;

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct GameSettings {
    seatControllers: *const *const i8,
    seatControllers_len: usize,
    overrideWall: *const Piece,
    overrideWall_len: usize,
    seed: u64,
}

impl GameSettings {
    pub unsafe fn new(
        seat_controllers: Vec<String>,
        override_wall: Vec<Piece>,
        seed: u64
    ) -> Self {
        // Convert Vec<String> to Vec<*const i8>
        let c_strings: Vec<_> = seat_controllers
            .iter()
            .map(|s| s.as_ptr() as *const i8)
            .collect();
        
        Self {
            seatControllers: c_strings.as_ptr(),
            seatControllers_len: c_strings.len(),
            overrideWall: override_wall.as_ptr(),
            overrideWall_len: override_wall.len(),
            seed,
        }
    }
}
