#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod audio;
mod panels;
mod structs;
mod ui;
mod utils;

use audio::AudioEngine;
use eframe::egui::{self};
use panels::{
    arranger::ArrangerPanel, files::FilesPanel, mixer::MixerPanel, piano::PianoPanel,
    settings::SettingsPanel,
};

fn main() {
    let result = eframe::run_native(
        "PhaseLab",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default(),
            ..Default::default()
        },
        Box::new(|ctx| {
            egui_extras::install_image_loaders(&ctx.egui_ctx);
            Ok(Box::<App>::default())
        }),
    );

    if let Err(e) = result {
        eprintln!("error: {}", e);
    }
}

#[allow(unused)]
struct App {
    audio_engine: AudioEngine,
    playing: bool,

    files_panel: FilesPanel,
    files_shown: bool,

    mixer_panel: MixerPanel,
    mixer_shown: bool,

    arranger_panel: ArrangerPanel,
    arranger_shown: bool,

    piano_panel: PianoPanel,
    piano_shown: bool,

    settings_panel: SettingsPanel,
    settings_shown: bool,
}

impl Default for App {
    fn default() -> Self {
        let mut engine = AudioEngine::new().expect("Failed to create audio engine");
        engine.init();

        Self {
            audio_engine: engine,

            playing: false,

            files_panel: FilesPanel::default(),
            files_shown: true,

            mixer_panel: MixerPanel::default(),
            mixer_shown: true,

            arranger_panel: ArrangerPanel::default(),
            arranger_shown: true,

            piano_panel: PianoPanel,
            piano_shown: false,

            settings_panel: SettingsPanel,
            settings_shown: false,
        }
    }
}
