use eframe::egui;

use super::FloatingPanel;

pub struct SettingsPanel;

impl Default for SettingsPanel {
    fn default() -> Self {
        Self
    }
}

impl FloatingPanel for SettingsPanel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("settings");
    }

    fn name(&self) -> &'static str {
        "settings"
    }
}
