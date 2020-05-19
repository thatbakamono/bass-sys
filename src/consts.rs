use crate::types::DWORD;
use std::os::raw::c_int;

// Error codes returned by BASS_ErrorGetCode
pub const BASS_OK: c_int = 0; // all is OK
pub const BASS_ERROR_MEM: c_int = 1; // memory error
pub const BASS_ERROR_FILEOPEN: c_int = 2; // can't open the file
pub const BASS_ERROR_DRIVER: c_int = 3; // can't find a free/valid driver
pub const BASS_ERROR_BUFLOST: c_int = 4; // the sample buffer was lost
pub const BASS_ERROR_HANDLE: c_int = 5; // invalid handle
pub const BASS_ERROR_FORMAT: c_int = 6; // unsupported sample format
pub const BASS_ERROR_POSITION: c_int = 7; // invalid position
pub const BASS_ERROR_INIT: c_int = 8; // BASS_Init has not been successfully called
pub const BASS_ERROR_START: c_int = 9; // BASS_Start has not been successfully called
pub const BASS_ERROR_SSL: c_int = 10; // SSL/HTTPS support isn't available
pub const BASS_ERROR_ALREADY: c_int = 14; // already initialized/paused/whatever
pub const BASS_ERROR_NOTAUDIO: c_int = 17; // file does not contain audio
pub const BASS_ERROR_NOCHAN: c_int = 18; // can't get a free channel
pub const BASS_ERROR_ILLTYPE: c_int = 19; // an illegal type was specified
pub const BASS_ERROR_ILLPARAM: c_int = 20; // an illegal parameter was specified
pub const BASS_ERROR_NO3D: c_int = 21; // no 3D support
pub const BASS_ERROR_NOEAX: c_int = 22; // no EAX support
pub const BASS_ERROR_DEVICE: c_int = 23; // illegal device number
pub const BASS_ERROR_NOPLAY: c_int = 24; // not playing
pub const BASS_ERROR_FREQ: c_int = 25; // illegal sample rate
pub const BASS_ERROR_NOTFILE: c_int = 27; // the stream is not a file stream
pub const BASS_ERROR_NOHW: c_int = 29; // no hardware voices available
pub const BASS_ERROR_EMPTY: c_int = 31; // the MOD music has no sequence data
pub const BASS_ERROR_NONET: c_int = 32; // no internet connection could be opened
pub const BASS_ERROR_CREATE: c_int = 33; // couldn't create the file
pub const BASS_ERROR_NOFX: c_int = 34; // effects are not available
pub const BASS_ERROR_NOTAVAIL: c_int = 37; // requested data/action is not available
pub const BASS_ERROR_DECODE: c_int = 38; // the channel is/isn't a "decoding channel"
pub const BASS_ERROR_DX: c_int = 39; // a sufficient DirectX version is not installed
pub const BASS_ERROR_TIMEOUT: c_int = 40; // connection timedout
pub const BASS_ERROR_FILEFORM: c_int = 41; // unsupported file format
pub const BASS_ERROR_SPEAKER: c_int = 42; // unavailable speaker
pub const BASS_ERROR_VERSION: c_int = 43; // invalid BASS version (used by add-ons)
pub const BASS_ERROR_CODEC: c_int = 44; // codec is not available/supported
pub const BASS_ERROR_ENDED: c_int = 45; // the channel/file has ended
pub const BASS_ERROR_BUSY: c_int = 46; // the device is busy
pub const BASS_ERROR_UNSTREAMABLE: c_int = 47; // unstreamable file
pub const BASS_ERROR_UNKNOWN: c_int = -1; // some other mystery problem

