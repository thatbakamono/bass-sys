use libloading::Library;
use once_cell::sync::{Lazy, OnceCell};

use crate::{generate_bindings, types::{
    Bass3DVector, BassChannelInfo, BassDeviceInfo, BassFileProcs, BassInfo, BassPluginInfo,
    BassRecordInfo, BassSample, BOOL, DOWNLOADPROC, DSPPROC, DWORD, HCHANNEL, HDSP, HFX, HMUSIC,
    HPLUGIN, HRECORD, HSAMPLE, HSTREAM, HSYNC, QWORD, RECORDPROC, STREAMPROC, SYNCPROC,
}};
use std::{env, os::raw::{c_char, c_int, c_void}, path::PathBuf};

static BASS_LIBRARY_NAME: OnceCell<String> = OnceCell::new();
static BASS_LIBRARY_SEARCH_PATHS: OnceCell<Vec<PathBuf>> = OnceCell::new();

static BASS_LIBRARY: Lazy<Library> = Lazy::new(|| {
    let library_name = BASS_LIBRARY_NAME.get_or_init(|| {
        #[cfg(target_os = "windows")]
        return String::from("bass.dll");

        #[cfg(target_os = "linux")]
        return String::from("libbass.so");

        #[cfg(target_os = "macos")]
        return String::from("libbass.dylib");
    });

    let library_search_paths = BASS_LIBRARY_SEARCH_PATHS.get_or_init(|| {
        if let Ok(mut current_directory) = env::current_exe() {
            current_directory.pop();

            return vec![current_directory];
        } else {
            panic!("Failed to retrieve current working directory, can't initialize library search paths.");
        }
    });

    for library_search_path in library_search_paths {
        let library_path = library_search_path.join(library_name);

        if library_path.exists() && library_path.is_file() {
            if let Ok(library) = unsafe { Library::new(library_path) } {
                return library;
            } else {
                panic!("Failed to load the library.");
            }
        }
    }

    panic!("Couldn't find the library.");
});
 
/// This function sets the library name. 
/// If not called manually, the library name and search paths are automatically set to the default values by the time you call any Bass function. 
/// If you wish to set them manually, do it before calling anything else.
///
/// Returns `Ok(())` if the library name wasn't already set, otherwise it returns `Err(name)`
///
/// The library name is used to find the dynamic library file of this name in paths set by `set_library_search_paths`. 
/// It should include the file extension as well (for example: `bass.dll`).
///
/// Note that the dynamic-link library file name is usually dependant on the currently run operating system, 
/// so in a multiplatform context one should use cfg to set the according library name.
///
/// For example:
///
/// ```no_run
/// #[cfg(target_os = "windows")]
/// bass_sys::set_library_name(String::from("bass.dll"));
///
/// #[cfg(target_os = "linux")]
/// bass_sys::set_library_name(String::from("libbass.so"));
///
/// #[cfg(target_os = "macos")]
/// bass_sys::set_library_name(String::from("libbass.dylib"));
/// ```
pub fn set_library_name(name: String) -> Result<(), String> {
    BASS_LIBRARY_NAME.set(name)
}

/// This function sets the library search paths.
/// If not called manually, the library name and search paths are automatically set to the default values by the time you call any Bass function. 
/// If you wish to set them manually, do it before calling anything else.
///
/// Returns `Ok(())` if the library search paths weren't already set, otherwise it returns `Err(search_paths)`
///
/// The library search paths are used to find the dynamic library file of the name set by `set_library_name`.
pub fn set_library_search_paths(search_paths: Vec<PathBuf>) -> Result<(), Vec<PathBuf>> {
    BASS_LIBRARY_SEARCH_PATHS.set(search_paths)
}

