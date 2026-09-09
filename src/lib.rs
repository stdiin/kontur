use eframe::egui;

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

#[derive(Default)]
pub struct App {}

impl App {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("status bar").show(ui, |ui| {
            ui.label("test")
        });
    }
}
