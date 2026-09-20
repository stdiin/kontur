use eframe::egui;

use crate::{map::viewer::MapViewer, screen::Screen};

pub struct Quiz {
    viewer: MapViewer,
}

impl Quiz {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            viewer: MapViewer::new(ctx, include_bytes!("../../assets/map.png")),
        }
    }
}

impl Screen for Quiz {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) -> super::Transition {
        self.viewer.render(ui);

        super::Transition::None
    }
}