generate_bindings! {
    binding BASS_SET_CONFIG fn BASS_SetConfig(option: DWORD, value: DWORD) -> BOOL;
    binding BASS_GET_CONFIG fn BASS_GetConfig(option: DWORD) -> DWORD;
    binding BASS_SET_CONFIG_PTR fn BASS_SetConfigPtr(option: DWORD, value: *mut c_void) -> BOOL;
    binding BASS_GET_CONFIG_PTR fn BASS_GetConfigPtr(option: DWORD) -> *mut c_void;
    binding BASS_GET_VERSION fn BASS_GetVersion() -> DWORD;
    binding BASS_ERROR_GET_CODE fn BASS_ErrorGetCode() -> c_int;
    binding BASS_GET_DEVICE_INFO fn BASS_GetDeviceInfo(device: DWORD, info: *mut BassDeviceInfo) -> BOOL;
    binding BASS_INIT fn BASS_Init(
        device: c_int,
        frequency: DWORD,
        flags: DWORD,
        window: *mut c_void,
        dsguid: *mut c_void
    ) -> BOOL;
    binding BASS_SET_DEVICE fn BASS_SetDevice(device: DWORD) -> BOOL;
    binding BASS_GET_DEVICE fn BASS_GetDevice() -> DWORD;
    binding BASS_FREE fn BASS_Free() -> BOOL;
    binding BASS_GET_INFO fn BASS_GetInfo(info: *mut BassInfo) -> BOOL;
    binding BASS_UPDATE fn BASS_Update(length: DWORD) -> BOOL;
    binding BASS_GET_CPU fn BASS_GetCPU() -> f32;
    binding BASS_START fn BASS_Start() -> BOOL;
    binding BASS_STOP fn BASS_Stop() -> BOOL;
    binding BASS_PAUSE fn BASS_Pause() -> BOOL;
    binding BASS_IS_STARTED fn BASS_IsStarted() -> DWORD;
    binding BASS_SET_VOLUME fn BASS_SetVolume(value: f32) -> BOOL;
    binding BASS_GET_VOLUME fn BASS_GetVolume() -> f32;
    binding BASS_PLUGIN_LOAD fn BASS_PluginLoad(file: *const c_void, flags: DWORD) -> HPLUGIN;
    binding BASS_PLUGIN_FREE fn BASS_PluginFree(handle: HPLUGIN) -> BOOL;
    binding BASS_PLUGIN_ENABLE fn BASS_PluginEnable(handle: HPLUGIN, enable: BOOL) -> BOOL;
    binding BASS_PLUGIN_GET_INFO fn BASS_PluginGetInfo(handle: HPLUGIN) -> *mut BassPluginInfo;
    binding BASS_SET_3D_FACTORS fn BASS_Set3DFactors(distance: f32, roll: f32, doppler_factor: f32) -> BOOL;
    binding BASS_GET_3D_FACTORS fn BASS_Get3DFactors(distance: *mut f32, roll: *mut f32, doppler_factor: *mut f32) -> BOOL;
    binding BASS_SET_3D_POSITION fn BASS_Set3DPosition(
        position: *const Bass3DVector,
        velocity: *const Bass3DVector,
        front: *const Bass3DVector,
        top: *const Bass3DVector
    ) -> BOOL;
    binding BASS_GET_3D_POSITION fn BASS_Get3DPosition(
        position: *mut Bass3DVector,
        velocity: *mut Bass3DVector,
        front: *mut Bass3DVector,
        top: *mut Bass3DVector
    ) -> BOOL;
    binding BASS_APPLY_3D fn BASS_Apply3D();
    binding BASS_MUSIC_LOAD fn BASS_MusicLoad(
        memory: BOOL,
        file: *const c_void,
        offset: QWORD,
        length: DWORD,
        flags: DWORD,
        frequency: DWORD
    ) -> HMUSIC;
    binding BASS_MUSIC_FREE fn BASS_MusicFree(handle: HMUSIC) -> BOOL;
    binding BASS_SAMPLE_LOAD fn BASS_SampleLoad(
        memory: BOOL,
        file: *const c_void,
        offset: QWORD,
        length: DWORD,
        maximum: DWORD,
        flags: DWORD
    ) -> HSAMPLE;
    binding BASS_SAMPLE_CREATE fn BASS_SampleCreate(
        length: DWORD,
        frequency: DWORD,
        channels: DWORD,
        maximum: DWORD,
        flags: DWORD
    ) -> HSAMPLE;
    binding BASS_SAMPLE_FREE fn BASS_SampleFree(handle: HSAMPLE) -> BOOL;
    binding BASS_SAMPLE_SET_DATA fn BASS_SampleSetData(handle: HSAMPLE, buffer: *const c_void) -> BOOL;
    binding BASS_SAMPLE_GET_DATA fn BASS_SampleGetData(handle: HSAMPLE, buffer: *mut c_void) -> BOOL;
    binding BASS_SAMPLE_GET_INFO fn BASS_SampleGetInfo(handle: HSAMPLE, info: *mut BassSample) -> BOOL;
    binding BASS_SAMPLE_SET_INFO fn BASS_SampleSetInfo(handle: HSAMPLE, info: *const BassSample) -> BOOL;
    binding BASS_SAMPLE_GET_CHANNEL fn BASS_SampleGetChannel(handle: HSAMPLE, flags: DWORD) -> HCHANNEL;
    binding BASS_SAMPLE_GET_CHANNELS fn BASS_SampleGetChannels(handle: HSAMPLE, channels: *mut HCHANNEL) -> DWORD;
    binding BASS_SAMPLE_STOP fn BASS_SampleStop(handle: HSAMPLE) -> BOOL;
    binding BASS_STREAM_CREATE fn BASS_StreamCreate(
        frequency: DWORD,
        channels: DWORD,
        flags: DWORD,
        proc: *mut STREAMPROC,
        user: *mut c_void
    ) -> HSTREAM;
    binding BASS_STREAM_CREATE_FILE fn BASS_StreamCreateFile(
        memory: BOOL,
        file: *const c_void,
        offset: QWORD,
        length: QWORD,
        flags: DWORD
    ) -> HSTREAM;
    binding BASS_STREAM_CREATE_URL fn BASS_StreamCreateURL(
        url: *const c_char,
        offset: DWORD,
        flags: DWORD,
        proc: *mut DOWNLOADPROC,
        user: *mut c_void
    ) -> HSTREAM;
    binding BASS_STREAM_CREATE_FILE_USER fn BASS_StreamCreateFileUser(
        system: DWORD,
        flags: DWORD,
        proc: *mut BassFileProcs,
        user: *mut c_void
    ) -> HSTREAM;
    binding BASS_STREAM_FREE fn BASS_StreamFree(handle: HSTREAM) -> BOOL;
    binding BASS_STREAM_GET_FILE_POSITION fn BASS_StreamGetFilePosition(handle: HSTREAM, mode: DWORD) -> QWORD;
    binding BASS_STREAM_PUT_DATA fn BASS_StreamPutData(handle: HSTREAM, buffer: *const c_void, length: DWORD) -> DWORD;
    binding BASS_STREAM_PUT_FILE_DATA fn BASS_StreamPutFileData(handle: HSTREAM, buffer: *const c_void, length: DWORD) -> DWORD;
    binding BASS_RECORD_GET_DEVICE_INFO fn BASS_RecordGetDeviceInfo(device: DWORD, info: *mut BassDeviceInfo) -> BOOL;
    binding BASS_RECORD_INIT fn BASS_RecordInit(device: c_int) -> BOOL;
    binding BASS_RECORD_SET_DEVICE fn BASS_RecordSetDevice(device: DWORD) -> BOOL;
    binding BASS_RECORD_GET_DEVICE fn BASS_RecordGetDevice() -> DWORD;
    binding BASS_RECORD_FREE fn BASS_RecordFree() -> BOOL;
    binding BASS_RECORD_GET_INFO fn BASS_RecordGetInfo(info: *mut BassRecordInfo) -> BOOL;
    binding BASS_RECORD_GET_INPUT_NAME fn BASS_RecordGetInputName(input: c_int) -> *const c_void;
    binding BASS_RECORD_SET_INPUT fn BASS_RecordSetInput(input: c_int, flags: DWORD, volume: f32) -> BOOL;
    binding BASS_RECORD_GET_INPUT fn BASS_RecordGetInput(input: c_int, volume: *mut f32) -> DWORD;
    binding BASS_RECORD_START fn BASS_RecordStart(
        frequency: DWORD,
        channels: DWORD,
        flags: DWORD,
        proc: *mut RECORDPROC,
        user: *mut c_void
    ) -> HRECORD;
    binding BASS_CHANNEL_BYTES_TO_SECONDS fn BASS_ChannelBytes2Seconds(handle: DWORD, position: QWORD) -> f64;
    binding BASS_CHANNEL_SECONDS_TO_BYTES fn BASS_ChannelSeconds2Bytes(handle: DWORD, position: f64) -> QWORD;
    binding BASS_CHANNEL_GET_DEVICE fn BASS_ChannelGetDevice(handle: DWORD) -> DWORD;
    binding BASS_CHANNEL_SET_DEVICE fn BASS_ChannelSetDevice(handle: DWORD, device: DWORD) -> BOOL;
    binding BASS_CHANNEL_IS_ACTIVE fn BASS_ChannelIsActive(handle: DWORD) -> DWORD;
    binding BASS_CHANNEL_GET_INFO fn BASS_ChannelGetInfo(handle: DWORD, info: *mut BassChannelInfo) -> BOOL;
    binding BASS_CHANNEL_GET_TAGS fn BASS_ChannelGetTags(handle: DWORD, tags: DWORD) -> *const c_char;
    binding BASS_CHANNEL_FLAGS fn BASS_ChannelFlags(handle: DWORD, flags: DWORD, mask: DWORD) -> DWORD;
    binding BASS_CHANNEL_UPDATE fn BASS_ChannelUpdate(handle: DWORD, length: DWORD) -> BOOL;
    binding BASS_CHANNEL_LOCK fn BASS_ChannelLock(handle: DWORD, lock: BOOL) -> BOOL;
    binding BASS_CHANNEL_FREE fn BASS_ChannelFree(handle: DWORD) -> BOOL;
    binding BASS_CHANNEL_PLAY fn BASS_ChannelPlay(handle: DWORD, restart: BOOL) -> BOOL;
    binding BASS_CHANNEL_STOP fn BASS_ChannelStop(handle: DWORD) -> BOOL;
    binding BASS_CHANNEL_PAUSE fn BASS_ChannelPause(handle: DWORD) -> BOOL;
    binding BASS_CHANNEL_SET_ATTRIBUTE fn BASS_ChannelSetAttribute(handle: DWORD, attribute: DWORD, value: f32) -> BOOL;
    binding BASS_CHANNEL_GET_ATTRIBUTE fn BASS_ChannelGetAttribute(handle: DWORD, attribute: DWORD, value: *mut f32) -> BOOL;
    binding BASS_CHANNEL_SLIDE_ATTRIBUTE fn BASS_ChannelSlideAttribute(handle: DWORD, attribute: DWORD, value: f32, time: DWORD)-> BOOL;
    binding BASS_CHANNEL_IS_SLIDING fn BASS_ChannelIsSliding(handle: DWORD, attribute: DWORD) -> BOOL;
    binding BASS_CHANNEL_SET_ATTRIBUTE_EX fn BASS_ChannelSetAttributeEx(
        handle: DWORD,
        attribute: DWORD,
        value: *mut c_void,
        size: DWORD
    ) -> BOOL;
    binding BASS_CHANNEL_GET_ATTRIBUTE_EX fn BASS_ChannelGetAttributeEx(
        handle: DWORD,
        attribute: DWORD,
        value: *mut c_void,
        size: DWORD
    ) -> DWORD;
    binding BASS_CHANNEL_SET_3D_ATTRIBUTES fn BASS_ChannelSet3DAttributes(
        handle: DWORD,
        mode: c_int,
        minimum: f32,
        maximum: f32,
        iangle: c_int,
        oangle: c_int,
        out_volume: f32
    ) -> BOOL;
    binding BASS_CHANNEL_GET_3D_ATTRIBUTES fn BASS_ChannelGet3DAttributes(
        handle: DWORD,
        mode: *mut DWORD,
        minimum: *mut f32,
        maximum: *mut f32,
        angle_of_inside_projection_cone: *mut DWORD,
        angle_of_outside_projection_cone: *mut DWORD,
        output_volume: *mut f32
    ) -> BOOL;
    binding BASS_CHANNEL_SET_3D_POSITION fn BASS_ChannelSet3DPosition(
        handle: DWORD,
        position: *const Bass3DVector,
        orientation: *const Bass3DVector,
        velocity: *const Bass3DVector
    ) -> BOOL;
    binding BASS_CHANNEL_GET_3D_POSITION fn BASS_ChannelGet3DPosition(
        handle: DWORD,
        position: *mut Bass3DVector,
        orientation: *mut Bass3DVector,
        velocity: *mut Bass3DVector
    ) -> BOOL;
    binding BASS_CHANNEL_GET_LENGTH fn BASS_ChannelGetLength(handle: DWORD, mode: DWORD) -> QWORD;
    binding BASS_CHANNEL_SET_POSITION fn BASS_ChannelSetPosition(handle: DWORD, position: QWORD, mode: DWORD) -> BOOL;
    binding BASS_CHANNEL_GET_POSITION fn BASS_ChannelGetPosition(handle: DWORD, mode: DWORD) -> QWORD;
    binding BASS_CHANNEL_GET_LEVEL fn BASS_ChannelGetLevel(handle: DWORD) -> DWORD;
    binding BASS_CHANNEL_GET_LEVEL_EX fn BASS_ChannelGetLevelEx(handle: DWORD, levels: *mut f32, length: f32, flags: DWORD) -> BOOL;
    binding BASS_CHANNEL_GET_DATA fn BASS_ChannelGetData(handle: DWORD, buffer: *mut c_void, length: DWORD) -> DWORD;
    binding BASS_CHANNEL_SET_SYNC fn BASS_ChannelSetSync(
        handle: DWORD,
        sync_type: DWORD,
        parameter: QWORD,
        proc: *mut SYNCPROC,
        user: *mut c_void
    ) -> HSYNC;
    binding BASS_CHANNEL_REMOVE_SYNC fn BASS_ChannelRemoveSync(handle: DWORD, sync: HSYNC) -> BOOL;
    binding BASS_CHANNEL_SET_DSP fn BASS_ChannelSetDSP(handle: DWORD, proc: *mut DSPPROC, user: *mut c_void, priority: c_int) -> HDSP;
    binding BASS_CHANNEL_REMOVE_DSP fn BASS_ChannelRemoveDSP(handle: DWORD, dsp: HDSP) -> BOOL;
    binding BASS_CHANNEL_SET_LINK fn BASS_ChannelSetLink(handle: DWORD, channel: DWORD) -> BOOL;
    binding BASS_CHANNEL_REMOVE_LINK fn BASS_ChannelRemoveLink(handle: DWORD, channel: DWORD) -> BOOL;
    binding BASS_CHANNEL_SET_FX fn BASS_ChannelSetFX(handle: DWORD, fx_type: DWORD, priority: c_int) -> HFX;
    binding BASS_CHANNEL_REMOVE_FX fn BASS_ChannelRemoveFX(handle: DWORD, fx: HFX) -> BOOL;
    binding BASS_FX_SET_PARAMETERS fn BASS_FxSetParameters(handle: HFX, parameters: *const c_void) -> BOOL;
    binding BASS_FX_GET_PARAMETERS fn BASS_FXGetParameters(handle: HFX, parameters: *mut c_void) -> BOOL;
    binding BASS_FX_RESET fn BASS_FXReset(handle: HFX);
    binding BASS_FX_SET_PRIORITY fn BASS_FXSetPriority(handle: HFX, priority: c_int) -> BOOL;
}
