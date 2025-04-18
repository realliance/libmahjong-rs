use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::slice;

use crate::ffi::event::Event;
use crate::ffi::piece::Piece;
use crate::ffi::wind::Wind;

/// Rust trait representing the C++ PlayerController interface
pub trait PlayerController {
    /// Called when a new game starts
    fn game_start(&mut self, player_id: i32);

    /// Called at the start of a new round
    fn round_start(&mut self, hand: Vec<Piece>, seat_wind: Wind, prevalent_wind: Wind);

    /// Called when an event occurs
    fn receive_event(&mut self, event: Event);

    /// Returns the player's decision as an Event
    fn retrieve_decision(&mut self) -> Event;

    /// Returns the player's name
    fn name(&self) -> String;
}

#[repr(C)]
pub struct PlayerControllerFFI {
    /// Opaque pointer to the concrete PlayerController implementation
    context: *mut libc::c_void,

    /// Function pointers to the implementation's methods
    game_start: unsafe extern "C" fn(context: *mut libc::c_void, player_id: c_int),
    round_start: unsafe extern "C" fn(
        context: *mut libc::c_void,
        hand: *const Piece,
        hand_size: c_int,
        seat_wind: Wind,
        prevalent_wind: Wind,
    ),
    receive_event: unsafe extern "C" fn(context: *mut libc::c_void, event: Event),
    retrieve_decision: unsafe extern "C" fn(context: *mut libc::c_void) -> Event,
    name: unsafe extern "C" fn(context: *mut libc::c_void) -> *const c_char,
    free: unsafe extern "C" fn(context: *mut libc::c_void),
}

impl PlayerControllerFFI {
    pub fn game_start(&self, player_id: i32) {
        unsafe {
            (self.game_start)(self.context, player_id as c_int);
        }
    }

    pub fn round_start(&self, hand: &[Piece], seat_wind: Wind, prevalent_wind: Wind) {
        unsafe {
            (self.round_start)(
                self.context,
                hand.as_ptr(),
                hand.len() as c_int,
                seat_wind,
                prevalent_wind,
            );
        }
    }

    pub fn receive_event(&self, event: Event) {
        unsafe {
            (self.receive_event)(self.context, event);
        }
    }

    pub fn retrieve_decision(&self) -> Event {
        unsafe { (self.retrieve_decision)(self.context) }
    }

    pub fn name(&self) -> String {
        unsafe {
            let c_str = CStr::from_ptr((self.name)(self.context));
            c_str.to_string_lossy().into_owned()
        }
    }
}

impl Drop for PlayerControllerFFI {
    fn drop(&mut self) {
        unsafe {
            (self.free)(self.context);
        }
    }
}

/// Helper to create a PlayerControllerFFI from a Rust implementation
pub fn create_player_controller_ffi<T: PlayerController + 'static>(
    controller: T,
) -> PlayerControllerFFI {
    extern "C" fn game_start_trampoline<T: PlayerController>(
        context: *mut libc::c_void,
        player_id: c_int,
    ) {
        let controller = unsafe { &mut *(context as *mut T) };
        controller.game_start(player_id as i32);
    }

    extern "C" fn round_start_trampoline<T: PlayerController>(
        context: *mut libc::c_void,
        hand_ptr: *const Piece,
        hand_size: c_int,
        seat_wind: Wind,
        prevalent_wind: Wind,
    ) {
        let controller = unsafe { &mut *(context as *mut T) };
        let hand = unsafe { slice::from_raw_parts(hand_ptr, hand_size as usize) }.to_vec();
        controller.round_start(hand, seat_wind, prevalent_wind);
    }

    extern "C" fn receive_event_trampoline<T: PlayerController>(
        context: *mut libc::c_void,
        event: Event,
    ) {
        let controller = unsafe { &mut *(context as *mut T) };
        controller.receive_event(event);
    }

    extern "C" fn retrieve_decision_trampoline<T: PlayerController>(
        context: *mut libc::c_void,
    ) -> Event {
        let controller = unsafe { &mut *(context as *mut T) };
        controller.retrieve_decision()
    }

    extern "C" fn name_trampoline<T: PlayerController>(
        context: *mut libc::c_void,
    ) -> *const c_char {
        let controller = unsafe { &*(context as *mut T) };
        let name = controller.name();
        let c_name = CString::new(name).unwrap_or_else(|_| CString::new("").unwrap());
        c_name.into_raw() as *const c_char
    }

    extern "C" fn free_trampoline<T: PlayerController>(context: *mut libc::c_void) {
        if !context.is_null() {
            unsafe {
                let _ = Box::from_raw(context as *mut T);
            }
        }
    }

    let context = Box::into_raw(Box::new(controller)) as *mut libc::c_void;

    PlayerControllerFFI {
        context,
        game_start: game_start_trampoline::<T>,
        round_start: round_start_trampoline::<T>,
        receive_event: receive_event_trampoline::<T>,
        retrieve_decision: retrieve_decision_trampoline::<T>,
        name: name_trampoline::<T>,
        free: free_trampoline::<T>,
    }
}
