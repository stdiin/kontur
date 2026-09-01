use super::Screen;
use crate::screens::{ScreenType, Transition};
use egui_macroquad::egui;

#[derive(Default)]
pub struct Menu;

impl Screen for Menu {
    fn ui(&mut self, ctx: &egui::Context) -> Transition {
        let mut transition = Transition::None;

        egui::Area::new("menu".into())
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("Kontur").size(75.0));

                    ui.add_space(100.0);

                    if Self::button(ui, "Play").clicked() {
                        transition = Transition::Switch(ScreenType::Quiz)
                    };

                    ui.add_space(20.0);

                    #[cfg(not(target_os = "android"))]
                    if Self::button(ui, "Editor").clicked() {
                        transition = Transition::Switch(ScreenType::Editor)
                    };

                    ui.add_space(20.0);

                    if Self::button(ui, "Quit").clicked() {
                        transition = Transition::Quit
                    };
                });
            });

        transition
    }
}

impl Menu {
    fn button(ui: &mut egui::Ui, content: &str) -> egui::Response {
        let width = ui.available_width().min(400.0);
        let height = (width * 0.22).clamp(55.0, 90.0);

        ui.add_sized(
            [width, height],
            egui::Button::new(
                egui::RichText::new(content)
                    .size((height * 0.35).clamp(18.0, 28.0)),
            )
            .corner_radius(height * 0.2),
        )
    }
}
