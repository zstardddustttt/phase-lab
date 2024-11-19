use eframe::egui::{self, style::HandleShape, DragValue, RichText};
use widgets::UIExtensions;

use crate::{audio::PLAYBACK_DATA, image_asset, panels::DockedPanel, utils::NumExtensions, App};

pub mod widgets;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut playback = PLAYBACK_DATA.lock().unwrap();

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.add_space(3.0);

            ui.horizontal(|ui| {
                ui.label("panels");
                ui.toggle_floating_panel(
                    &mut self.piano_panel,
                    &mut self.piano_shown,
                    image_asset!("piano.svg"),
                );

                ui.toggle_floating_panel(
                    &mut self.settings_panel,
                    &mut self.settings_shown,
                    image_asset!("settings.svg"),
                );

                ui.toggle_image(&mut self.files_shown, image_asset!("folders.svg"));

                ui.toggle_image(&mut self.arranger_shown, image_asset!("pencil.svg"));

                ui.toggle_image(
                    &mut self.mixer_shown,
                    image_asset!("sliders-horizontal.svg"),
                );

                ui.separator();

                ui.label("tempo");
                ui.add(DragValue::new(&mut playback.tempo).range(1..=1000));
                let res = ui.toggle_image(&mut playback.metronome, image_asset!("metronome.svg"));
                res.on_hover_text("metronome");

                ui.separator();

                ui.label("volume");
                let res = ui.add(
                    egui::Slider::new(&mut playback.volume, 0.0..=1.0)
                        .show_value(false)
                        .handle_shape(HandleShape::Rect { aspect_ratio: 1.2 }),
                );
                res.on_hover_text("volume").context_menu(|ui| {
                    let res = ui.button("reset");
                    if res.clicked() {
                        playback.volume = 0.8;
                    }
                });

                let res = ui.add(egui::Button::image(
                    egui::Image::new(image_asset!("player-skip-back.svg"))
                        .tint(ui.style().visuals.text_color()),
                ));

                if res.clicked() {
                    playback.reset_elapsed();
                }

                let res = ui.toggle_images(
                    &mut self.playing,
                    image_asset!("player-pause.svg"),
                    image_asset!("player-play.svg"),
                );

                if res.clicked() {
                    if self.playing {
                        self.audio_engine.play();
                    } else {
                        self.audio_engine.pause();
                    }
                }

                let res = ui.add(egui::Button::image(
                    egui::Image::new(image_asset!("player-stop.svg"))
                        .tint(ui.style().visuals.text_color()),
                ));

                if res.clicked() {
                    self.playing = false;
                    playback.reset_elapsed();
                    self.audio_engine.pause();
                }

                let duration_sec = playback.duration.to_seconds(playback.tempo);
                let progress = playback.elapsed / duration_sec;

                ui.add(
                    egui::ProgressBar::new(progress as f32)
                        .text(RichText::new(playback.elapsed.seconds_to_msm()).monospace())
                        .desired_width(ui.available_width() - 100.0),
                );

                ui.label(RichText::new(duration_sec.seconds_to_msm()).monospace());
            });

            ui.add_space(1.0);
        });

        if self.mixer_shown {
            egui::TopBottomPanel::bottom(self.mixer_panel.name())
                .resizable(true)
                .min_height(50.0)
                .default_height(300.0)
                .show(ctx, |ui| {
                    ui.add_space(6.0);
                    self.mixer_panel.ui(ui);
                    ui.allocate_space(ui.available_size());
                });
        }

        if self.files_shown {
            egui::SidePanel::left(self.files_panel.name())
                .min_width(50.0)
                .default_width(300.0)
                .show(ctx, |ui| {
                    ui.add_space(6.0);
                    self.files_panel.ui(ui);
                    ui.allocate_space(ui.available_size());
                });
        }

        if self.arranger_shown {
            egui::CentralPanel::default().show(ctx, |ui| {
                self.arranger_panel.ui(ui);
                ui.allocate_space(ui.available_size());
            });
        }

        if self.playing {
            ctx.request_repaint();
        }
    }
}
