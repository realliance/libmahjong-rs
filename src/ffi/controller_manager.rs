use std::collections::HashMap;
use std::ffi::{c_char, CStr, CString};
use std::os::raw::c_void;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Mutex, OnceLock};

use super::player_controller::PlayerControllerFFI;

// FFI-compatible type for controller constructor
pub type NewControllerInstFn = unsafe extern "C" fn() -> *mut c_void;

// Type for the controller constructor function
type ControllerConstructor = Box<dyn Fn() -> Box<PlayerControllerFFI> + Send + Sync>;

// Our Rust-side controller manager
#[derive(Default)]
pub struct ControllerManagerImpl {
    available_controllers: HashMap<String, ControllerConstructor>,
}

impl ControllerManagerImpl {
    fn new() -> Self {
        Self {
            available_controllers: HashMap::new(),
        }
    }

    fn get_available_controllers(&self) -> Vec<String> {
        self.available_controllers.keys().cloned().collect()
    }

    fn new_controller(&self, controller: &str) -> Option<Box<PlayerControllerFFI>> {
        self.available_controllers
            .get(controller)
            .map(|constructor| constructor())
    }

    pub fn register_controller(&mut self, constructor: ControllerConstructor, name: &str) -> bool {
        if self.available_controllers.contains_key(name) {
            return false;
        }
        self.available_controllers
            .insert(name.to_string(), constructor);
        true
    }
}

// Global instance of the controller manager
pub static CONTROLLER_MANAGER: OnceLock<Mutex<ControllerManagerImpl>> = OnceLock::new();

pub fn controller_manager() -> &'static Mutex<ControllerManagerImpl> {
    CONTROLLER_MANAGER.get_or_init(init_controller_manager)
}

fn init_controller_manager() -> Mutex<ControllerManagerImpl> {
    Mutex::new(ControllerManagerImpl::new())
}

// FFI functions

#[no_mangle]
pub unsafe extern "C" fn controller_manager_get_available_controllers() -> *mut *mut c_char {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let manager = CONTROLLER_MANAGER
            .get_or_init(init_controller_manager)
            .lock()
            .unwrap();
        let controllers = manager.get_available_controllers();

        // Convert Vec<String> to C-compatible array of C strings
        let mut c_strings: Vec<*mut c_char> = controllers
            .into_iter()
            .map(|s| CString::new(s).unwrap().into_raw())
            .collect();

        // Add null terminator to indicate end of array
        c_strings.push(std::ptr::null_mut());

        // Allocate memory on the heap for the array
        let ptr = Box::into_raw(c_strings.into_boxed_slice()) as *mut *mut c_char;
        ptr
    }));

    match result {
        Ok(ptr) => ptr,
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn controller_manager_free_string_array(array: *mut *mut c_char) {
    if array.is_null() {
        return;
    }

    // Free each string in the array
    let mut current = array;
    while !(*current).is_null() {
        let _ = CString::from_raw(*current);
        current = current.add(1);
    }

    // Free the array itself
    let _ = Box::from_raw(array);
}

#[no_mangle]
pub unsafe extern "C" fn controller_manager_new_controller(
    controller_name: *const c_char,
) -> *mut c_void {
    if controller_name.is_null() {
        return std::ptr::null_mut();
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let controller_name = CStr::from_ptr(controller_name).to_str().unwrap();
        let manager = CONTROLLER_MANAGER
            .get_or_init(init_controller_manager)
            .lock()
            .unwrap();

        match manager.new_controller(controller_name) {
            Some(controller) => Box::into_raw(controller) as *mut c_void,
            None => std::ptr::null_mut(),
        }
    }));

    match result {
        Ok(ptr) => ptr,
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn controller_manager_register_controller(
    constructor: Option<NewControllerInstFn>,
    name: *const c_char,
) -> bool {
    if constructor.is_none() || name.is_null() {
        return false;
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let constructor_fn = constructor.unwrap();
        let name = CStr::from_ptr(name).to_str().unwrap();

        // Create a wrapper for the C++ constructor function
        let wrapped_constructor = Box::new(move || {
            let ptr = constructor_fn();
            if ptr.is_null() {
                panic!("Controller constructor returned null");
            }
            // Convert raw pointer to Box
            unsafe { Box::from_raw(ptr as *mut PlayerControllerFFI) }
        });

        let mut manager = CONTROLLER_MANAGER
            .get_or_init(init_controller_manager)
            .lock()
            .unwrap();

        manager.register_controller(wrapped_constructor, name)
    }));

    result.unwrap_or(false)
}

#[no_mangle]
pub unsafe extern "C" fn controller_manager_free_controller(controller: *mut c_void) {
    if !controller.is_null() {
        let _ = Box::from_raw(controller as *mut PlayerControllerFFI);
    }
}