// BASS_SetConfig options
pub const BASS_CONFIG_BUFFER: DWORD = 0;
pub const BASS_CONFIG_UPDATEPERIOD: DWORD = 1;
pub const BASS_CONFIG_GVOL_SAMPLE: DWORD = 4;
pub const BASS_CONFIG_GVOL_STREAM: DWORD = 5;
pub const BASS_CONFIG_GVOL_MUSIC: DWORD = 6;
pub const BASS_CONFIG_CURVE_VOL: DWORD = 7;
pub const BASS_CONFIG_CURVE_PAN: DWORD = 8;
pub const BASS_CONFIG_FLOATDSP: DWORD = 9;
pub const BASS_CONFIG_3DALGORITHM: DWORD = 10;
pub const BASS_CONFIG_NET_TIMEOUT: DWORD = 11;
pub const BASS_CONFIG_NET_BUFFER: DWORD = 12;
pub const BASS_CONFIG_PAUSE_NOPLAY: DWORD = 13;
pub const BASS_CONFIG_NET_PREBUF: DWORD = 15;
pub const BASS_CONFIG_NET_PASSIVE: DWORD = 18;
pub const BASS_CONFIG_REC_BUFFER: DWORD = 19;
pub const BASS_CONFIG_NET_PLAYLIST: DWORD = 21;
pub const BASS_CONFIG_MUSIC_VIRTUAL: DWORD = 22;
pub const BASS_CONFIG_VERIFY: DWORD = 23;
pub const BASS_CONFIG_UPDATETHREADS: DWORD = 24;
pub const BASS_CONFIG_DEV_BUFFER: DWORD = 27;
pub const BASS_CONFIG_REC_LOOPBACK: DWORD = 28;
pub const BASS_CONFIG_VISTA_TRUEPOS: DWORD = 30;
pub const BASS_CONFIG_IOS_SESSION: DWORD = 34;
pub const BASS_CONFIG_IOS_MIXAUDIO: DWORD = 34;
pub const BASS_CONFIG_DEV_DEFAULT: DWORD = 36;
pub const BASS_CONFIG_NET_READTIMEOUT: DWORD = 37;
pub const BASS_CONFIG_VISTA_SPEAKERS: DWORD = 38;
pub const BASS_CONFIG_IOS_SPEAKER: DWORD = 39;
pub const BASS_CONFIG_MF_DISABLE: DWORD = 40;
pub const BASS_CONFIG_HANDLES: DWORD = 41;
pub const BASS_CONFIG_UNICODE: DWORD = 42;
pub const BASS_CONFIG_SRC: DWORD = 43;
pub const BASS_CONFIG_SRC_SAMPLE: DWORD = 44;
pub const BASS_CONFIG_ASYNCFILE_BUFFER: DWORD = 45;
pub const BASS_CONFIG_OGG_PRESCAN: DWORD = 47;
pub const BASS_CONFIG_MF_VIDEO: DWORD = 48;
pub const BASS_CONFIG_AIRPLAY: DWORD = 49;
pub const BASS_CONFIG_DEV_NONSTOP: DWORD = 50;
pub const BASS_CONFIG_IOS_NOCATEGORY: DWORD = 51;
pub const BASS_CONFIG_VERIFY_NET: DWORD = 52;
pub const BASS_CONFIG_DEV_PERIOD: DWORD = 53;
pub const BASS_CONFIG_FLOAT: DWORD = 54;
pub const BASS_CONFIG_NET_SEEK: DWORD = 56;
pub const BASS_CONFIG_AM_DISABLE: DWORD = 58;
pub const BASS_CONFIG_NET_PLAYLIST_DEPTH: DWORD = 59;
pub const BASS_CONFIG_NET_PREBUF_WAIT: DWORD = 60;
pub const BASS_CONFIG_ANDROID_SESSIONID: DWORD = 62;
pub const BASS_CONFIG_WASAPI_PERSIST: DWORD = 65;
pub const BASS_CONFIG_REC_WASAPI: DWORD = 66;
pub const BASS_CONFIG_ANDROID_AAUDIO: DWORD = 67;

// BASS_SetConfigPtr options
pub const BASS_CONFIG_NET_AGENT: DWORD = 16;
pub const BASS_CONFIG_NET_PROXY: DWORD = 17;
pub const BASS_CONFIG_IOS_NOTIFY: DWORD = 46;
pub const BASS_CONFIG_LIBSSL: DWORD = 64;

// BASS_CONFIG_IOS_SESSION flags
pub const BASS_IOS_SESSION_MIX: DWORD = 1;
pub const BASS_IOS_SESSION_DUCK: DWORD = 2;
pub const BASS_IOS_SESSION_AMBIENT: DWORD = 4;
pub const BASS_IOS_SESSION_SPEAKER: DWORD = 8;
pub const BASS_IOS_SESSION_DISABLE: DWORD = 16;

// BASS_Init flags
pub const BASS_DEVICE_8BITS: DWORD = 1; // 8 bit
pub const BASS_DEVICE_MONO: DWORD = 2; // mono
pub const BASS_DEVICE_3D: DWORD = 4; // enable 3D functionality
pub const BASS_DEVICE_16BITS: DWORD = 8; // limit output to 16 bit
pub const BASS_DEVICE_LATENCY: DWORD = 0x100; // calculate device latency (BASS_INFO struct)
pub const BASS_DEVICE_CPSPEAKERS: DWORD = 0x400; // detect speakers via Windows control panel
pub const BASS_DEVICE_SPEAKERS: DWORD = 0x800; // force enabling of speaker assignment
pub const BASS_DEVICE_NOSPEAKER: DWORD = 0x1000; // ignore speaker arrangement
pub const BASS_DEVICE_DMIX: DWORD = 0x2000; // use ALSA "dmix" plugin
pub const BASS_DEVICE_FREQ: DWORD = 0x4000; // set device sample rate
pub const BASS_DEVICE_STEREO: DWORD = 0x8000; // limit output to stereo
pub const BASS_DEVICE_HOG: DWORD = 0x10000; // hog/exclusive mode
pub const BASS_DEVICE_AUDIOTRACK: DWORD = 0x20000; // use AudioTrack output
pub const BASS_DEVICE_DSOUND: DWORD = 0x40000; // use DirectSound output

