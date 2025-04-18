use std::{cmp::Ordering, fmt::Display};

#[repr(C)]
#[derive(Clone, Copy, Debug)]

pub struct Piece {
    p: u8,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_read_five = self.is_red_five();
        let suit = self.get_suit();
        let piece_num = self.get_piece_num();

        let suit_char = match suit {
            constants::BAMBOO_SUIT => 's',
            constants::PIN_SUIT => 'p',
            constants::CHARACTER_SUIT => 'm',
            constants::HONOR_SUIT => 'h',
            _ => '?',
        };

        if is_read_five {
            write!(f, "r")?;
        }

        write!(f, "{}{}", suit_char, piece_num)
    }
}

#[allow(dead_code)]
pub mod constants {
    pub const TERMINAL_BIT: u8 = 1 << 7;
    pub const HONOR_SUIT: u8 = 0 << 5;
    pub const BAMBOO_SUIT: u8 = 1 << 5;
    pub const PIN_SUIT: u8 = 2 << 5;
    pub const CHARACTER_SUIT: u8 = 3 << 5;
    pub const RED_FIVE: u8 = 1 << 4;
    pub const ERROR: u8 = 0;

    // Bamboo pieces
    pub const ONE_BAMBOO: u8 = BAMBOO_SUIT | 1 | TERMINAL_BIT;
    pub const TWO_BAMBOO: u8 = BAMBOO_SUIT | 2;
    pub const THREE_BAMBOO: u8 = BAMBOO_SUIT | 3;
    pub const FOUR_BAMBOO: u8 = BAMBOO_SUIT | 4;
    pub const FIVE_BAMBOO: u8 = BAMBOO_SUIT | 5;
    pub const RED_FIVE_BAMBOO: u8 = BAMBOO_SUIT | 5 | RED_FIVE;
    pub const SIX_BAMBOO: u8 = BAMBOO_SUIT | 6;
    pub const SEVEN_BAMBOO: u8 = BAMBOO_SUIT | 7;
    pub const EIGHT_BAMBOO: u8 = BAMBOO_SUIT | 8;
    pub const NINE_BAMBOO: u8 = BAMBOO_SUIT | 9 | TERMINAL_BIT;

    // Pin pieces
    pub const ONE_PIN: u8 = PIN_SUIT | 1 | TERMINAL_BIT;
    pub const TWO_PIN: u8 = PIN_SUIT | 2;
    pub const THREE_PIN: u8 = PIN_SUIT | 3;
    pub const FOUR_PIN: u8 = PIN_SUIT | 4;
    pub const FIVE_PIN: u8 = PIN_SUIT | 5;
    pub const RED_FIVE_PIN: u8 = PIN_SUIT | 5 | RED_FIVE;
    pub const SIX_PIN: u8 = PIN_SUIT | 6;
    pub const SEVEN_PIN: u8 = PIN_SUIT | 7;
    pub const EIGHT_PIN: u8 = PIN_SUIT | 8;
    pub const NINE_PIN: u8 = PIN_SUIT | 9 | TERMINAL_BIT;

    // Character pieces
    pub const ONE_CHARACTER: u8 = CHARACTER_SUIT | 1 | TERMINAL_BIT;
    pub const TWO_CHARACTER: u8 = CHARACTER_SUIT | 2;
    pub const THREE_CHARACTER: u8 = CHARACTER_SUIT | 3;
    pub const FOUR_CHARACTER: u8 = CHARACTER_SUIT | 4;
    pub const FIVE_CHARACTER: u8 = CHARACTER_SUIT | 5;
    pub const RED_FIVE_CHARACTER: u8 = CHARACTER_SUIT | 5 | RED_FIVE;
    pub const SIX_CHARACTER: u8 = CHARACTER_SUIT | 6;
    pub const SEVEN_CHARACTER: u8 = CHARACTER_SUIT | 7;
    pub const EIGHT_CHARACTER: u8 = CHARACTER_SUIT | 8;
    pub const NINE_CHARACTER: u8 = CHARACTER_SUIT | 9 | TERMINAL_BIT;

