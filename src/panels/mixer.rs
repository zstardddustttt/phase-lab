use eframe::egui;

use super::DockedPanel;

pub struct MixerPanel;

impl Default for MixerPanel {
    fn default() -> Self {
        Self
    }
}

impl DockedPanel for MixerPanel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("mixer");
    }

    fn name(&self) -> &'static str {
        "mixer"
    }
}