// DirectSound interfaces (for use with BASS_GetDSoundObject)
pub const BASS_OBJECT_DS: DWORD = 1; // IDirectSound
pub const BASS_OBJECT_DS3DL: DWORD = 2; // IDirectSound3DListener

// BASS_DEVICEINFO flags
pub const BASS_DEVICE_ENABLED: DWORD = 1;
pub const BASS_DEVICE_DEFAULT: DWORD = 2;
pub const BASS_DEVICE_INIT: DWORD = 4;
pub const BASS_DEVICE_LOOPBACK: DWORD = 8;

pub const BASS_DEVICE_TYPE_MASK: DWORD = 0xff000000;
pub const BASS_DEVICE_TYPE_NETWORK: DWORD = 0x01000000;
pub const BASS_DEVICE_TYPE_SPEAKERS: DWORD = 0x02000000;
pub const BASS_DEVICE_TYPE_LINE: DWORD = 0x03000000;
pub const BASS_DEVICE_TYPE_HEADPHONES: DWORD = 0x04000000;
pub const BASS_DEVICE_TYPE_MICROPHONE: DWORD = 0x05000000;
pub const BASS_DEVICE_TYPE_HEADSET: DWORD = 0x06000000;
pub const BASS_DEVICE_TYPE_HANDSET: DWORD = 0x07000000;
pub const BASS_DEVICE_TYPE_DIGITAL: DWORD = 0x08000000;
pub const BASS_DEVICE_TYPE_SPDIF: DWORD = 0x09000000;
pub const BASS_DEVICE_TYPE_HDMI: DWORD = 0x0a000000;
pub const BASS_DEVICE_TYPE_DISPLAYPORT: DWORD = 0x40000000;

// BASS_GetDeviceInfo flags
pub const BASS_DEVICES_AIRPLAY: DWORD = 0x1000000;

// BASS_INFO flags (from DSOUND.H)
pub const DSCAPS_CONTINUOUSRATE: DWORD = 0x00000010; // supports all sample rates between min/maxrate
pub const DSCAPS_EMULDRIVER: DWORD = 0x00000020; // device does NOT have hardware DirectSound support
pub const DSCAPS_CERTIFIED: DWORD = 0x00000040; // device driver has been certified by Microsoft
pub const DSCAPS_SECONDARYMONO: DWORD = 0x00000100; // mono
pub const DSCAPS_SECONDARYSTEREO: DWORD = 0x00000200; // stereo
pub const DSCAPS_SECONDARY8BIT: DWORD = 0x00000400; // 8 bit
pub const DSCAPS_SECONDARY16BIT: DWORD = 0x00000800; // 16 bit

// BASS_RECORDINFO flags (from DSOUND.H)
pub const DSCCAPS_EMULDRIVER: DWORD = DSCAPS_EMULDRIVER; // device does NOT have hardware DirectSound recording support
pub const DSCCAPS_CERTIFIED: DWORD = DSCAPS_CERTIFIED; // device driver has been certified by Microsoft

pub const WAVE_FORMAT_1M08: DWORD = 0x00000001; // 11.025 kHz, Mono,   8-bit
pub const WAVE_FORMAT_1S08: DWORD = 0x00000002; // 11.025 kHz, Stereo, 8-bit
pub const WAVE_FORMAT_1M16: DWORD = 0x00000004; // 11.025 kHz, Mono,   16-bit
pub const WAVE_FORMAT_1S16: DWORD = 0x00000008; // 11.025 kHz, Stereo, 16-bit
pub const WAVE_FORMAT_2M08: DWORD = 0x00000010; // 22.05  kHz, Mono,   8-bit
pub const WAVE_FORMAT_2S08: DWORD = 0x00000020; // 22.05  kHz, Stereo, 8-bit
pub const WAVE_FORMAT_2M16: DWORD = 0x00000040; // 22.05  kHz, Mono,   16-bit
pub const WAVE_FORMAT_2S16: DWORD = 0x00000080; // 22.05  kHz, Stereo, 16-bit
pub const WAVE_FORMAT_4M08: DWORD = 0x00000100; // 44.1   kHz, Mono,   8-bit
pub const WAVE_FORMAT_4S08: DWORD = 0x00000200; // 44.1   kHz, Stereo, 8-bit
pub const WAVE_FORMAT_4M16: DWORD = 0x00000400; // 44.1   kHz, Mono,   16-bit
pub const WAVE_FORMAT_4S16: DWORD = 0x00000800; // 44.1   kHz, Stereo, 16-bit

