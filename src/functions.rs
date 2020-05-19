use crate::types::{
    Bass3DVector, BassChannelInfo, BassDeviceInfo, BassFileProcs, BassInfo, BassPluginInfo,
    BassRecordInfo, BassSample, BOOL, DOWNLOADPROC, DSPPROC, DWORD, HCHANNEL, HDSP, HFX, HMUSIC,
    HPLUGIN, HRECORD, HSAMPLE, HSTREAM, HSYNC, QWORD, RECORDPROC, STREAMPROC, SYNCPROC,
};
use std::os::raw::{c_char, c_int, c_void};

#[allow(dead_code)]
extern "C" {
    fn BASS_SetConfig(option: DWORD, value: DWORD) -> BOOL;
    fn BASS_GetConfig(option: DWORD) -> DWORD;
    fn BASS_SetConfigPtr(option: DWORD, value: *mut c_void) -> BOOL;
    fn BASS_GetConfigPtr(option: DWORD) -> *mut c_void;
    fn BASS_GetVersion() -> DWORD;
    fn BASS_ErrorGetCode() -> c_int;
    fn BASS_GetDeviceInfo(device: DWORD, info: *mut BassDeviceInfo) -> BOOL;
    fn BASS_Init(
        device: c_int,
        frequency: DWORD,
        flags: DWORD,
        window: *mut c_void,
        dsguid: *mut c_void,
    ) -> BOOL;
    fn BASS_SetDevice(device: DWORD) -> BOOL;
    fn BASS_GetDevice() -> DWORD;
    fn BASS_Free() -> BOOL;
    fn BASS_GetInfo(info: *mut BassInfo) -> BOOL;
    fn BASS_Update(length: DWORD) -> BOOL;
    fn BASS_GetCPU() -> f32;
    fn BASS_Start() -> BOOL;
    fn BASS_Stop() -> BOOL;
    fn BASS_Pause() -> BOOL;
    fn BASS_IsStarted() -> BOOL;
    fn BASS_SetVolume(value: f32) -> BOOL;
    fn BASS_GetVolume() -> f32;
    fn BASS_PluginLoad(file: *mut c_char, flags: DWORD) -> HPLUGIN;
    fn BASS_PluginFree(handle: HPLUGIN) -> BOOL;
    fn BASS_PluginGetInfo(handle: HPLUGIN) -> *mut BassPluginInfo;
    fn BASS_Set3DFactors(distance: f32, roll: f32, doppler_factor: f32) -> BOOL;
    fn BASS_Get3DFactors(distance: f32, roll: f32, doppler_factor: f32) -> BOOL;
    fn BASS_Set3DPosition(
        position: *const Bass3DVector,
        velocity: *const Bass3DVector,
        front: *const Bass3DVector,
        top: *const Bass3DVector,
    ) -> BOOL;
    fn BASS_Get3DPosition(
        position: *mut Bass3DVector,
        velocity: *mut Bass3DVector,
        front: *mut Bass3DVector,
        top: *mut Bass3DVector,
    ) -> BOOL;
    fn BASS_Apply3D();
    fn BASS_SetEAXParameters(environment: c_int, volume: f32, decay: f32, damp: f32) -> BOOL;
    fn BASS_GetEAXParameters(
        environment: *mut DWORD,
        volume: *mut f32,
        decay: *mut f32,
        damp: *mut f32,
    ) -> BOOL;
    fn BASS_MusicLoad(
        memory: BOOL,
        file: *const c_char,
        offset: QWORD,
        length: DWORD,
        flags: DWORD,
        frequency: DWORD,
    ) -> HMUSIC;
    fn BASS_MusicFree(handle: HMUSIC) -> BOOL;
    fn BASS_SampleLoad(
        memory: BOOL,
        file: *const c_char,
        offset: QWORD,
        length: DWORD,
        maximum: DWORD,
        flags: DWORD,
    ) -> HSAMPLE;
    fn BASS_SampleCreate(
        length: DWORD,
        frequency: DWORD,
        channels: DWORD,
        maximum: DWORD,
        flags: DWORD,
    ) -> HSAMPLE;
    fn BASS_SampleFree(handle: HSAMPLE) -> BOOL;
    fn BASS_SampleSetData(handle: HSAMPLE, buffer: *const c_void) -> BOOL;
    fn BASS_SampleGetData(handle: HSAMPLE, buffer: *mut c_void) -> BOOL;
    fn BASS_SampleGetInfo(handle: HSAMPLE, info: *mut BassSample) -> BOOL;
    fn BASS_SampleSetInfo(handle: HSAMPLE, info: *const BassSample) -> BOOL;
    fn BASS_SampleGetChannel(handle: HSAMPLE, only_new: BOOL) -> HCHANNEL;
    fn BASS_SampleGetChannels(handle: HSAMPLE, channels: *mut HCHANNEL) -> DWORD;
    fn BASS_SampleStop(handle: HSAMPLE) -> BOOL;
    fn BASS_StreamCreate(
        frequency: DWORD,
        channels: DWORD,
        flags: DWORD,
        proc: *mut STREAMPROC,
        user: *mut c_void,
    ) -> HSTREAM;
    fn BASS_StreamCreateFile(
        memory: BOOL,
        file: *const c_char,
        offset: QWORD,
        length: QWORD,
        flags: DWORD,
    ) -> HSTREAM;
    fn BASS_StreamCreateURL(
        url: *const c_char,
        offset: DWORD,
        flags: DWORD,
        proc: *mut DOWNLOADPROC,
        user: *mut c_void,
    ) -> HSTREAM;
    fn BASS_StreamCreateFileUser(
        system: DWORD,
        flags: DWORD,
        proc: *mut BassFileProcs,
        user: *mut c_void,
    ) -> HSTREAM;
    fn BASS_StreamFree(handle: HSTREAM) -> BOOL;
    fn BASS_StreamGetFilePosition(handle: HSTREAM, mode: DWORD) -> QWORD;
    fn BASS_StreamPutData(handle: HSTREAM, buffer: *const c_void, length: DWORD) -> DWORD;
    fn BASS_StreamPutFileData(handle: HSTREAM, buffer: *const c_void, length: DWORD) -> DWORD;
    fn BASS_RecordGetDevicoInfo(device: DWORD, info: *mut BassDeviceInfo) -> BOOL;
    fn BASS_RecordInit(device: c_int) -> BOOL;
    fn BASS_RecordSetDevice(device: DWORD) -> BOOL;
    fn BASS_RecordGetDevice() -> DWORD;
    fn BASS_RecordFree() -> BOOL;
    fn BASS_RecordGetInfo(info: *mut BassRecordInfo) -> BOOL;
    fn BASS_RecordGetInputName(input: c_int) -> *const c_char;
    fn BASS_RecordSetInput(input: c_int, flags: DWORD, volume: f32) -> BOOL;
    fn BASS_RecordGetInput(input: c_int, volume: *mut f32) -> DWORD;
    fn BASS_RecordStart(
        frequency: DWORD,
        channels: DWORD,
        flags: DWORD,
        proc: *mut RECORDPROC,
        user: *mut c_void,
    ) -> HRECORD;
    fn BASS_ChannelBytes2Seconds(handle: DWORD, position: QWORD) -> f64;
    fn BASS_ChannelSeconds2Bytes(handle: DWORD, position: f64) -> QWORD;
    fn BASS_ChannelGetDevice(handle: DWORD) -> DWORD;
    fn BASS_ChannelSetDevice(handle: DWORD, device: DWORD) -> BOOL;
    fn BASS_ChannelIsActive(handle: DWORD) -> DWORD;
    fn BASS_ChannelGetInfo(handle: DWORD, info: *mut BassChannelInfo) -> BOOL;
    fn BASS_ChannelGetTags(handle: DWORD, tags: DWORD) -> *const c_char;
    fn BASS_ChannelFlags(handle: DWORD, flags: DWORD, mask: DWORD) -> DWORD;
    fn BASS_ChannelUpdate(handle: DWORD, length: DWORD) -> BOOL;
    fn BASS_ChannelLock(handle: DWORD, lock: BOOL) -> BOOL;
    fn BASS_ChannelPlay(handle: DWORD, restart: BOOL) -> BOOL;
    fn BASS_ChannelStop(handle: DWORD) -> BOOL;
    fn BASS_ChannelPause(handle: DWORD) -> BOOL;
    fn BASS_ChannelSetAttribute(handle: DWORD, attribute: DWORD, value: f32) -> BOOL;
    fn BASS_ChannelGetAttribute(handle: DWORD, attribute: DWORD, value: *mut f32) -> BOOL;
    fn BASS_ChannelSlideAttribute(handle: DWORD, attribute: DWORD, value: f32, time: DWORD)
        -> BOOL;
    fn BASS_ChannelIsSliding(handle: DWORD, attribute: DWORD) -> BOOL;
    fn BASS_ChannelSetAttributeEx(
        handle: DWORD,
        attribute: DWORD,
        value: *mut c_void,
        size: DWORD,
    ) -> BOOL;
    fn BASS_ChannelGetAttributeEx(
        handle: DWORD,
        attribute: DWORD,
        value: *mut c_void,
        size: DWORD,
    ) -> BOOL;
    fn BASS_ChannelSet3DAttributes(
        handle: DWORD,
        mode: c_int,
        minimum: f32,
        maximum: f32,
        iangle: c_int,
        oangle: c_int,
        out_volume: f32,
    ) -> BOOL;
    fn BASS_ChannelGet3DAttributes(
        handle: DWORD,
        mode: *mut DWORD,
        minimum: *mut f32,
        maximum: *mut f32,
        angle_of_inside_projection_cone: *mut DWORD,
        angle_of_outside_projection_cone: *mut DWORD,
        output_volume: *mut f32,
    ) -> BOOL;
    fn BASS_ChannelSet3DPosition(
        handle: DWORD,
        position: *const Bass3DVector,
        orientation: *const Bass3DVector,
        velocity: *const Bass3DVector,
    ) -> BOOL;
    fn BASS_ChannelGet3DPosition(
        handle: DWORD,
        position: *mut Bass3DVector,
        orientation: *mut Bass3DVector,
        velocity: *mut Bass3DVector,
    ) -> BOOL;
    fn BASS_ChannelGetLength(handle: DWORD, mode: DWORD) -> QWORD;
    fn BASS_ChannelSetPosition(handle: DWORD, position: QWORD, mode: DWORD) -> BOOL;
    fn BASS_ChannelGetPosition(handle: DWORD, mode: DWORD) -> QWORD;
    fn BASS_ChannelGetLevel(handle: DWORD) -> DWORD;
    fn BASS_ChannelGetLevelEx(handle: DWORD, levels: *mut f32, length: f32, flags: DWORD) -> BOOL;
    fn BASS_ChannelGetData(handle: DWORD, buffer: *mut c_void, length: DWORD) -> DWORD;
    fn BASS_ChannelSetSync(
        handle: DWORD,
        sync_type: DWORD,
        parameter: QWORD,
        proc: *mut SYNCPROC,
        user: *mut c_void,
    ) -> HSYNC;
    fn BASS_ChannelRemoveSync(handle: DWORD, sync: HSYNC) -> BOOL;
    fn BASS_ChannelSetDSP(handle: DWORD, proc: *mut DSPPROC, user: *mut c_void, priority: c_int);
    fn BASS_ChannelRemoveDSP(handle: DWORD, dsp: HDSP) -> BOOL;
    fn BASS_ChannelSetLink(handle: DWORD, channel: DWORD) -> BOOL;
    fn BASS_ChannelRemoveLink(handle: DWORD, channel: DWORD) -> BOOL;
    fn BASS_ChannelSetFX(handle: DWORD, fx_type: DWORD, priority: c_int) -> HFX;
    fn BASS_ChannelRemoveFX(handle: DWORD, fx: HFX) -> BOOL;
    fn BASS_FxSetParameters(handle: HFX, parameters: *const c_void) -> BOOL;
    fn BASS_FXGetParameters(handle: HFX, parameters: *mut c_void) -> BOOL;
    fn BASS_FXReset(handle: HFX);
    fn BASS_FXSetPriority(handle: HFX, priority: c_int) -> BOOL;
}
