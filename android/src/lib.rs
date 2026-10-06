#[unsafe(no_mangle)]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };

    eframe::run_native(
        "Kontur",
        options,
        Box::new(|cc| Ok(Box::new(kontur::App::new(cc)))),
    )
    .unwrap()
}
