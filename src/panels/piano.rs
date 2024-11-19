use eframe::egui;

use super::FloatingPanel;

pub struct PianoPanel;

impl Default for PianoPanel {
    fn default() -> Self {
        Self
    }
}

impl FloatingPanel for PianoPanel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("piano");
    }

    fn name(&self) -> &'static str {
        "piano"
    }
}
