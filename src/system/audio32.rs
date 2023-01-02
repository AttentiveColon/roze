#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::style)]

use crate::system::win32::*;
use std::ffi::c_void;

///////////////////////////////////////////////////////////////////////////////////////////////////
// WIN32 XAudio2 TYPES
///////////////////////////////////////////////////////////////////////////////////////////////////

pub type IXAudio2 = *mut c_void;
pub type IXAudio2MasteringVoice = *mut c_void;
pub type IXAudio2EngineCallback = *mut c_void;
pub type IXAudio2VoiceCallback = *mut c_void;
pub type IXAudio2SourceVoice = *mut c_void;
pub type IXAudio2SubmixVoice = *mut c_void;
pub type IXAudio2Voice = *mut c_void;
pub type XAUDIO2_EFFECT_CHAIN = *mut c_void;
pub type IUnknown = *mut c_void;
pub type XAUDIO2_BUFFER_WMA = *const c_void;
pub type XAUDIO2_PROCESSOR = UINT32;
pub type AUDIO_STREAM_CATEGORY = i32;
pub type XAUDIO2_FILTER_TYPE = i32;

///////////////////////////////////////////////////////////////////////////////////////////////////
// WIN32 XAudio2 CONSTANTS
///////////////////////////////////////////////////////////////////////////////////////////////////

pub const COINIT_MULTITHREADED: COINIT = 0u32;

pub const XAUDIO2_DEFAULT_PROCESSOR: XAUDIO2_PROCESSOR = 1u32;
pub const XAUDIO2_DEFAULT_CHANNELS: u32 = 0u32;
pub const XAUDIO2_DEFAULT_SAMPLERATE: u32 = 0u32;
pub const XAUDIO2_DEFAULT_FREQ_RATIO: f32 = 2.0;
pub const XAUDIO_END_OF_STREAM: u32 = 0x40;
pub const XAUDIO2_COMMIT_NOW: u32 = 0;

pub const AUDIOCATEGORY_GAMEEFFECTS: AUDIO_STREAM_CATEGORY = 6i32;

pub const LOWPASSFILTER: XAUDIO2_FILTER_TYPE = 0i32;

pub const XAUDIO2_LOOP_INFINITE: u32 = 255u32;

///////////////////////////////////////////////////////////////////////////////////////////////////
// WIN32 XAudio2 STRUCTURES
///////////////////////////////////////////////////////////////////////////////////////////////////

// #[repr(C)]
// pub struct XAUDIO2_EFFECT_CHAIN {
//     pub EffectCount: u32,
//     pub pEffectDescriptors: *mut XAUDIO2_EFFECT_DESCRIPTOR,
// }

// #[repr(C)]
// pub struct XAUDIO2_EFFECT_DESCRIPTOR {
//     pub pEffect: IUnknown,
//     pub InitialState: BOOL,
//     pub OutputChannels: u32,
// }

#[repr(C)]
pub struct IXAudio2Struct {
    pub lpVtable: *mut IXAudio2Vtbl,
}

#[repr(C)]
pub struct IXAudio2MasteringVoiceStruct {
    pub lpVtable: *mut IXAudio2MasteringVoiceVtbl,
}

#[repr(C)]
pub struct IXAudio2SourceVoiceStruct {
    pub lpVtable: *mut IXAudio2SourceVoiceVtbl,
}

#[derive(Debug)]
#[repr(C)]
pub struct IXAudio2EngineCallbackStruct {
    pub lpVtable: IXAudio2EngineCallbackVtbl,
}

pub struct AudioData {
    pub data: *const u8,
    pub size: UINT32,
    pub format: WAVEFORMATEXTENSIBLE,
}

#[repr(C)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

#[repr(C, packed(1))]
pub struct XAUDIO2_PERFORMANCE_DATA {
    pub AudioCyclesSinceLastQuery: u64,
    pub TotalCyclesSinceLastQuery: u64,
    pub MinimumCyclesPerQuantum: u32,
    pub MaximumCyclesPerQuantum: u32,
    pub MemoryUsageInBytes: u32,
    pub CurrentLatencyInSamples: u32,
    pub GlitchesSinceEngineStarted: u32,
    pub ActiveSourceVoiceCount: u32,
    pub TotalSourceVoiceCount: u32,
    pub ActiveSubmixVoiceCount: u32,
    pub ActiveResamplerCount: u32,
    pub ActiveMatrixMixCount: u32,
    pub ActiveXmaSourceVoices: u32,
    pub ActiveXmaStreams: u32,
}