pub const BASS_SAMPLE_8BITS: DWORD = 1; // 8 bit
pub const BASS_SAMPLE_FLOAT: DWORD = 256; // 32 bit floating-point
pub const BASS_SAMPLE_MONO: DWORD = 2; // mono
pub const BASS_SAMPLE_LOOP: DWORD = 4; // looped
pub const BASS_SAMPLE_3D: DWORD = 8; // 3D functionality
pub const BASS_SAMPLE_SOFTWARE: DWORD = 16; // not using hardware mixing
pub const BASS_SAMPLE_MUTEMAX: DWORD = 32; // mute at max distance (3D only)
pub const BASS_SAMPLE_VAM: DWORD = 64; // DX7 voice allocation & management
pub const BASS_SAMPLE_FX: DWORD = 128; // old implementation of DX8 effects
pub const BASS_SAMPLE_OVER_VOL: DWORD = 0x10000; // override lowest volume
pub const BASS_SAMPLE_OVER_POS: DWORD = 0x20000; // override longest playing
pub const BASS_SAMPLE_OVER_DIST: DWORD = 0x30000; // override furthest from listener (3D only)

pub const BASS_STREAM_PRESCAN: DWORD = 0x20000; // enable pin-point seeking/length (MP3/MP2/MP1)
pub const BASS_STREAM_AUTOFREE: DWORD = 0x40000; // automatically free the stream when it stop/ends
pub const BASS_STREAM_RESTRATE: DWORD = 0x80000; // restrict the download rate of internet file streams
pub const BASS_STREAM_BLOCK: DWORD = 0x100000; // download/play internet file stream in small blocks
pub const BASS_STREAM_DECODE: DWORD = 0x200000; // don't play the stream, only decode (BASS_ChannelGetData)
pub const BASS_STREAM_STATUS: DWORD = 0x800000; // give server status info (HTTP/ICY tags) in DOWNLOADPROC

pub const BASS_MP3_IGNOREDELAY: DWORD = 0x200; // ignore LAME/Xing/VBRI/iTunes delay & padding info
pub const BASS_MP3_SETPOS: DWORD = BASS_STREAM_PRESCAN;

pub const BASS_MUSIC_FLOAT: DWORD = BASS_SAMPLE_FLOAT;
pub const BASS_MUSIC_MONO: DWORD = BASS_SAMPLE_MONO;
pub const BASS_MUSIC_LOOP: DWORD = BASS_SAMPLE_LOOP;
pub const BASS_MUSIC_3D: DWORD = BASS_SAMPLE_3D;
pub const BASS_MUSIC_FX: DWORD = BASS_SAMPLE_FX;
pub const BASS_MUSIC_AUTOFREE: DWORD = BASS_STREAM_AUTOFREE;
pub const BASS_MUSIC_DECODE: DWORD = BASS_STREAM_DECODE;
pub const BASS_MUSIC_PRESCAN: DWORD = BASS_STREAM_PRESCAN; // calculate playback length
pub const BASS_MUSIC_CALCLEN: DWORD = BASS_MUSIC_PRESCAN;
pub const BASS_MUSIC_RAMP: DWORD = 0x200; // normal ramping
pub const BASS_MUSIC_RAMPS: DWORD = 0x400; // sensitive ramping
pub const BASS_MUSIC_SURROUND: DWORD = 0x800; // surround sound
pub const BASS_MUSIC_SURROUND2: DWORD = 0x1000; // surround sound (mode 2)
pub const BASS_MUSIC_FT2PAN: DWORD = 0x2000; // apply FastTracker 2 panning to XM files
pub const BASS_MUSIC_FT2MOD: DWORD = 0x2000; // play .MOD as FastTracker 2 does
pub const BASS_MUSIC_PT1MOD: DWORD = 0x4000; // play .MOD as ProTracker 1 does
pub const BASS_MUSIC_NONINTER: DWORD = 0x10000; // non-interpolated sample mixing
pub const BASS_MUSIC_SINCINTER: DWORD = 0x800000; // sinc interpolated sample mixing
pub const BASS_MUSIC_POSRESET: DWORD = 0x8000; // stop all notes when moving position
pub const BASS_MUSIC_POSRESETEX: DWORD = 0x400000; // stop all notes and reset bmp/etc when moving position
pub const BASS_MUSIC_STOPBACK: DWORD = 0x80000; // stop the music on a backwards jump effect
pub const BASS_MUSIC_NOSAMPLE: DWORD = 0x100000; // don't load the samples

