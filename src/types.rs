use std::os::raw::{c_char, c_void};

pub type BYTE = u8;
pub type WORD = u16;
pub type DWORD = u32;
pub type QWORD = u64;
pub type BOOL = i32;

pub type HMUSIC = DWORD;
pub type HSAMPLE = DWORD;
pub type HCHANNEL = DWORD;
pub type HSTREAM = DWORD;
pub type HRECORD = DWORD;
pub type HSYNC = DWORD;
pub type HDSP = DWORD;
pub type HFX = DWORD;
pub type HPLUGIN = DWORD;

pub type STREAMPROC = extern "C" fn(HSTREAM, *mut c_void, DWORD, *mut c_void) -> DWORD;
pub type FILECLOSEPROC = extern "C" fn(*mut c_void);
pub type FILELENPROC = extern "C" fn(*mut c_void) -> QWORD;
pub type FILEREADPROC = extern "C" fn(*mut c_void, DWORD, *mut c_void) -> DWORD;
pub type FILESEEKPROC = extern "C" fn(QWORD, *mut c_void) -> BOOL;
pub type DOWNLOADPROC = extern "C" fn(*mut c_void, DWORD, *mut c_void);
pub type SYNCPROC = extern "C" fn(HSYNC, DWORD, DWORD, *mut c_void);
pub type DSPPROC = extern "C" fn(HDSP, DWORD, *mut c_void, DWORD, *mut c_void);
pub type RECORDPROC = extern "C" fn(HRECORD, *mut c_void, DWORD, *mut c_void) -> BOOL;
pub type IOSNOTIFYPROC = extern "C" fn(DWORD);

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BassDeviceInfo {
    pub name: *const c_void,
    pub driver: *const c_void,
    pub flags: DWORD,
}

