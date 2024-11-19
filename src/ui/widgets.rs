use eframe::egui::{self, ImageSource, Rect};

use crate::panels::FloatingPanel;

pub trait UIExtensions<'a> {
    fn toggle_image(
        &mut self,
        value: &mut bool,
        source: impl Into<ImageSource<'a>>,
    ) -> egui::Response;

    fn toggle_images(
        &mut self,
        value: &mut bool,
        first_source: impl Into<ImageSource<'a>>,
        second_source: impl Into<ImageSource<'a>>,
    ) -> egui::Response;

    fn toggle_floating_panel(
        &mut self,
        panel: &mut impl FloatingPanel,
        shown: &mut bool,
        source: impl Into<ImageSource<'a>>,
    ) -> egui::Response;
}

impl<'a> UIExtensions<'a> for egui::Ui {
    fn toggle_image(
        &mut self,
        value: &mut bool,
        source: impl Into<ImageSource<'a>>,
    ) -> egui::Response {
        let res = self.toggle_value(value, "    ");
        let img = if *value {
            egui::Image::new(source).tint(self.style().visuals.strong_text_color())
        } else {
            egui::Image::new(source).tint(self.style().visuals.text_color())
        };

        self.put(
            Rect::from_center_size(res.rect.center(), res.rect.size() - egui::vec2(2.0, 2.0)),
            img,
        );

        res
    }

    fn toggle_images(
        &mut self,
        value: &mut bool,
        first_source: impl Into<ImageSource<'a>>,
        second_source: impl Into<ImageSource<'a>>,
    ) -> egui::Response {
        let res = self.toggle_value(value, "    ");
        let img = if *value {
            egui::Image::new(first_source).tint(self.style().visuals.strong_text_color())
        } else {
            egui::Image::new(second_source).tint(self.style().visuals.text_color())
        };

        self.put(
            Rect::from_center_size(res.rect.center(), res.rect.size() - egui::vec2(2.0, 2.0)),
            img,
        );

        res
    }

    fn toggle_floating_panel(
        &mut self,
        panel: &mut impl FloatingPanel,
        shown: &mut bool,
        source: impl Into<ImageSource<'a>>,
    ) -> egui::Response {
        let res = self.toggle_image(shown, source);
        if *shown {
            panel.show(self.ctx(), shown);
        }

        res.on_hover_text(panel.name())
    }
}
