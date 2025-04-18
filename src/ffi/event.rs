use std::fmt::{self, Display};

use strum_macros::EnumString;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, strum_macros::Display)]
pub enum EventType {
    // out of hand events
    Ron,
    Kan,
    Pon,
    Chi,
    Decline,
    // in hand events
    Tsumo,
    ConcealedKan,
    ConvertedKan,
    Riichi,
    Discard,
    // other game events
    Dora,
    PointDiff,
    ExhaustiveDraw, // not used should be tho
    End,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Event {
    pub event_type: EventType,
    pub player: i32,
    pub piece: i16,
    pub decision: bool,
}

impl Event {
    pub const fn decline() -> Self {
        Event {
            event_type: EventType::Decline,
            player: -1,
            piece: 0,
            decision: false,
        }
    }

    pub fn action(event: EventType, piece: i16) -> Self {
        Event {
            event_type: event,
            player: 0,
            piece,
            decision: false,
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (player: {}, piece: {}, decision: {})",
            self.event_type.to_string(),
            self.player,
            self.piece,
            self.decision
        )
    }
}

// Constant definitions

pub const END_EVENT: Event = Event {
    event_type: EventType::End,
    player: -1,
    piece: 0,
    decision: false,
};