impl BassDeviceInfo {
    pub fn new(name: *const c_void, driver: *const c_void, flags: DWORD) -> Self {
        Self {
            name,
            driver,
            flags,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassInfo {
    pub flags: DWORD,
    pub size_of_hardware_memory: DWORD,
    pub size_of_free_hardware_memory: DWORD,
    pub free_samples: DWORD,
    pub free_3d_sample_slots: DWORD,
    pub minimum_sample_rate: DWORD,
    pub maxiumum_sample_rate: DWORD,
    pub eax_support: BOOL,
    pub minimum_buffer_length: DWORD,
    pub direct_sound_version: DWORD,
    pub latency: DWORD,
    pub init_flags: DWORD,
    pub speakers: DWORD,
    pub output_frequency: DWORD,
}

impl BassInfo {
    pub fn new(
        flags: DWORD,
        size_of_hardware_memory: DWORD,
        size_of_free_hardware_memory: DWORD,
        free_samples: DWORD,
        free_3d_sample_slots: DWORD,
        minimum_sample_rate: DWORD,
        maxiumum_sample_rate: DWORD,
        eax_support: BOOL,
        minimum_buffer_length: DWORD,
        direct_sound_version: DWORD,
        latency: DWORD,
        init_flags: DWORD,
        speakers: DWORD,
        output_frequency: DWORD,
    ) -> Self {
        Self {
            flags,
            size_of_hardware_memory,
            size_of_free_hardware_memory,
            free_samples,
            free_3d_sample_slots,
            minimum_sample_rate,
            maxiumum_sample_rate,
            eax_support,
            minimum_buffer_length,
            direct_sound_version,
            latency,
            init_flags,
            speakers,
            output_frequency,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassRecordInfo {
    pub flags: DWORD,
    pub supported_formats: DWORD,
    pub number_of_inputs: DWORD,
    pub single_in: BOOL,
    pub current_frequency: DWORD,
}

impl BassRecordInfo {
    pub fn new(
        flags: DWORD,
        supported_formats: DWORD,
        number_of_inputs: DWORD,
        single_in: BOOL,
        current_frequency: DWORD,
    ) -> Self {
        Self {
            flags,
            supported_formats,
            number_of_inputs,
            single_in,
            current_frequency,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassSample {
    pub default_frequency: DWORD,
    pub volume: f32,
    pub pan: f32,
    pub flags: DWORD,
    pub length: DWORD,
    pub maximum_simultaneous_playbacks: DWORD,
    pub original_resolution: DWORD,
    pub number_of_channels: DWORD,
    pub minimum_gap: DWORD,
    pub mode_3d: DWORD,
    pub minimum_distance: f32,
    pub maximum_distance: f32,
    pub angle_of_inside_projection_cone: DWORD,
    pub angle_of_outside_projection_cone: DWORD,
    pub volume_delta_of_outside_projection_cone: f32,
    pub voice_allocation_flags: DWORD,
    pub priority: DWORD,
}

impl BassSample {
    pub fn new(
        default_frequency: DWORD,
        volume: f32,
        pan: f32,
        flags: DWORD,
        length: DWORD,
        maximum_simultaneous_playbacks: DWORD,
        original_resolution: DWORD,
        number_of_channels: DWORD,
        minimum_gap: DWORD,
        mode_3d: DWORD,
        minimum_distance: f32,
        maximum_distance: f32,
        angle_of_inside_projection_cone: DWORD,
        angle_of_outside_projection_cone: DWORD,
        volume_delta_of_outside_projection_cone: f32,
        voice_allocation_flags: DWORD,
        priority: DWORD,
    ) -> Self {
        Self {
            default_frequency,
            volume,
            pan,
            flags,
            length,
            maximum_simultaneous_playbacks,
            original_resolution,
            number_of_channels,
            minimum_gap,
            mode_3d,
            minimum_distance,
            maximum_distance,
            angle_of_inside_projection_cone,
            angle_of_outside_projection_cone,
            volume_delta_of_outside_projection_cone,
            voice_allocation_flags,
            priority,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BassChannelInfo {
    pub default_frequency: DWORD,
    pub channels: DWORD,
    pub flags: DWORD,
    pub type_of_channel: DWORD,
    pub original_resolution: DWORD,
    pub plugin: HPLUGIN,
    pub sample: HSAMPLE,
    pub file_name: *const c_char,
}

impl BassChannelInfo {
    pub fn new(
        default_frequency: DWORD,
        channels: DWORD,
        flags: DWORD,
        type_of_channel: DWORD,
        original_resolution: DWORD,
        plugin: HPLUGIN,
        sample: HSAMPLE,
        file_name: *const c_char,
    ) -> Self {
        Self {
            default_frequency,
            channels,
            flags,
            type_of_channel,
            original_resolution,
            plugin,
            sample,
            file_name,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BassPluginForm {
    pub name: *const c_void,
    pub file_extension_filter: *const c_void,
}

impl BassPluginForm {
    pub fn new(name: *const c_void, file_extension_filter: *const c_void) -> Self {
        Self {
            name,
            file_extension_filter,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BassPluginInfo {
    pub version: DWORD,
    pub format_count: DWORD,
    pub formats: *mut BassPluginForm,
}

impl BassPluginInfo {
    pub fn new(version: DWORD, format_count: DWORD, formats: *mut BassPluginForm) -> Self {
        Self {
            version,
            format_count,
            formats,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct Bass3DVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Bass3DVector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BassFileProcs {
    pub close_handle: *mut FILECLOSEPROC,
    pub length_handle: *mut FILELENPROC,
    pub read_handle: *mut FILEREADPROC,
    pub seek_handle: *mut FILESEEKPROC,
}

impl BassFileProcs {
    pub fn new(
        close_handle: *mut FILECLOSEPROC,
        length_handle: *mut FILELENPROC,
        read_handle: *mut FILEREADPROC,
        seek_handle: *mut FILESEEKPROC,
    ) -> Self {
        Self {
            close_handle,
            length_handle,
            read_handle,
            seek_handle,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct TagId3 {
    pub id: [c_char; 3],
    pub title: [c_char; 30],
    pub artist: [c_char; 30],
    pub album: [c_char; 30],
    pub year: [c_char; 4],
    pub comment: [c_char; 30],
    pub genre: BYTE,
}

impl TagId3 {
    pub fn new(
        id: [c_char; 3],
        title: [c_char; 30],
        artist: [c_char; 30],
        album: [c_char; 30],
        year: [c_char; 4],
        comment: [c_char; 30],
        genre: BYTE,
    ) -> Self {
        Self {
            id,
            title,
            artist,
            album,
            year,
            comment,
            genre,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TagApeBinary {
    pub key: *mut c_char,
    pub data: *mut c_void,
    pub length: DWORD,
}

impl TagApeBinary {
    pub fn new(key: *mut c_char, data: *mut c_void, length: DWORD) -> Self {
        Self { key, data, length }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TagBext {
    pub description: [c_char; 256],
    pub originator: [c_char; 32],
    pub originator_reference: [c_char; 32],
    pub originator_date: [c_char; 10],
    pub originator_time: [c_char; 8],
    pub time_reference: QWORD,
    pub version: WORD,
    pub umid: [BYTE; 64],
    pub reserved: [BYTE; 190],
    pub coding_history: Vec<c_char>,
}

impl TagBext {
    pub fn new(
        description: [c_char; 256],
        originator: [c_char; 32],
        originator_reference: [c_char; 32],
        originator_date: [c_char; 10],
        originator_time: [c_char; 8],
        time_reference: QWORD,
        version: WORD,
        umid: [BYTE; 64],
        reserved: [BYTE; 190],
        coding_history: Vec<c_char>,
    ) -> Self {
        Self {
            description,
            originator,
            originator_reference,
            originator_date,
            originator_time,
            time_reference,
            version,
            umid,
            reserved,
            coding_history,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct TagCartTimer {
    pub timer_usage_id: DWORD,
    pub timer_value: DWORD,
}

impl TagCartTimer {
    pub fn new(timer_usage_id: DWORD, timer_value: DWORD) -> Self {
        Self {
            timer_usage_id,
            timer_value,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TagCart {
    pub version: [c_char; 4],
    pub title: [c_char; 64],
    pub artist: [c_char; 64],
    pub cut_id: [c_char; 64],
    pub client_id: [c_char; 64],
    pub category: [c_char; 64],
    pub classification: [c_char; 64],
    pub out_cue: [c_char; 64],
    pub start_date: [c_char; 10],
    pub start_time: [c_char; 8],
    pub end_date: [c_char; 10],
    pub end_time: [c_char; 8],
    pub procuder_app_id: [c_char; 64],
    pub producer_app_version: [c_char; 64],
    pub user_text: [c_char; 64],
    pub sample_value_reference: DWORD,
    pub post_timer: [TagCartTimer; 8],
    pub reserved: [c_char; 276],
    pub url: [c_char; 1024],
    pub tag_text: Vec<c_char>,
}

impl TagCart {
    pub fn new(
        version: [c_char; 4],
        title: [c_char; 64],
        artist: [c_char; 64],
        cut_id: [c_char; 64],
        client_id: [c_char; 64],
        category: [c_char; 64],
        classification: [c_char; 64],
        out_cue: [c_char; 64],
        start_date: [c_char; 10],
        start_time: [c_char; 8],
        end_date: [c_char; 10],
        end_time: [c_char; 8],
        procuder_app_id: [c_char; 64],
        producer_app_version: [c_char; 64],
        user_text: [c_char; 64],
        sample_value_reference: DWORD,
        post_timer: [TagCartTimer; 8],
        reserved: [c_char; 276],
        url: [c_char; 1024],
        tag_text: Vec<c_char>,
    ) -> Self {
        Self {
            version,
            title,
            artist,
            cut_id,
            client_id,
            category,
            classification,
            out_cue,
            start_date,
            start_time,
            end_date,
            end_time,
            procuder_app_id,
            producer_app_version,
            user_text,
            sample_value_reference,
            post_timer,
            reserved,
            url,
            tag_text,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct TagCuePoint {
    pub name: DWORD,
    pub position: DWORD,
    pub chunk: DWORD,
    pub chun_start: DWORD,
    pub block_start: DWORD,
    pub sample_offset: DWORD,
}

impl TagCuePoint {
    pub fn new(
        name: DWORD,
        position: DWORD,
        chunk: DWORD,
        chun_start: DWORD,
        block_start: DWORD,
        sample_offset: DWORD,
    ) -> Self {
        Self {
            name,
            position,
            chunk,
            chun_start,
            block_start,
            sample_offset,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct TagCue {
    pub cue_points_count: DWORD,
    pub cue_points: Vec<TagCuePoint>,
}

impl TagCue {
    pub fn new(cue_points_count: DWORD, cue_points: Vec<TagCuePoint>) -> Self {
        Self {
            cue_points_count,
            cue_points,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct TagSampleLoop {
    pub identifier: DWORD,
    pub sample_type: DWORD,
    pub start: DWORD,
    pub end: DWORD,
    pub fraction: DWORD,
    pub play_count: DWORD,
}

impl TagSampleLoop {
    pub fn new(
        identifier: DWORD,
        sample_type: DWORD,
        start: DWORD,
        end: DWORD,
        fraction: DWORD,
        play_count: DWORD,
    ) -> Self {
        Self {
            identifier,
            sample_type,
            start,
            end,
            fraction,
            play_count,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct TagSample {
    pub manufacturer: DWORD,
    pub product: DWORD,
    pub sample_period: DWORD,
    pub midi_unity_note: DWORD,
    pub midi_pitch_fraction: DWORD,
    pub smpte_format: DWORD,
    pub smpte_offset: DWORD,
    pub sample_loops_count: DWORD,
    pub sampler_data: DWORD,
    pub sample_loops: Vec<TagSampleLoop>,
}

impl TagSample {
    pub fn new(
        manufacturer: DWORD,
        product: DWORD,
        sample_period: DWORD,
        midi_unity_note: DWORD,
        midi_pitch_fraction: DWORD,
        smpte_format: DWORD,
        smpte_offset: DWORD,
        sample_loops_count: DWORD,
        sampler_data: DWORD,
        sample_loops: Vec<TagSampleLoop>,
    ) -> Self {
        Self {
            manufacturer,
            product,
            sample_period,
            midi_unity_note,
            midi_pitch_fraction,
            smpte_format,
            smpte_offset,
            sample_loops_count,
            sampler_data,
            sample_loops,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TagCaCodec {
    pub file_type: DWORD,
    pub audio_format: DWORD,
    pub description: *mut c_char,
}

impl TagCaCodec {
    pub fn new(file_type: DWORD, audio_format: DWORD, description: *mut c_char) -> Self {
        Self {
            file_type,
            audio_format,
            description,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct WaveFormatEx {
    pub format_tag: WORD,
    pub channels_number: WORD,
    pub samples_per_second: DWORD,
    pub average_bytes_per_second: DWORD,
    pub block_align: WORD,
    pub bits_per_sample: WORD,
    pub size: WORD,
}

impl WaveFormatEx {
    pub fn new(
        format_tag: WORD,
        channels_number: WORD,
        samples_per_second: DWORD,
        average_bytes_per_second: DWORD,
        block_align: WORD,
        bits_per_sample: WORD,
        size: WORD,
    ) -> Self {
        Self {
            format_tag,
            channels_number,
            samples_per_second,
            average_bytes_per_second,
            block_align,
            bits_per_sample,
            size,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassDx8Chorus {
    pub wet_dry_mix: f32,
    pub depth: f32,
    pub feedback: f32,
    pub frequency: f32,
    pub waveform: DWORD,
    pub delay: f32,
    pub phase: DWORD,
}

impl BassDx8Chorus {
    pub fn new(
        wet_dry_mix: f32,
        depth: f32,
        feedback: f32,
        frequency: f32,
        waveform: DWORD,
        delay: f32,
        phase: DWORD,
    ) -> Self {
        assert!((0..=1).contains(&waveform));

        Self {
            wet_dry_mix,
            depth,
            feedback,
            frequency,
            waveform,
            delay,
            phase,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassDx8Compressor {
    pub gain: f32,
    pub attack: f32,
    pub release: f32,
    pub threshold: f32,
    pub ratio: f32,
    pub predelay: f32,
}

impl BassDx8Compressor {
    pub fn new(
        gain: f32,
        attack: f32,
        release: f32,
        threshold: f32,
        ratio: f32,
        predelay: f32,
    ) -> Self {
        Self {
            gain,
            attack,
            release,
            threshold,
            ratio,
            predelay,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassDx8Distortion {
    pub gain: f32,
    pub edge: f32,
    pub post_eq_center_frequency: f32,
    pub post_eq_bandwidth: f32,
    pub pre_lowpass_cutoff: f32,
}

impl BassDx8Distortion {
    pub fn new(
        gain: f32,
        edge: f32,
        post_eq_center_frequency: f32,
        post_eq_bandwidth: f32,
        pre_lowpass_cutoff: f32,
    ) -> Self {
        Self {
            gain,
            edge,
            post_eq_center_frequency,
            post_eq_bandwidth,
            pre_lowpass_cutoff,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassDx8Echo {
    pub wet_dry_mix: f32,
    pub feedback: f32,
    pub left_delay: f32,
    pub right_delay: f32,
    pub pan_delay: f32,
}

impl BassDx8Echo {
    pub fn new(
        wet_dry_mix: f32,
        feedback: f32,
        left_delay: f32,
        right_delay: f32,
        pan_delay: f32,
    ) -> Self {
        Self {
            wet_dry_mix,
            feedback,
            left_delay,
            right_delay,
            pan_delay,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassDx8Flanger {
    pub wet_dry_mix: f32,
    pub depth: f32,
    pub feedback: f32,
    pub frequency: f32,
    pub waveform: DWORD,
    pub delay: f32,
    pub phase: DWORD,
}

impl BassDx8Flanger {
    pub fn new(
        wet_dry_mix: f32,
        depth: f32,
        feedback: f32,
        frequency: f32,
        waveform: DWORD,
        delay: f32,
        phase: DWORD,
    ) -> Self {
        assert!((0..=1).contains(&waveform));

        Self {
            wet_dry_mix,
            depth,
            feedback,
            frequency,
            waveform,
            delay,
            phase,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassDx8Gargle {
    pub rate_hz: DWORD,
    pub wave_shape: DWORD,
}

impl BassDx8Gargle {
    pub fn new(rate_hz: DWORD, wave_shape: DWORD) -> Self {
        assert!((0..=1).contains(&wave_shape));

        Self {
            rate_hz,
            wave_shape,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassDx8I3Dl2Reverb {
    pub room: i32,
    pub room_hf: i32,
    pub room_rolloff_factor: f32,
    pub decay_time: f32,
    pub decay_hf_ratio: f32,
    pub reflections: i32,
    pub reflections_delay: f32,
    pub reverb: i32,
    pub reverb_delay: f32,
    pub diffusion: f32,
    pub density: f32,
    pub hf_reference: f32,
}

impl BassDx8I3Dl2Reverb {
    pub fn new(
        room: i32,
        room_hf: i32,
        room_rolloff_factor: f32,
        decay_time: f32,
        decay_hf_ratio: f32,
        reflections: i32,
        reflections_delay: f32,
        reverb: i32,
        reverb_delay: f32,
        diffusion: f32,
        density: f32,
        hf_reference: f32,
    ) -> Self {
        assert!((-10000..=0).contains(&room));
        assert!((-10000..=0).contains(&room_hf));
        assert!((0.0..=10.0).contains(&room_rolloff_factor));
        assert!((0.1..=20.0).contains(&decay_time));
        assert!((0.1..=2.0).contains(&decay_hf_ratio));
        assert!((-10000..=1000).contains(&reflections));
        assert!((0.0..=0.3).contains(&reflections_delay));
        assert!((-10000..=2000).contains(&reverb));
        assert!((0.0..=0.1).contains(&reverb_delay));
        assert!((0.0..=100.0).contains(&diffusion));
        assert!((0.0..=100.0).contains(&density));
        assert!((20.0..=20000.0).contains(&hf_reference));

        Self {
            room,
            room_hf,
            room_rolloff_factor,
            decay_time,
            decay_hf_ratio,
            reflections,
            reflections_delay,
            reverb,
            reverb_delay,
            diffusion,
            density,
            hf_reference,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassDx8ParamEq {
    pub center: f32,
    pub bandwidth: f32,
    pub gain: f32,
}

impl BassDx8ParamEq {
    pub fn new(center: f32, bandwidth: f32, gain: f32) -> Self {
        Self {
            center,
            bandwidth,
            gain,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassDx8Reverb {
    pub gain: f32,
    pub reverb_mix: f32,
    pub reverb_time: f32,
    pub high_frequency_rt_ratio: f32,
}

impl BassDx8Reverb {
    pub fn new(gain: f32, reverb_mix: f32, reverb_time: f32, high_frequency_rt_ratio: f32) -> Self {
        assert!((-96.0..=0.0).contains(&gain));
        assert!((-96.0..=0.0).contains(&reverb_mix));
        assert!((0.001..=3000.0).contains(&reverb_time));
        assert!((0.001..=0.999).contains(&high_frequency_rt_ratio));

        Self {
            gain,
            reverb_mix,
            reverb_time,
            high_frequency_rt_ratio,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassFxVolumeParam {
    pub target: f32,
    pub current: f32,
    pub time: f32,
    pub curve: DWORD,
}

impl BassFxVolumeParam {
    pub fn new(target: f32, current: f32, time: f32, curve: DWORD) -> Self {
        Self {
            target,
            current,
            time,
            curve,
        }
    }
}