    // Honor pieces
    pub const EAST_WIND: u8 = HONOR_SUIT | 1;
    pub const SOUTH_WIND: u8 = HONOR_SUIT | 2;
    pub const WEST_WIND: u8 = HONOR_SUIT | 3;
    pub const NORTH_WIND: u8 = HONOR_SUIT | 4;
    pub const RED_DRAGON: u8 = HONOR_SUIT | 5;
    pub const WHITE_DRAGON: u8 = HONOR_SUIT | 6;
    pub const GREEN_DRAGON: u8 = HONOR_SUIT | 7;
}

impl Piece {
    pub const PIECE_SIZE: usize = 256;

    pub fn new() -> Self {
        Self {
            p: constants::ERROR,
        }
    }

    pub fn from_u8(p: u8) -> Self {
        Self { p }
    }

    pub fn is_honor(&self) -> bool {
        self.get_suit() == constants::HONOR_SUIT
    }

    pub fn is_terminal(&self) -> bool {
        (self.p & constants::TERMINAL_BIT) != 0
    }

    pub fn is_green(&self) -> bool {
        matches!(
            self.p,
            constants::TWO_BAMBOO
                | constants::THREE_BAMBOO
                | constants::FOUR_BAMBOO
                | constants::SIX_BAMBOO
                | constants::EIGHT_BAMBOO
                | constants::GREEN_DRAGON
        )
    }

    pub fn is_red_five(&self) -> bool {
        (self.p & constants::RED_FIVE) != 0
    }

    pub fn is_board_piece(&self) -> bool {
        self.p != constants::ERROR
    }

    pub fn to_u8(&self) -> u8 {
        self.p
    }

    pub fn raw_value(&self) -> u8 {
        self.p
    }

    pub fn get_suit(&self) -> u8 {
        self.p & 0xE0 // Extract the suit bits
    }

    pub fn get_piece_num(&self) -> u8 {
        self.p & 0x0F // Extract the number bits
    }

    pub fn from_wind(wind: u8) -> Self {
        // This would need the proper Wind enum from the winds.h file
        // This is a placeholder implementation
        match wind {
            0 => Self {
                p: constants::EAST_WIND,
            },
            1 => Self {
                p: constants::SOUTH_WIND,
            },
            2 => Self {
                p: constants::WEST_WIND,
            },
            3 => Self {
                p: constants::NORTH_WIND,
            },
            _ => Self {
                p: constants::ERROR,
            },
        }
    }

    pub fn form_piece(suit: u8, number: u8, is_red_five: bool) -> Self {
        let mut p = suit | number;

        if number == 1 || number == 9 {
            p |= constants::TERMINAL_BIT;
        }

        if is_red_five && number == 5 {
            p |= constants::RED_FIVE;
        }

        Self { p }
    }

    pub fn increment(&mut self) {
        self.p += 1;
    }

    pub fn add(&self, i: u8) -> Self {
        if i == 0 {
            return Self { p: self.p };
        }

        if self.is_honor() || (self.get_piece_num() + i > 9) {
            return Self {
                p: constants::ERROR,
            };
        }

        if self.get_piece_num() + i == 9 {
            return Self {
                p: ((self.p + i) & !constants::RED_FIVE) | constants::TERMINAL_BIT,
            };
        }

        Self {
            p: ((self.p + i) & !constants::RED_FIVE) & !constants::TERMINAL_BIT,
        }
    }

    pub fn subtract(&self, i: u8) -> Self {
        if i == 0 {
            return Self { p: self.p };
        }

        if self.is_honor() || (self.get_piece_num() as i32 - i as i32) < 1 {
            return Self {
                p: constants::ERROR,
            };
        }

        if self.get_piece_num() - i == 1 {
            return Self {
                p: ((self.p - i) & !constants::RED_FIVE) | constants::TERMINAL_BIT,
            };
        }

        Self {
            p: ((self.p - i) & !constants::RED_FIVE) & !constants::TERMINAL_BIT,
        }
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        (self.p | constants::RED_FIVE) == (other.p | constants::RED_FIVE)
    }
}

impl Eq for Piece {}

impl PartialOrd for Piece {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.p.cmp(&other.p))
    }
}

impl Ord for Piece {
    fn cmp(&self, other: &Self) -> Ordering {
        self.p.cmp(&other.p)
    }
}
