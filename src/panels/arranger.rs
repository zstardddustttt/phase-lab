use eframe::egui;

use super::DockedPanel;

#[derive(Debug)]
pub struct ArrangerPanel;

impl Default for ArrangerPanel {
    fn default() -> Self {
        Self
    }
}

impl DockedPanel for ArrangerPanel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("arranger");
    }

    fn name(&self) -> &'static str {
        "arranger"
    }
}
