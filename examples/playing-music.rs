use std::os::raw::c_void;
use std::{io, convert::TryFrom};
use std::io::prelude::*;
use bass_sys::{BassString, BASS_Init, BASS_Free, BASS_StreamCreateFile, BASS_StreamFree, BASS_ChannelPlay, BASS_ErrorGetCode, BASS_UNICODE};

fn main() {
    // Explained in initializing-bass
    unsafe { BASS_Init(-1, 44000, 0, 0 as *mut c_void, 0 as *mut c_void); }

    // Explained in initializing-bass
    let mut error_code = unsafe { BASS_ErrorGetCode() };

    // Explained in initializing-bass
    if error_code != 0 {
        panic!("Error code: {}", error_code);
    }

    // Keep in mind this path is relative and will most likely point to a project directory
    let path = BassString::try_from("test.mp3").unwrap();
    // Create a stream for a specified file (mp3 is supported by default, if you want to use another magic format, you have to install a proper bass plugin in the first place)
    let handle = unsafe { BASS_StreamCreateFile(0, path, 0, 0, BASS_UNICODE) };

    error_code = unsafe { BASS_ErrorGetCode() };

    if error_code != 0 {
        panic!("Error code: {}", error_code);
    }

    // Start playing music!!!!!!
    unsafe { BASS_ChannelPlay(handle, 0); }

    error_code = unsafe { BASS_ErrorGetCode() };

    if error_code != 0 {
        panic!("Error code: {}", error_code);
    }

    // Wait for any user input
    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();

    // Free the stream allocated with BASS_StreamCreateFile
    unsafe { BASS_StreamFree(handle) };

    // Explained in initializing-bass
    unsafe { BASS_Free(); }

    error_code = unsafe { BASS_ErrorGetCode() };

    if error_code != 0 {
        panic!("Error code: {}", error_code);
    }
}