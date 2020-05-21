use std::os::raw::c_void;
use bass_sys::{BASS_Init, BASS_Free, BASS_ErrorGetCode};

fn main() {
        // Initialize bass with some default values
    // Only three first parameters are interesting for us
    // First parameter is device, -1 means bass will detect it automatically
    // Second parameter is frequency, 44000 is kinda default but it might be different
    // Third parameter are flags, you can find them in consts.rs (BASS_DEVICE_*)
    unsafe { BASS_Init(-1, 44000, 0, 0 as *mut c_void, 0 as *mut c_void); }

    // Get error code
    let mut error_code = unsafe { BASS_ErrorGetCode() };

    // If error code is 0 then everything is ok (BASS_OK), otherwise we should panic because something went wrong
    if error_code != 0 {
        panic!("Error code: {}", error_code);
    }

    // Free all memory allocated by bass
    unsafe { BASS_Free(); }

    error_code = unsafe { BASS_ErrorGetCode() };

    if error_code != 0 {
        panic!("Error code: {}", error_code);
    }
}