#[repr(C, packed(1))]
pub struct XAUDIO2_DEBUG_CONFIGURATION {
    pub TraceMask: u32,
    pub BreakMask: u32,
    pub LogThreadID: BOOL,
    pub LogFileline: BOOL,
    pub LogFunctionName: BOOL,
    pub LogTiming: BOOL,
}

#[repr(C, packed(1))]
pub struct WAVEFORMATEXTENSIBLE {
    pub Format: WAVEFORMATEX,
    pub Samples: WAVEFORMATEXTENSIBLE_0,
    pub dwChannelMask: u32,
    pub SubFormat: GUID,
}

#[derive(Debug)]
#[repr(C)]
pub struct WAVEFORMATEX {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wBitsPerSample: u16,
    pub cbSize: u16,
}

#[repr(C, packed(1))]
pub union WAVEFORMATEXTENSIBLE_0 {
    pub wValidBitsPerSample: u16,
    pub wSamplesPerBlock: u16,
    pub wReserved: u16,
}

#[repr(C, packed(1))]
pub struct XAUDIO2_VOICE_DETAILS {
    pub CreationFlags: u32,
    pub ActiveFlags: u32,
    pub InputChannels: u32,
    pub InputSampleRate: u32,
}

#[repr(C, packed(1))]
pub struct XAUDIO2_VOICE_STATE {
    pub pCurrentBufferContext: *mut c_void,
    pub BuffersQueued: u32,
    pub SamplesPlayed: u64,
}

#[repr(C, packed(1))]
pub struct XAUDIO2_VOICE_SENDS {
    pub SendCount: u32,
    pub pSends: *mut XAUDIO2_SEND_DESCRIPTOR,
}

#[repr(C, packed(1))]
pub struct XAUDIO2_SEND_DESCRIPTOR {
    pub Flags: u32,
    pub pOutputVoice: IXAudio2Voice,
}

#[repr(C, packed(1))]
pub struct XAUDIO2_FILTER_PARAMETERS {
    pub Type: XAUDIO2_FILTER_TYPE,
    pub Frequency: f32,
    pub OneOverQ: f32,
}

#[derive(Debug)]
#[repr(C)]
pub struct XAUDIO2_BUFFER {
    pub Flags: u32,
    pub AudioBytes: u32,
    pub pAudioData: *const u8,
    pub PlayBegin: u32,
    pub PlayLength: u32,
    pub LoopBegin: u32,
    pub LoopLength: u32,
    pub LoopCount: u32,
    pub pContext: *mut c_void,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//V-TABLES
///////////////////////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct IXAudio2Vtbl {
    pub QueryInterface:
        fn(this: *const IXAudio2Struct, guid: *const GUID, *const *const c_void) -> HRESULT,
    pub AddRef: fn(this: *const IXAudio2Struct) -> u32,
    pub Release: fn(this: *const IXAudio2Struct) -> u32,
    pub RegisterForCallbacks:
        fn(this: *const IXAudio2Struct, xa2EC: IXAudio2EngineCallback) -> HRESULT,
    pub UnregisterForCallbacks:
        fn(this: *const IXAudio2Struct, xa2EC: IXAudio2EngineCallback) -> (),

    pub CreateSourceVoice: fn(
        *const IXAudio2Struct,
        *const IXAudio2SourceVoice,
        *const WAVEFORMATEX,
        UINT32,
        f32,
        *const IXAudio2EngineCallback,
        *const XAUDIO2_VOICE_SENDS,
        *const XAUDIO2_EFFECT_CHAIN,
    ) -> HRESULT,
    pub CreateSubmixVoice: fn(
        *const IXAudio2Struct,
        *const IXAudio2SubmixVoice,
        UINT32,
        UINT32,
        UINT32,
        UINT32,
        *const XAUDIO2_VOICE_SENDS,
        *const XAUDIO2_EFFECT_CHAIN,
    ) -> HRESULT,
    pub CreateMasteringVoice: fn(
        this: *const IXAudio2Struct,
        ppMasteringVoice: *mut IXAudio2MasteringVoice,
        inputChannels: UINT32,
        inputSampleRate: UINT32,
        flags: UINT32,
        szDeviceId: LPCWSTR,
        pEffectChain: XAUDIO2_EFFECT_CHAIN,
        streamCategory: AUDIO_STREAM_CATEGORY,
    ) -> HRESULT,

    pub StartEngine: fn(*const IXAudio2Struct) -> HRESULT,
    pub StopEngine: fn(*const IXAudio2Struct) -> (),
    pub CommitChanges: fn(*const IXAudio2Struct, u32) -> HRESULT,

