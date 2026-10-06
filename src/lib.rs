use crate::screen::{Screen, ScreenType, Transition};
use eframe::egui;

mod screen;
mod map;

pub struct App {
    screen: Box<dyn Screen>
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        Self {
            screen: ScreenType::Menu.build(&cc.egui_ctx)
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        match self.screen.ui(ui) {
            Transition::None => {}
            Transition::Quit => ui.send_viewport_cmd(egui::ViewportCommand::Close),
            Transition::Switch(next_screen) => self.screen = next_screen.build(ui.ctx()),
        }

        self.screen.update();
    }
}
