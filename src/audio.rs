#![allow(dead_code)]

use crate::system::audio32::*;
use crate::system::win32::*;

use std::collections::HashMap;
use std::ffi::c_void;

use std::fs::File;
use std::io::Read;

#[allow(non_snake_case)]
mod callbacks {
    use super::*;

    pub fn OnStreamEnd(_this: *const IXAudio2EngineCallback) {}
    pub fn OnVoiceProcessingPassEnd(_this: *const IXAudio2EngineCallback) {}
    pub fn OnVoiceProcessingPassStart(
        _this: *const IXAudio2EngineCallback,
        _samplesRequired: UINT32,
    ) {
    }
    pub fn OnBufferEnd(_this: *const IXAudio2EngineCallback) {}
    pub fn OnBufferStart(_this: *const IXAudio2EngineCallback, _pBufferContext: *mut c_void) {}
    pub fn OnLoopEnd(_this: *const IXAudio2EngineCallback, _pBufferContext: *mut c_void) {}
    pub fn OnVoiceError(
        _this: *const IXAudio2EngineCallback,
        _pBufferContext: *mut c_void,
        _error: HRESULT,
    ) {
    }
}

use self::callbacks::*;
static ENGINE_CALLBACKS: IXAudio2EngineCallbackStruct = IXAudio2EngineCallbackStruct {
    lpVtable: IXAudio2EngineCallbackVtbl {
        OnStreamEnd,
        OnVoiceProcessingPassEnd,
        OnVoiceProcessingPassStart,
        OnBufferEnd,
        OnBufferStart,
        OnLoopEnd,
        OnVoiceError,
    },
};

#[derive(PartialEq)]
pub enum VoiceType {
    OneShot,
    Loop,
}

pub struct AudioDataVec {
    pub data: Vec<u8>,
    pub size: UINT32,
    pub format: WAVEFORMATEXTENSIBLE,
}

#[derive(Debug)]
pub struct AudioBuffer {
    audio_buffer: XAUDIO2_BUFFER,
    _raw_buffer: Vec<u8>,
}

pub struct Audio {
    pub device: *const IXAudio2Struct,
    pub master_voice: *const IXAudio2MasteringVoiceStruct,
    pub format: WAVEFORMATEX,
    pub engine_callbacks: *const IXAudio2EngineCallbackStruct,

    pub loops: Vec<*const IXAudio2SourceVoiceStruct>,
    pub one_shots: Vec<*const IXAudio2SourceVoiceStruct>,
    pub current_one_shot: u8,

    pub audio_data: HashMap<String, AudioBuffer>,
}

impl std::fmt::Debug for Audio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "device: {:p}\nmaster voice: {:p}\nengine callbacks: {:p}\n",
            &self.device, &self.master_voice, &self.engine_callbacks,
        )
    }
}