    pub GetPerformanceData: fn(*const IXAudio2Struct, *const XAUDIO2_PERFORMANCE_DATA) -> (),
    pub SetDebugConfiguration:
        fn(*const IXAudio2Struct, *const XAUDIO2_DEBUG_CONFIGURATION, *const c_void) -> (),
}

#[repr(C)]
pub struct IXAudio2MasteringVoiceVtbl {
    pub GetVoiceDetails: fn(*const IXAudio2MasteringVoiceStruct, *mut XAUDIO2_VOICE_DETAILS) -> (),
    pub SetOutputVoices:
        fn(*const IXAudio2MasteringVoiceStruct, *const XAUDIO2_VOICE_SENDS) -> HRESULT,
    pub SetEffectChain:
        fn(*const IXAudio2MasteringVoiceStruct, *const XAUDIO2_EFFECT_CHAIN) -> HRESULT,
    pub EnableEffect: fn(*const IXAudio2MasteringVoiceStruct, UINT, UINT) -> HRESULT,
    pub DisableEffect: fn(*const IXAudio2MasteringVoiceStruct, UINT, UINT) -> HRESULT,
    pub GetEffectState: fn(*const IXAudio2MasteringVoiceStruct, UINT, LPDWORD) -> (),
    pub SetEffectParameters:
        fn(*const IXAudio2MasteringVoiceStruct, UINT, *const c_void, UINT, UINT) -> HRESULT,
    pub GetEffectParameters:
        fn(*const IXAudio2MasteringVoiceStruct, UINT, *mut c_void, UINT) -> HRESULT,
    pub SetFilterParameters:
        fn(*const IXAudio2MasteringVoiceStruct, *const XAUDIO2_FILTER_PARAMETERS, UINT) -> HRESULT,
    pub GetFilterParameters:
        fn(*const IXAudio2MasteringVoiceStruct, *mut XAUDIO2_FILTER_PARAMETERS) -> (),
    pub SetOutputFilterParameters: fn(
        *const IXAudio2MasteringVoiceStruct,
        *mut IXAudio2Voice,
        *const XAUDIO2_FILTER_PARAMETERS,
        UINT,
    ) -> HRESULT,
    pub GetOutputFilterParameters: fn(
        *const IXAudio2MasteringVoiceStruct,
        *mut IXAudio2Voice,
        *mut XAUDIO2_FILTER_PARAMETERS,
    ) -> (),
    pub SetVolume: fn(*const IXAudio2MasteringVoiceStruct, FLOAT, UINT) -> HRESULT,
    pub GetVolume: fn(*const IXAudio2MasteringVoiceStruct, *mut FLOAT) -> (),
    pub SetChannelVolumes:
        fn(*const IXAudio2MasteringVoiceStruct, UINT, *const FLOAT, UINT) -> HRESULT,
    pub GetChannelVolumes: fn(*const IXAudio2MasteringVoiceStruct, UINT, *mut FLOAT) -> (),
    pub SetOutputMatrix: fn(
        *const IXAudio2MasteringVoiceStruct,
        *mut IXAudio2Voice,
        UINT,
        UINT,
        *const FLOAT,
        UINT,
    ) -> HRESULT,
    pub GetOutputMatrix:
        fn(*const IXAudio2MasteringVoiceStruct, *mut IXAudio2Voice, UINT, UINT, *mut FLOAT) -> (),
    pub DestroyVoice: fn(*const IXAudio2MasteringVoiceStruct) -> (),
    pub GetChannelMask: fn(*const IXAudio2MasteringVoiceStruct, PULONG) -> HRESULT,
}

