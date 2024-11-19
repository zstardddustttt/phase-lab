use eframe::egui;

use super::DockedPanel;

pub struct FilesPanel;

impl Default for FilesPanel {
    fn default() -> Self {
        Self
    }
}

impl DockedPanel for FilesPanel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("files");
    }

    fn name(&self) -> &'static str {
        "files"
    }
}