// Speaker assignment flags
pub const BASS_SPEAKER_FRONT: DWORD = 0x1000000; // front speakers
pub const BASS_SPEAKER_REAR: DWORD = 0x2000000; // rear/side speakers
pub const BASS_SPEAKER_CENLFE: DWORD = 0x3000000; // center & LFE speakers (5.1)
pub const BASS_SPEAKER_REAR2: DWORD = 0x4000000; // rear center speakers (7.1)
pub const BASS_SPEAKER_LEFT: DWORD = 0x10000000; // modifier: left
pub const BASS_SPEAKER_RIGHT: DWORD = 0x20000000; // modifier: right
pub const BASS_SPEAKER_FRONTLEFT: DWORD = BASS_SPEAKER_FRONT | BASS_SPEAKER_LEFT;
pub const BASS_SPEAKER_FRONTRIGHT: DWORD = BASS_SPEAKER_FRONT | BASS_SPEAKER_RIGHT;
pub const BASS_SPEAKER_REARLEFT: DWORD = BASS_SPEAKER_REAR | BASS_SPEAKER_LEFT;
pub const BASS_SPEAKER_REARRIGHT: DWORD = BASS_SPEAKER_REAR | BASS_SPEAKER_RIGHT;
pub const BASS_SPEAKER_CENTER: DWORD = BASS_SPEAKER_CENLFE | BASS_SPEAKER_LEFT;
pub const BASS_SPEAKER_LFE: DWORD = BASS_SPEAKER_CENLFE | BASS_SPEAKER_RIGHT;
pub const BASS_SPEAKER_REAR2LEFT: DWORD = BASS_SPEAKER_REAR2 | BASS_SPEAKER_LEFT;
pub const BASS_SPEAKER_REAR2RIGHT: DWORD = BASS_SPEAKER_REAR2 | BASS_SPEAKER_RIGHT;

pub const BASS_ASYNCFILE: DWORD = 0x40000000;
pub const BASS_UNICODE: DWORD = 0x80000000;

pub const BASS_RECORD_PAUSE: DWORD = 0x8000; // start recording paused
pub const BASS_RECORD_ECHOCANCEL: DWORD = 0x2000;
pub const BASS_RECORD_AGC: DWORD = 0x4000;

// DX7 voice allocation & management flags
pub const BASS_VAM_HARDWARE: DWORD = 1;
pub const BASS_VAM_SOFTWARE: DWORD = 2;
pub const BASS_VAM_TERM_TIME: DWORD = 4;
pub const BASS_VAM_TERM_DIST: DWORD = 8;
pub const BASS_VAM_TERM_PRIO: DWORD = 16;

pub const BASS_ORIGRES_FLOAT: DWORD = 0x10000;

// BASS_CHANNELINFO types
pub const BASS_CTYPE_SAMPLE: DWORD = 1;
pub const BASS_CTYPE_RECORD: DWORD = 2;
pub const BASS_CTYPE_STREAM: DWORD = 0x10000;
pub const BASS_CTYPE_STREAM_VORBIS: DWORD = 0x10002;
pub const BASS_CTYPE_STREAM_OGG: DWORD = 0x10002;
pub const BASS_CTYPE_STREAM_MP1: DWORD = 0x10003;
pub const BASS_CTYPE_STREAM_MP2: DWORD = 0x10004;
pub const BASS_CTYPE_STREAM_MP3: DWORD = 0x10005;
pub const BASS_CTYPE_STREAM_AIFF: DWORD = 0x10006;
pub const BASS_CTYPE_STREAM_CA: DWORD = 0x10007;
pub const BASS_CTYPE_STREAM_MF: DWORD = 0x10008;
pub const BASS_CTYPE_STREAM_AM: DWORD = 0x10009;
pub const BASS_CTYPE_STREAM_DUMMY: DWORD = 0x18000;
pub const BASS_CTYPE_STREAM_DEVICE: DWORD = 0x18001;
pub const BASS_CTYPE_STREAM_WAV: DWORD = 0x40000; // WAVE flag, LOWORD=codec
pub const BASS_CTYPE_STREAM_WAV_PCM: DWORD = 0x50001;
pub const BASS_CTYPE_STREAM_WAV_FLOAT: DWORD = 0x50003;
pub const BASS_CTYPE_MUSIC_MOD: DWORD = 0x20000;
pub const BASS_CTYPE_MUSIC_MTM: DWORD = 0x20001;
pub const BASS_CTYPE_MUSIC_S3M: DWORD = 0x20002;
pub const BASS_CTYPE_MUSIC_XM: DWORD = 0x20003;
pub const BASS_CTYPE_MUSIC_IT: DWORD = 0x20004;
pub const BASS_CTYPE_MUSIC_MO3: DWORD = 0x00100; // MO3 flag

// 3D channel modes
pub const BASS_3DMODE_NORMAL: DWORD = 0; // normal 3D processing
pub const BASS_3DMODE_RELATIVE: DWORD = 1; // position is relative to the listener
pub const BASS_3DMODE_OFF: DWORD = 2; // no 3D processing

// software 3D mixing algorithms (used with BASS_CONFIG_3DALGORITHM)
pub const BASS_3DALG_DEFAULT: DWORD = 0;
pub const BASS_3DALG_OFF: DWORD = 1;
pub const BASS_3DALG_FULL: DWORD = 2;
pub const BASS_3DALG_LIGHT: DWORD = 3;

