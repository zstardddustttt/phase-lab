use eframe::egui;

pub mod arranger;
pub mod files;
pub mod mixer;
pub mod piano;
pub mod settings;

pub trait FloatingPanel {
    fn show(&mut self, ctx: &egui::Context, shown: &mut bool) {
        let window = egui::Window::new(self.name())
            .default_width(600.0)
            .default_height(400.0)
            .vscroll(false)
            .open(shown);

        window.show(ctx, |ui| {
            self.ui(ui);
            ui.allocate_space(ui.available_size());
        });
    }

    fn ui(&mut self, ui: &mut egui::Ui);
    fn name(&self) -> &'static str;
}

pub trait DockedPanel {
    fn ui(&mut self, ui: &mut egui::Ui);
    fn name(&self) -> &'static str;
}
