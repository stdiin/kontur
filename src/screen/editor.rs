use std::path::PathBuf;

use egui_file_dialog::FileDialog;
use crate::screen::Screen;

pub struct Editor {
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>
}

impl Editor {
    pub fn new() -> Self {
        Self {
            file_dialog: FileDialog::new(),
            picked_file: None
        }
    }
}

impl Screen for Editor {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) -> super::Transition {
        if ui.button("Pick a file").clicked() {
            self.file_dialog.pick_file();
        }

        self.file_dialog.update(ui);

        if let Some(path) = self.file_dialog.take_picked() {
            self.picked_file = Some(path.to_path_buf());
            println!("picked: {}", path.display());
        }

        super::Transition::None
    }
}