pub const BASS_STREAMPROC_END: DWORD = 0x80000000; // end of user stream flag

// BASS_StreamCreateFileUser file systems
pub const STREAMFILE_NOBUFFER: DWORD = 0;
pub const STREAMFILE_BUFFER: DWORD = 1;
pub const STREAMFILE_BUFFERPUSH: DWORD = 2;

// BASS_StreamPutFileData options
pub const BASS_FILEDATA_END: DWORD = 0; // end & close the file

// BASS_StreamGetFilePosition modes
pub const BASS_FILEPOS_CURRENT: DWORD = 0;
pub const BASS_FILEPOS_DECODE: DWORD = BASS_FILEPOS_CURRENT;
pub const BASS_FILEPOS_DOWNLOAD: DWORD = 1;
pub const BASS_FILEPOS_END: DWORD = 2;
pub const BASS_FILEPOS_START: DWORD = 3;
pub const BASS_FILEPOS_CONNECTED: DWORD = 4;
pub const BASS_FILEPOS_BUFFER: DWORD = 5;
pub const BASS_FILEPOS_SOCKET: DWORD = 6;
pub const BASS_FILEPOS_ASYNCBUF: DWORD = 7;
pub const BASS_FILEPOS_SIZE: DWORD = 8;
pub const BASS_FILEPOS_BUFFERING: DWORD = 9;

// BASS_ChannelSetSync types
pub const BASS_SYNC_POS: DWORD = 0;
pub const BASS_SYNC_END: DWORD = 2;
pub const BASS_SYNC_META: DWORD = 4;
pub const BASS_SYNC_SLIDE: DWORD = 5;
pub const BASS_SYNC_STALL: DWORD = 6;
pub const BASS_SYNC_DOWNLOAD: DWORD = 7;
pub const BASS_SYNC_FREE: DWORD = 8;
pub const BASS_SYNC_SETPOS: DWORD = 11;
pub const BASS_SYNC_MUSICPOS: DWORD = 10;
pub const BASS_SYNC_MUSICINST: DWORD = 1;
pub const BASS_SYNC_MUSICFX: DWORD = 3;
pub const BASS_SYNC_OGG_CHANGE: DWORD = 12;
pub const BASS_SYNC_DEV_FAIL: DWORD = 14;
pub const BASS_SYNC_DEV_FORMAT: DWORD = 15;
pub const BASS_SYNC_THREAD: DWORD = 0x20000000; // flag: call sync in other thread
pub const BASS_SYNC_MIXTIME: DWORD = 0x40000000; // flag: sync at mixtime, else at playtime
pub const BASS_SYNC_ONETIME: DWORD = 0x80000000; // flag: sync only once, else continuously

// BASS_ChannelIsActive return values
pub const BASS_ACTIVE_STOPPED: DWORD = 0;
pub const BASS_ACTIVE_PLAYING: DWORD = 1;
pub const BASS_ACTIVE_STALLED: DWORD = 2;
pub const BASS_ACTIVE_PAUSED: DWORD = 3;
pub const BASS_ACTIVE_PAUSED_DEVICE: DWORD = 4;

// Channel attributes
pub const BASS_ATTRIB_FREQ: DWORD = 1;
pub const BASS_ATTRIB_VOL: DWORD = 2;
pub const BASS_ATTRIB_PAN: DWORD = 3;
pub const BASS_ATTRIB_EAXMIX: DWORD = 4;
pub const BASS_ATTRIB_NOBUFFER: DWORD = 5;
pub const BASS_ATTRIB_VBR: DWORD = 6;
pub const BASS_ATTRIB_CPU: DWORD = 7;
pub const BASS_ATTRIB_SRC: DWORD = 8;
pub const BASS_ATTRIB_NET_RESUME: DWORD = 9;
pub const BASS_ATTRIB_SCANINFO: DWORD = 10;
pub const BASS_ATTRIB_NORAMP: DWORD = 11;
pub const BASS_ATTRIB_BITRATE: DWORD = 12;
pub const BASS_ATTRIB_BUFFER: DWORD = 13;
pub const BASS_ATTRIB_GRANULE: DWORD = 14;
pub const BASS_ATTRIB_MUSIC_AMPLIFY: DWORD = 0x100;
pub const BASS_ATTRIB_MUSIC_PANSEP: DWORD = 0x101;
pub const BASS_ATTRIB_MUSIC_PSCALER: DWORD = 0x102;
pub const BASS_ATTRIB_MUSIC_BPM: DWORD = 0x103;
pub const BASS_ATTRIB_MUSIC_SPEED: DWORD = 0x104;
pub const BASS_ATTRIB_MUSIC_VOL_GLOBAL: DWORD = 0x105;
pub const BASS_ATTRIB_MUSIC_ACTIVE: DWORD = 0x106;
pub const BASS_ATTRIB_MUSIC_VOL_CHAN: DWORD = 0x200; // + channel #
pub const BASS_ATTRIB_MUSIC_VOL_INST: DWORD = 0x300; // + instrument #

