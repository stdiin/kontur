use crate::screen::{Screen, ScreenType, Transition};
use eframe::egui;

mod screen;
mod map;

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };

    eframe::run_native(
        "Kontur",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
    .unwrap()
}

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
        self.screen.render();
    }
}
