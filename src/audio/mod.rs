use std::{pin::Pin, time::Instant};

use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device, Host, Stream, StreamConfig,
};
use utils::ToSample;

use crate::{asset, structs::BeatBarTick};

pub mod loader;
pub mod utils;

pub struct AudioEngine {
    pub host: Host,
    pub device: Device,
    pub config: StreamConfig,
    stream: Option<Stream>,
}

#[derive(Debug)]
pub enum EngineInitError {
    FailedToInitHost,
    NoOutputDeviceFound,
    FailedToGetOutputConfig,
}

impl AudioEngine {
    pub fn new() -> Result<Self, EngineInitError> {
        #[cfg(target_os = "windows")]
        let Ok(host) = cpal::host_from_id(cpal::HostId::Wasapi) else {
            return Err(EngineInitError::FailedToInitHost);
        };

        let Some(device) = host.default_output_device() else {
            return Err(EngineInitError::NoOutputDeviceFound);
        };

        let Ok(config) = device.default_output_config().map(|val| val.into()) else {
            return Err(EngineInitError::FailedToGetOutputConfig);
        };

        Ok(Self {
            host,
            device,
            config,
            stream: None,
        })
    }

    pub fn init(&mut self) {
        let stream = self
            .device
            .build_output_stream(
                &self.config,
                |data: &mut [f32], info: &cpal::OutputCallbackInfo| {
                    self.write_samples(data, info);
                },
                |err| eprintln!("error: {}", err),
                None,
            )
            .unwrap();

        self.stream = Some(stream);
    }

    pub fn pause(&mut self) -> Result<(), cpal::PauseStreamError> {
        //self.stream.pause()?;
        //self.playback.borrow_mut().previous_call = None;

        Ok(())
    }

    pub fn play(&mut self) -> Result<(), cpal::PlayStreamError> {
        //self.stream.play()?;

        Ok(())
    }

    fn write_samples(&mut self, data: &mut [f32], _: &cpal::OutputCallbackInfo) {}
}

pub struct Playback {
    pub volume: f32,
    pub elapsed: f64,
    pub duration: BeatBarTick,

    pub tempo: f32,
    pub metronome: bool,
    beat: u32,
    metronome_sound: Vec<u16>,
    force_silence: bool,
    previous_beat_elapsed: f64,

    sample_rate: u32,
    previous_call: Option<Instant>,

    temp_offset: usize,
}

impl Default for Playback {
    fn default() -> Self {
        let metronome_sound = loader::load_audio_file(asset!("metronome_tick.ogg")).unwrap();
        Self {
            volume: 0.8,
            elapsed: 0.0,
            duration: BeatBarTick::new(1, 0, 0),

            tempo: 130.0,
            beat: 0,
            metronome: true,
            metronome_sound,
            previous_beat_elapsed: 1.0,
            force_silence: false,

            sample_rate: 0,
            previous_call: None,

            temp_offset: 0,
        }
    }
}

impl Playback {
    pub fn reset_elapsed(&mut self) {
        self.elapsed = 0.0;
        self.previous_beat_elapsed = 1.0;
        self.beat = 0;
        self.previous_call = None;
    }

    pub fn restart(&mut self) {
        self.elapsed = self.elapsed - self.duration.to_seconds(self.tempo);
        self.previous_beat_elapsed = 1.0;
        self.beat = 0;
        self.force_silence = true;
    }

    fn write_samples(&mut self, data: &mut [f32], _: &cpal::OutputCallbackInfo) {
        let sample_rate = self.sample_rate as f64;
        let step_time = data.len() as f64 / sample_rate / 2.0;

        let current_call = Instant::now();
        let mut delta_time = step_time;
        if let Some(previous_call) = self.previous_call {
            delta_time = current_call.duration_since(previous_call).as_secs_f64();
        }
        self.previous_call = Some(current_call);
        let elapsed = self.elapsed + delta_time;

        let beat_duration = 60.0 / self.tempo;
        let beat_elapsed = elapsed % beat_duration as f64;

        if beat_elapsed < self.previous_beat_elapsed {
            if !self.force_silence {
                self.temp_offset = 0;
            }

            self.force_silence = false;

            if elapsed >= beat_duration as f64 {
                self.beat += 1;

                if self.beat >= 4 {
                    self.beat = 0;
                }
            }
        }
        self.previous_beat_elapsed = beat_elapsed;

        let volume = self.volume;
        let stretch = if self.beat == 0 { 1.5 } else { 1.0 };

        let mut i = self.temp_offset as f32;
        for sample in data.iter_mut() {
            *sample = 0.0;

            if (i as usize) < self.metronome_sound.len() && self.metronome {
                *sample += self.metronome_sound[i as usize].to_sample();
            }

            *sample *= volume;
            i += stretch;
        }

        self.temp_offset = i as usize;
        self.elapsed = elapsed;
        if self.elapsed >= self.duration.to_seconds(self.tempo) {
            self.restart();
        }
    }
}