// BASS_ChannelSlideAttribute flags
pub const BASS_SLIDE_LOG: DWORD = 0x1000000;

// BASS_ChannelGetData flags
pub const BASS_DATA_AVAILABLE: DWORD = 0; // query how much data is buffered
pub const BASS_DATA_FIXED: DWORD = 0x20000000; // flag: return 8.24 fixed-point data
pub const BASS_DATA_FLOAT: DWORD = 0x40000000; // flag: return floating-point sample data
pub const BASS_DATA_FFT256: DWORD = 0x80000000; // 256 sample FFT
pub const BASS_DATA_FFT512: DWORD = 0x80000001; // 512 FFT
pub const BASS_DATA_FFT1024: DWORD = 0x80000002; // 1024 FFT
pub const BASS_DATA_FFT2048: DWORD = 0x80000003; // 2048 FFT
pub const BASS_DATA_FFT4096: DWORD = 0x80000004; // 4096 FFT
pub const BASS_DATA_FFT8192: DWORD = 0x80000005; // 8192 FFT
pub const BASS_DATA_FFT16384: DWORD = 0x80000006; // 16384 FFT
pub const BASS_DATA_FFT32768: DWORD = 0x80000007; // 32768 FFT
pub const BASS_DATA_FFT_INDIVIDUAL: DWORD = 0x10; // FFT flag: FFT for each channel, else all combined
pub const BASS_DATA_FFT_NOWINDOW: DWORD = 0x20; // FFT flag: no Hanning window
pub const BASS_DATA_FFT_REMOVEDC: DWORD = 0x40; // FFT flag: pre-remove DC bias
pub const BASS_DATA_FFT_COMPLEX: DWORD = 0x80; // FFT flag: return complex data
pub const BASS_DATA_FFT_NYQUIST: DWORD = 0x100; // FFT flag: return extra Nyquist value

// BASS_ChannelGetLevelEx flags
pub const BASS_LEVEL_MONO: DWORD = 1;
pub const BASS_LEVEL_STEREO: DWORD = 2;
pub const BASS_LEVEL_RMS: DWORD = 4;
pub const BASS_LEVEL_VOLPAN: DWORD = 8;

// BASS_ChannelGetTags types : what's returned
pub const BASS_TAG_ID3: DWORD = 0; // ID3v1 tags : TAG_ID3 structure
pub const BASS_TAG_ID3V2: DWORD = 1; // ID3v2 tags : variable length block
pub const BASS_TAG_OGG: DWORD = 2; // OGG comments : series of null-terminated UTF-8 strings
pub const BASS_TAG_HTTP: DWORD = 3; // HTTP headers : series of null-terminated ANSI strings
pub const BASS_TAG_ICY: DWORD = 4; // ICY headers : series of null-terminated ANSI strings
pub const BASS_TAG_META: DWORD = 5; // ICY metadata : ANSI string
pub const BASS_TAG_APE: DWORD = 6; // APE tags : series of null-terminated UTF-8 strings
pub const BASS_TAG_MP4: DWORD = 7; // MP4/iTunes metadata : series of null-terminated UTF-8 strings
pub const BASS_TAG_WMA: DWORD = 8; // WMA tags : series of null-terminated UTF-8 strings
pub const BASS_TAG_VENDOR: DWORD = 9; // OGG encoder : UTF-8 string
pub const BASS_TAG_LYRICS3: DWORD = 10; // Lyric3v2 tag : ASCII string
pub const BASS_TAG_CA_CODEC: DWORD = 11; // CoreAudio codec info : TAG_CA_CODEC structure
pub const BASS_TAG_MF: DWORD = 13; // Media Foundation tags : series of null-terminated UTF-8 strings
pub const BASS_TAG_WAVEFORMAT: DWORD = 14; // WAVE format : WAVEFORMATEEX structure
pub const BASS_TAG_AM_MIME: DWORD = 15; // Android Media MIME type : ASCII string
pub const BASS_TAG_AM_NAME: DWORD = 16; // Android Media codec name : ASCII string
pub const BASS_TAG_RIFF_INFO: DWORD = 0x100; // RIFF "INFO" tags : series of null-terminated ANSI strings
pub const BASS_TAG_RIFF_BEXT: DWORD = 0x101; // RIFF/BWF "bext" tags : TAG_BEXT structure
pub const BASS_TAG_RIFF_CART: DWORD = 0x102; // RIFF/BWF "cart" tags : TAG_CART structure
pub const BASS_TAG_RIFF_DISP: DWORD = 0x103; // RIFF "DISP" text tag : ANSI string
pub const BASS_TAG_RIFF_CUE: DWORD = 0x104; // RIFF "cue " chunk : TAG_CUE structure
pub const BASS_TAG_RIFF_SMPL: DWORD = 0x105; // RIFF "smpl" chunk : TAG_SMPL structure
pub const BASS_TAG_APE_BINARY: DWORD = 0x1000; // + index #, binary APE tag : TAG_APE_BINARY structure
pub const BASS_TAG_MUSIC_NAME: DWORD = 0x10000; // MOD music name : ANSI string
pub const BASS_TAG_MUSIC_MESSAGE: DWORD = 0x10001; // MOD message : ANSI string
pub const BASS_TAG_MUSIC_ORDERS: DWORD = 0x10002; // MOD order list : BYTE array of pattern numbers
pub const BASS_TAG_MUSIC_AUTH: DWORD = 0x10003; // MOD author : UTF-8 string
pub const BASS_TAG_MUSIC_INST: DWORD = 0x10100; // + instrument #, MOD instrument name : ANSI string
pub const BASS_TAG_MUSIC_SAMPLE: DWORD = 0x10300; // + sample #, MOD sample name : ANSI string

