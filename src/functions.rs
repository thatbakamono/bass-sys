use crate::types::{
    Bass3DVector, BassChannelInfo, BassDeviceInfo, BassFileProcs, BassInfo, BassPluginInfo,
    BassRecordInfo, BassSample, BOOL, DOWNLOADPROC, DSPPROC, DWORD, HCHANNEL, HDSP, HFX, HMUSIC,
    HPLUGIN, HRECORD, HSAMPLE, HSTREAM, HSYNC, QWORD, RECORDPROC, STREAMPROC, SYNCPROC,
};
use std::os::raw::{c_char, c_int, c_void};

#[allow(dead_code)]
extern "C" {
    pub fn BASS_SetConfig(option: DWORD, value: DWORD) -> BOOL;
    pub fn BASS_GetConfig(option: DWORD) -> DWORD;
    pub fn BASS_SetConfigPtr(option: DWORD, value: *mut c_void) -> BOOL;
    pub fn BASS_GetConfigPtr(option: DWORD) -> *mut c_void;
    pub fn BASS_GetVersion() -> DWORD;
    pub fn BASS_ErrorGetCode() -> c_int;
    pub fn BASS_GetDeviceInfo(device: DWORD, info: *mut BassDeviceInfo) -> BOOL;
    pub fn BASS_Init(
        device: c_int,
        frequency: DWORD,
        flags: DWORD,
        window: *mut c_void,
        dsguid: *mut c_void,
    ) -> BOOL;
    pub fn BASS_SetDevice(device: DWORD) -> BOOL;
    pub fn BASS_GetDevice() -> DWORD;
    pub fn BASS_Free() -> BOOL;
    pub fn BASS_GetInfo(info: *mut BassInfo) -> BOOL;
    pub fn BASS_Update(length: DWORD) -> BOOL;
    pub fn BASS_GetCPU() -> f32;
    pub fn BASS_Start() -> BOOL;
    pub fn BASS_Stop() -> BOOL;
    pub fn BASS_Pause() -> BOOL;
    pub fn BASS_IsStarted() -> BOOL;
    pub fn BASS_SetVolume(value: f32) -> BOOL;
    pub fn BASS_GetVolume() -> f32;
    pub fn BASS_PluginLoad(file: *mut c_char, flags: DWORD) -> HPLUGIN;
    pub fn BASS_PluginFree(handle: HPLUGIN) -> BOOL;
    pub fn BASS_PluginGetInfo(handle: HPLUGIN) -> *mut BassPluginInfo;
    pub fn BASS_Set3DFactors(distance: f32, roll: f32, doppler_factor: f32) -> BOOL;
    pub fn BASS_Get3DFactors(distance: f32, roll: f32, doppler_factor: f32) -> BOOL;
    pub fn BASS_Set3DPosition(
        position: *const Bass3DVector,
        velocity: *const Bass3DVector,
        front: *const Bass3DVector,
        top: *const Bass3DVector,
    ) -> BOOL;
    pub fn BASS_Get3DPosition(
        position: *mut Bass3DVector,
        velocity: *mut Bass3DVector,
        front: *mut Bass3DVector,
        top: *mut Bass3DVector,
    ) -> BOOL;
    pub fn BASS_Apply3D();
    pub fn BASS_SetEAXParameters(environment: c_int, volume: f32, decay: f32, damp: f32) -> BOOL;
    pub fn BASS_GetEAXParameters(
        environment: *mut DWORD,
        volume: *mut f32,
        decay: *mut f32,
        damp: *mut f32,
    ) -> BOOL;
    pub fn BASS_MusicLoad(
        memory: BOOL,
        file: *const c_char,
        offset: QWORD,
        length: DWORD,
        flags: DWORD,
        frequency: DWORD,
    ) -> HMUSIC;
    pub fn BASS_MusicFree(handle: HMUSIC) -> BOOL;
    pub fn BASS_SampleLoad(
        memory: BOOL,
        file: *const c_char,
        offset: QWORD,
        length: DWORD,
        maximum: DWORD,
        flags: DWORD,
    ) -> HSAMPLE;
    pub fn BASS_SampleCreate(
        length: DWORD,
        frequency: DWORD,
        channels: DWORD,
        maximum: DWORD,
        flags: DWORD,
    ) -> HSAMPLE;
    pub fn BASS_SampleFree(handle: HSAMPLE) -> BOOL;
    pub fn BASS_SampleSetData(handle: HSAMPLE, buffer: *const c_void) -> BOOL;
    pub fn BASS_SampleGetData(handle: HSAMPLE, buffer: *mut c_void) -> BOOL;
    pub fn BASS_SampleGetInfo(handle: HSAMPLE, info: *mut BassSample) -> BOOL;
    pub fn BASS_SampleSetInfo(handle: HSAMPLE, info: *const BassSample) -> BOOL;
    pub fn BASS_SampleGetChannel(handle: HSAMPLE, only_new: BOOL) -> HCHANNEL;
    pub fn BASS_SampleGetChannels(handle: HSAMPLE, channels: *mut HCHANNEL) -> DWORD;
    pub fn BASS_SampleStop(handle: HSAMPLE) -> BOOL;
    pub fn BASS_StreamCreate(
        frequency: DWORD,
        channels: DWORD,
        flags: DWORD,
        proc: *mut STREAMPROC,
        user: *mut c_void,
    ) -> HSTREAM;
    pub fn BASS_StreamCreateFile(
        memory: BOOL,
        file: *const c_char,
        offset: QWORD,
        length: QWORD,
        flags: DWORD,
    ) -> HSTREAM;
    pub fn BASS_StreamCreateURL(
        url: *const c_char,
        offset: DWORD,
        flags: DWORD,
        proc: *mut DOWNLOADPROC,
        user: *mut c_void,
    ) -> HSTREAM;
    pub fn BASS_StreamCreateFileUser(
        system: DWORD,
        flags: DWORD,
        proc: *mut BassFileProcs,
        user: *mut c_void,
    ) -> HSTREAM;
    pub fn BASS_StreamFree(handle: HSTREAM) -> BOOL;
    pub fn BASS_StreamGetFilePosition(handle: HSTREAM, mode: DWORD) -> QWORD;
    pub fn BASS_StreamPutData(handle: HSTREAM, buffer: *const c_void, length: DWORD) -> DWORD;
    pub fn BASS_StreamPutFileData(handle: HSTREAM, buffer: *const c_void, length: DWORD) -> DWORD;
    pub fn BASS_RecordGetDevicoInfo(device: DWORD, info: *mut BassDeviceInfo) -> BOOL;
    pub fn BASS_RecordInit(device: c_int) -> BOOL;
    pub fn BASS_RecordSetDevice(device: DWORD) -> BOOL;
    pub fn BASS_RecordGetDevice() -> DWORD;
    pub fn BASS_RecordFree() -> BOOL;
    pub fn BASS_RecordGetInfo(info: *mut BassRecordInfo) -> BOOL;
    pub fn BASS_RecordGetInputName(input: c_int) -> *const c_char;
    pub fn BASS_RecordSetInput(input: c_int, flags: DWORD, volume: f32) -> BOOL;
    pub fn BASS_RecordGetInput(input: c_int, volume: *mut f32) -> DWORD;
    pub fn BASS_RecordStart(
        frequency: DWORD,
        channels: DWORD,
        flags: DWORD,
        proc: *mut RECORDPROC,
        user: *mut c_void,
    ) -> HRECORD;
    pub fn BASS_ChannelBytes2Seconds(handle: DWORD, position: QWORD) -> f64;
    pub fn BASS_ChannelSeconds2Bytes(handle: DWORD, position: f64) -> QWORD;
    pub fn BASS_ChannelGetDevice(handle: DWORD) -> DWORD;
    pub fn BASS_ChannelSetDevice(handle: DWORD, device: DWORD) -> BOOL;
    pub fn BASS_ChannelIsActive(handle: DWORD) -> DWORD;
    pub fn BASS_ChannelGetInfo(handle: DWORD, info: *mut BassChannelInfo) -> BOOL;
    pub fn BASS_ChannelGetTags(handle: DWORD, tags: DWORD) -> *const c_char;
    pub fn BASS_ChannelFlags(handle: DWORD, flags: DWORD, mask: DWORD) -> DWORD;
    pub fn BASS_ChannelUpdate(handle: DWORD, length: DWORD) -> BOOL;
    pub fn BASS_ChannelLock(handle: DWORD, lock: BOOL) -> BOOL;
    pub fn BASS_ChannelPlay(handle: DWORD, restart: BOOL) -> BOOL;
    pub fn BASS_ChannelStop(handle: DWORD) -> BOOL;
    pub fn BASS_ChannelPause(handle: DWORD) -> BOOL;
    pub fn BASS_ChannelSetAttribute(handle: DWORD, attribute: DWORD, value: f32) -> BOOL;
    pub fn BASS_ChannelGetAttribute(handle: DWORD, attribute: DWORD, value: *mut f32) -> BOOL;
    pub fn BASS_ChannelSlideAttribute(handle: DWORD, attribute: DWORD, value: f32, time: DWORD)
        -> BOOL;
    pub fn BASS_ChannelIsSliding(handle: DWORD, attribute: DWORD) -> BOOL;
    pub fn BASS_ChannelSetAttributeEx(
        handle: DWORD,
        attribute: DWORD,
        value: *mut c_void,
        size: DWORD,
    ) -> BOOL;
    pub fn BASS_ChannelGetAttributeEx(
        handle: DWORD,
        attribute: DWORD,
        value: *mut c_void,
        size: DWORD,
    ) -> BOOL;
    pub fn BASS_ChannelSet3DAttributes(
        handle: DWORD,
        mode: c_int,
        minimum: f32,
        maximum: f32,
        iangle: c_int,
        oangle: c_int,
        out_volume: f32,
    ) -> BOOL;
    pub fn BASS_ChannelGet3DAttributes(
        handle: DWORD,
        mode: *mut DWORD,
        minimum: *mut f32,
        maximum: *mut f32,
        angle_of_inside_projection_cone: *mut DWORD,
        angle_of_outside_projection_cone: *mut DWORD,
        output_volume: *mut f32,
    ) -> BOOL;
    pub fn BASS_ChannelSet3DPosition(
        handle: DWORD,
        position: *const Bass3DVector,
        orientation: *const Bass3DVector,
        velocity: *const Bass3DVector,
    ) -> BOOL;
    pub fn BASS_ChannelGet3DPosition(
        handle: DWORD,
        position: *mut Bass3DVector,
        orientation: *mut Bass3DVector,
        velocity: *mut Bass3DVector,
    ) -> BOOL;
    pub fn BASS_ChannelGetLength(handle: DWORD, mode: DWORD) -> QWORD;
    pub fn BASS_ChannelSetPosition(handle: DWORD, position: QWORD, mode: DWORD) -> BOOL;
    pub fn BASS_ChannelGetPosition(handle: DWORD, mode: DWORD) -> QWORD;
    pub fn BASS_ChannelGetLevel(handle: DWORD) -> DWORD;
    pub fn BASS_ChannelGetLevelEx(handle: DWORD, levels: *mut f32, length: f32, flags: DWORD) -> BOOL;
    pub fn BASS_ChannelGetData(handle: DWORD, buffer: *mut c_void, length: DWORD) -> DWORD;
    pub fn BASS_ChannelSetSync(
        handle: DWORD,
        sync_type: DWORD,
        parameter: QWORD,
        proc: *mut SYNCPROC,
        user: *mut c_void,
    ) -> HSYNC;
    pub fn BASS_ChannelRemoveSync(handle: DWORD, sync: HSYNC) -> BOOL;
    pub fn BASS_ChannelSetDSP(handle: DWORD, proc: *mut DSPPROC, user: *mut c_void, priority: c_int);
    pub fn BASS_ChannelRemoveDSP(handle: DWORD, dsp: HDSP) -> BOOL;
    pub fn BASS_ChannelSetLink(handle: DWORD, channel: DWORD) -> BOOL;
    pub fn BASS_ChannelRemoveLink(handle: DWORD, channel: DWORD) -> BOOL;
    pub fn BASS_ChannelSetFX(handle: DWORD, fx_type: DWORD, priority: c_int) -> HFX;
    pub fn BASS_ChannelRemoveFX(handle: DWORD, fx: HFX) -> BOOL;
    pub fn BASS_FxSetParameters(handle: HFX, parameters: *const c_void) -> BOOL;
    pub fn BASS_FXGetParameters(handle: HFX, parameters: *mut c_void) -> BOOL;
    pub fn BASS_FXReset(handle: HFX);
    pub fn BASS_FXSetPriority(handle: HFX, priority: c_int) -> BOOL;
}