#[repr(C)]
pub struct IXAudio2SourceVoiceVtbl {
    pub GetVoiceDetails: fn(*const IXAudio2SourceVoiceStruct, *mut XAUDIO2_VOICE_DETAILS) -> (),
    pub SetOutputVoices:
        fn(*const IXAudio2SourceVoiceStruct, *const XAUDIO2_VOICE_SENDS) -> HRESULT,
    pub SetEffectChain:
        fn(*const IXAudio2SourceVoiceStruct, *const XAUDIO2_EFFECT_CHAIN) -> HRESULT,
    pub EnableEffect: fn(*const IXAudio2SourceVoiceStruct, UINT, UINT) -> HRESULT,
    pub DisableEffect: fn(*const IXAudio2SourceVoiceStruct, UINT, UINT) -> HRESULT,
    pub GetEffectState: fn(*const IXAudio2SourceVoiceStruct, UINT, *mut LONG) -> (),
    pub SetEffectParameters:
        fn(*const IXAudio2SourceVoiceStruct, UINT, *const c_void, UINT, UINT) -> HRESULT,
    pub GetEffectParameters:
        fn(*const IXAudio2SourceVoiceStruct, UINT, *mut c_void, UINT) -> HRESULT,
    pub SetFilterParameters:
        fn(*const IXAudio2SourceVoiceStruct, *const XAUDIO2_FILTER_PARAMETERS, UINT) -> HRESULT,
    pub GetFilterParameters:
        fn(*const IXAudio2SourceVoiceStruct, *mut XAUDIO2_FILTER_PARAMETERS) -> (),
    pub SetOutputFilterParameters: fn(
        *const IXAudio2SourceVoiceStruct,
        *mut IXAudio2Voice,
        *const XAUDIO2_FILTER_PARAMETERS,
        UINT,
    ) -> HRESULT,
    pub GetOutputFilterParameters: fn(
        *const IXAudio2SourceVoiceStruct,
        *mut IXAudio2Voice,
        *mut XAUDIO2_FILTER_PARAMETERS,
    ) -> (),
    pub SetVolume: fn(*const IXAudio2SourceVoiceStruct, FLOAT, UINT) -> HRESULT,
    pub GetVolume: fn(*const IXAudio2SourceVoiceStruct, *mut FLOAT) -> (),
    pub SetChannelVolumes:
        fn(*const IXAudio2SourceVoiceStruct, UINT, *const FLOAT, UINT) -> HRESULT,
    pub GetChannelVolumes: fn(*const IXAudio2SourceVoiceStruct, UINT, *mut FLOAT) -> (),
    pub SetOutputMatrix: fn(
        *const IXAudio2SourceVoiceStruct,
        *mut IXAudio2Voice,
        UINT,
        UINT,
        *const FLOAT,
        UINT,
    ) -> HRESULT,
    pub GetOutputMatrix:
        fn(*const IXAudio2SourceVoiceStruct, *mut IXAudio2Voice, UINT, UINT, *mut FLOAT) -> (),
    pub DestroyVoice: fn(*const IXAudio2SourceVoiceStruct) -> (),
    pub Start: fn(*const IXAudio2SourceVoiceStruct, UINT, UINT) -> HRESULT,
    pub Stop: fn(*const IXAudio2SourceVoiceStruct, UINT, UINT) -> HRESULT,
    pub SubmitSourceBuffer: fn(
        *const IXAudio2SourceVoiceStruct,
        *const XAUDIO2_BUFFER,
        *const XAUDIO2_BUFFER_WMA,
    ) -> HRESULT,
    pub FlushSourceBuffer: fn(*const IXAudio2SourceVoiceStruct) -> HRESULT,
    pub Discontinuity: fn(*const IXAudio2SourceVoiceStruct) -> HRESULT,
    pub ExitLoop: fn(*const IXAudio2SourceVoiceStruct, UINT) -> HRESULT,
    pub GetState: fn(*const IXAudio2SourceVoiceStruct, *mut XAUDIO2_VOICE_STATE, UINT) -> (),
    pub SetFrequencyRatio: fn(*const IXAudio2SourceVoiceStruct, FLOAT, UINT) -> HRESULT,
    pub GetFrequencyRatio: fn(*const IXAudio2SourceVoiceStruct, *mut FLOAT) -> (),
    pub SetSourceSampleRate: fn(*const IXAudio2SourceVoiceStruct, UINT) -> HRESULT,
}

#[derive(Debug)]
#[repr(C)]
pub struct IXAudio2EngineCallbackVtbl {
    pub OnStreamEnd: fn(*const IXAudio2EngineCallback) -> (),
    pub OnVoiceProcessingPassEnd: fn(*const IXAudio2EngineCallback) -> (),
    pub OnVoiceProcessingPassStart: fn(*const IXAudio2EngineCallback, UINT32) -> (),
    pub OnBufferEnd: fn(*const IXAudio2EngineCallback) -> (),
    pub OnBufferStart: fn(*const IXAudio2EngineCallback, *mut c_void) -> (),
    pub OnLoopEnd: fn(*const IXAudio2EngineCallback, *mut c_void) -> (),
    pub OnVoiceError: fn(*const IXAudio2EngineCallback, *mut c_void, HRESULT) -> (),
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// WIN32 XAudio2 FUNCTIONS
///////////////////////////////////////////////////////////////////////////////////////////////////

#[link(name = "Xaudio2")]
extern "system" {
    pub fn XAudio2Create(
        ppXAudio2: *mut IXAudio2,
        flags: UINT32,
        XAudio2Processor: XAUDIO2_PROCESSOR,
    ) -> HRESULT;
}
