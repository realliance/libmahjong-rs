use strum_macros::EnumString;

/// Represents the wind directions in Mahjong
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
pub enum Wind {
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}
