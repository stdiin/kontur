use super::Screen;
use crate::{screens::Transition, viewer::MapViewer};
use egui_macroquad::{egui, macroquad::prelude::*};

pub struct Quiz {
    viewer: MapViewer,
}

impl Default for Quiz {
    fn default() -> Self {
        Self {
            viewer: MapViewer::new(Texture2D::from_file_with_format(
                include_bytes!("../../assets/map.png"),
                Some(ImageFormat::Png),
            )),
        }
    }
}

impl Screen for Quiz {
    fn ui(&mut self, ctx: &egui::Context) -> Transition {
        egui::CentralPanel::default()
            .frame(egui::Frame::new().multiply_with_opacity(1.0))
            .show(ctx, |ui| {
                let rect = ui.max_rect();

                self.viewer.set_viewport(Rect::new(
                    rect.left(),
                    rect.top(),
                    rect.right(),
                    rect.bottom(),
                ));
            });

        Transition::None
    }

    fn update(&mut self) {
        self.viewer.update();
    }

    fn render(&self) {
        self.viewer.render();
    }
}