// BASS_ChannelGetLength/GetPosition/SetPosition modes
pub const BASS_POS_BYTE: DWORD = 0; // byte position
pub const BASS_POS_MUSIC_ORDER: DWORD = 1; // order.row position, MAKELONG(order,row)
pub const BASS_POS_OGG: DWORD = 3; // OGG bitstream number
pub const BASS_POS_RESET: DWORD = 0x2000000; // flag: reset user file buffers
pub const BASS_POS_RELATIVE: DWORD = 0x4000000; // flag: seek relative to the current position
pub const BASS_POS_INEXACT: DWORD = 0x8000000; // flag: allow seeking to inexact position
pub const BASS_POS_DECODE: DWORD = 0x10000000; // flag: get the decoding (not playing) position
pub const BASS_POS_DECODETO: DWORD = 0x20000000; // flag: decode to the position instead of seeking
pub const BASS_POS_SCAN: DWORD = 0x40000000; // flag: scan to the position

// BASS_ChannelSetDevice/GetDevice option
pub const BASS_NODEVICE: DWORD = 0x20000;

// BASS_RecordSetInput flags
pub const BASS_INPUT_OFF: DWORD = 0x10000;
pub const BASS_INPUT_ON: DWORD = 0x20000;

pub const BASS_INPUT_TYPE_MASK: DWORD = 0xff000000;
pub const BASS_INPUT_TYPE_UNDEF: DWORD = 0x00000000;
pub const BASS_INPUT_TYPE_DIGITAL: DWORD = 0x01000000;
pub const BASS_INPUT_TYPE_LINE: DWORD = 0x02000000;
pub const BASS_INPUT_TYPE_MIC: DWORD = 0x03000000;
pub const BASS_INPUT_TYPE_SYNTH: DWORD = 0x04000000;
pub const BASS_INPUT_TYPE_CD: DWORD = 0x05000000;
pub const BASS_INPUT_TYPE_PHONE: DWORD = 0x06000000;
pub const BASS_INPUT_TYPE_SPEAKER: DWORD = 0x07000000;
pub const BASS_INPUT_TYPE_WAVE: DWORD = 0x08000000;
pub const BASS_INPUT_TYPE_AUX: DWORD = 0x09000000;
pub const BASS_INPUT_TYPE_ANALOG: DWORD = 0x0a000000;

// BASS_ChannelSetFX effect types
pub const BASS_FX_DX8_CHORUS: DWORD = 0;
pub const BASS_FX_DX8_COMPRESSOR: DWORD = 1;
pub const BASS_FX_DX8_DISTORTION: DWORD = 2;
pub const BASS_FX_DX8_ECHO: DWORD = 3;
pub const BASS_FX_DX8_FLANGER: DWORD = 4;
pub const BASS_FX_DX8_GARGLE: DWORD = 5;
pub const BASS_FX_DX8_I3DL2REVERB: DWORD = 6;
pub const BASS_FX_DX8_PARAMEQ: DWORD = 7;
pub const BASS_FX_DX8_REVERB: DWORD = 8;
pub const BASS_FX_VOLUME: DWORD = 9;

pub const BASS_DX8_PHASE_NEG_180: DWORD = 0;
pub const BASS_DX8_PHASE_NEG_90: DWORD = 1;
pub const BASS_DX8_PHASE_ZERO: DWORD = 2;
pub const BASS_DX8_PHASE_90: DWORD = 3;
pub const BASS_DX8_PHASE_180: DWORD = 4;

pub const BASS_IOSNOTIFY_INTERRUPT: DWORD = 1; // interruption started
pub const BASS_IOSNOTIFY_INTERRUPT_END: DWORD = 2; // interruption ended