impl Audio {
    pub fn new(one_shot_voices: u8, loop_voices: u8) -> Box<Audio> {
        let mut audio = Box::new(Audio::initialize());
        audio.create_source_voices(one_shot_voices, VoiceType::OneShot);
        audio.create_source_voices(loop_voices, VoiceType::Loop);
        audio.start_one_shots();
        audio
    }
    pub fn initialize() -> Audio {
        unsafe {
            let mut device: IXAudio2 = std::ptr::null_mut();
            let mut master_voice: IXAudio2MasteringVoice = std::ptr::null_mut();

            let mut hr = CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED);
            failed(hr, "CoInit Failed");

            hr = XAudio2Create(&mut device, 0, XAUDIO2_DEFAULT_PROCESSOR);
            let device = device as *const IXAudio2Struct;
            failed(hr, "XAudio2 Device Creation Failed");

            hr = ((*(*device).lpVtable).CreateMasteringVoice)(
                device,
                &mut master_voice,
                XAUDIO2_DEFAULT_CHANNELS,
                XAUDIO2_DEFAULT_SAMPLERATE,
                0,
                std::ptr::null(),
                std::ptr::null_mut(),
                AUDIOCATEGORY_GAMEEFFECTS,
            );
            let master_voice = master_voice as *const IXAudio2MasteringVoiceStruct;
            failed(hr, "XAudio2 Mastering Voice Creation Failed");

            let engine_callbacks = &ENGINE_CALLBACKS;

            let format: WAVEFORMATEX = WAVEFORMATEX {
                wFormatTag: 1,
                nChannels: 2,
                nSamplesPerSec: 44100,
                nAvgBytesPerSec: 176400,
                nBlockAlign: 4,
                wBitsPerSample: 16,
                cbSize: 0,
            };

            Self {
                device,
                master_voice,
                format,
                engine_callbacks,

                loops: Vec::new(),
                one_shots: Vec::new(),
                current_one_shot: 0,

                audio_data: HashMap::new(),
            }
        }
    }

    pub fn get_device(&self) -> &IXAudio2Struct {
        unsafe { &*self.device }
    }

    pub fn get_buffer(&self, sound_name: &str) -> *const XAUDIO2_BUFFER {
        let sound = self.audio_data.get(sound_name);
        let ptr = &sound.unwrap().audio_buffer;
        ptr as *const XAUDIO2_BUFFER
    }

    pub fn create_source_voices(&mut self, voices: u8, voice_type: VoiceType) {
        for _ in 0..voices {
            let mut source_voice: IXAudio2SourceVoice = std::ptr::null_mut();

            unsafe {
                let hr = ((*(*self.device).lpVtable).CreateSourceVoice)(
                    self.device,
                    &mut source_voice as *const IXAudio2SourceVoice,
                    &self.format,
                    0,
                    XAUDIO2_DEFAULT_FREQ_RATIO,
                    &self.engine_callbacks as *const _ as *const *mut c_void,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );
                failed(hr, "XAudio2 Source Voice Creation Failed");
            }
            if voice_type == VoiceType::OneShot {
                self.one_shots
                    .push(source_voice as *const IXAudio2SourceVoiceStruct);
            } else {
                self.loops
                    .push(source_voice as *const IXAudio2SourceVoiceStruct);
            }
        }
    }

    pub fn load_audio(&mut self, filename: &str) {
        let extension = std::path::Path::new(&filename).extension().unwrap();
        if extension.to_ascii_lowercase() != "wav" {
            eprintln!("Couldn't load: {filename} - Unsupported file format");
            return;
        }

        let audio_data = buf_read_file(filename);
        if audio_data.data.as_ptr().is_null() {
            eprintln!("Failed to load audio data: {filename}");
            return;
        }
        let key = std::path::Path::new(&filename)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        unsafe {
            let mut audio_buffer: XAUDIO2_BUFFER = std::mem::zeroed();

            audio_buffer.AudioBytes = audio_data.size;
            audio_buffer.pAudioData = audio_data.data.as_ptr();
            audio_buffer.Flags = XAUDIO_END_OF_STREAM;

            let audio = AudioBuffer {
                audio_buffer,
                _raw_buffer: audio_data.data,
            };

            self.audio_data.insert(String::from(key), audio);
        }
    }

    pub fn load_assets(&mut self, folder: &str) {
        use std::fs;
        use std::path::Path;
        let path = Path::new(folder);

        if !path.is_dir() {
            eprintln!("Invalid audio directory: {folder}");
            return;
        }

        let entries = fs::read_dir(path).unwrap();

        for entry in entries {
            let file_path = entry
                .unwrap()
                .path()
                .into_os_string()
                .into_string()
                .unwrap();

            #[cfg(debug_assertions)]
            println!("Audio File Loaded: {}", file_path);
            self.load_audio(&file_path);
        }
    }

    pub fn start_one_shots(&self) {
        for i in 0..self.one_shots.len() {
            unsafe {
                let hr = (*(self.one_shots[i])).start();
                failed(hr, "Couldn't start source voice");
            }
        }
    }

    pub fn one_shot(&mut self, audio: &str, level: f32) {
        let audio_buffer = self.get_buffer(audio);
        unsafe {
            if self.current_one_shot as usize >= self.one_shots.len() {
                self.current_one_shot = 0;
            }
            let voice = &(*(*self.one_shots)[self.current_one_shot as usize]);
            voice.stop();
            voice.flush();
            voice.set_volume(level);
            voice.submit(audio_buffer, std::ptr::null_mut());
            voice.start();
            self.current_one_shot += 1;
        }
    }

    pub fn submit_loop(&mut self, audio: &str, channel: u8, level: f32) {
        if channel as usize >= self.loops.len() {
            println!("Channel {} does not exist", channel);
            return;
        }
        let audio_buffer = self.get_buffer(audio);
        unsafe {
            let audio_buffer = &*audio_buffer;
            let audio_buffer = XAUDIO2_BUFFER {
                Flags: audio_buffer.Flags,
                AudioBytes: audio_buffer.AudioBytes,
                pAudioData: audio_buffer.pAudioData,
                PlayBegin: audio_buffer.PlayBegin,
                PlayLength: audio_buffer.PlayLength,
                LoopBegin: audio_buffer.PlayBegin,
                LoopLength: audio_buffer.PlayLength,
                LoopCount: XAUDIO2_LOOP_INFINITE,
                pContext: audio_buffer.pContext,
            };

            let voice = &(*(*self.loops)[channel as usize]);
            voice.stop();
            voice.flush();
            voice.set_volume(level);
            voice.submit(&audio_buffer as *const XAUDIO2_BUFFER, std::ptr::null_mut());
            voice.start();
        }
    }

    pub fn stop_loop(&mut self, channel: u8) {
        if channel as usize >= self.loops.len() {
            println!("Channel {} does not exist", channel);
            return;
        }
        unsafe {
            let voice = &(*(*self.loops)[channel as usize]);
            voice.stop();
            voice.flush();
        }
    }

    pub fn set_master_volume(&self, value: f32) {
        unsafe {
            (*self.master_voice).set_volume(value);
        }
    }
}

