use kontur::App;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Kontur", native_options, Box::new(|cc| Ok(Box::new(App::new(cc)))))
}