impl IXAudio2SourceVoiceStruct {
    pub fn start(&self) -> HRESULT {
        unsafe { ((*(self.lpVtable)).Start)(self, 0, XAUDIO2_COMMIT_NOW) }
    }
    pub fn submit(
        &self,
        audio_buffer: *const XAUDIO2_BUFFER,
        buffer_wma: *const XAUDIO2_BUFFER_WMA,
    ) -> HRESULT {
        unsafe { ((*(self.lpVtable)).SubmitSourceBuffer)(self, audio_buffer, buffer_wma) }
    }
    pub fn stop(&self) -> HRESULT {
        unsafe { ((*(self.lpVtable)).Stop)(self, 0, 0) }
    }
    pub fn flush(&self) -> HRESULT {
        unsafe { ((*(self.lpVtable)).FlushSourceBuffer)(self) }
    }
    pub fn set_volume(&self, level: f32) -> HRESULT {
        unsafe { ((*(self.lpVtable)).SetVolume)(self, level, 0) }
    }
}

impl IXAudio2MasteringVoiceStruct {
    pub fn set_volume(&self, level: f32) -> HRESULT {
        unsafe { ((*(self.lpVtable)).SetVolume)(self, level, 0) }
    }
}

//-------------------------------------------------------------------------------------------------
//HELPERS
//-------------------------------------------------------------------------------------------------

pub fn failed(value: i32, error: &str) {
    if value < 0 {
        eprintln!("Error: {}", error);
        panic!();
    }
}

pub fn buf_read_file(filename: &str) -> AudioDataVec {
    let mut file = File::open(filename).unwrap();

    let chunk_type = read_chunk(&mut file);
    if chunk_type != 1179011410 {
        panic!("First Chunk Invalid");
    }

    let _chunk_data_size = read_chunk(&mut file);
    let file_format = read_chunk(&mut file);
    if file_format != 1163280727 {
        panic!("Wrong File Format");
    }

    let chunk_type = read_chunk(&mut file);
    if chunk_type != 544501094 {
        panic!("Bad 'fmt ' subchunk");
    }

    let chunk_data_size = read_chunk(&mut file);
    let format = read_wav_format(&mut file, chunk_data_size);
    let chunk_type = read_chunk(&mut file);
    if chunk_type != 1635017060 {
        panic!("Bad Subchunk");
    }

    let size = read_chunk(&mut file);
    let mut data = Vec::with_capacity(chunk_data_size as usize);
    file.read_to_end(&mut data).unwrap();

    AudioDataVec { data, size, format }
}

pub fn read_wav_format(file: &mut File, size: u32) -> WAVEFORMATEXTENSIBLE {
    let vaules: Vec<_> = file
        .by_ref()
        .bytes()
        .take(size as usize)
        .map(|x| x.unwrap())
        .collect();
    unsafe {
        let format: WAVEFORMATEXTENSIBLE = std::ptr::read(vaules.as_ptr() as *const _);
        format
    }
}

pub fn read_chunk(file: &mut File) -> u32 {
    let values: Vec<_> = file
        .by_ref()
        .bytes()
        .take(4)
        .map(|x| x.unwrap() as u32)
        .collect();
    values[3] << 24 | values[2] << 16 | values[1] << 8 | values[0]
}
