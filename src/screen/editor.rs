use std::path::PathBuf;
use eframe::egui::{Align2, Area, MenuBar, Panel, Ui, Vec2};
use egui_file_dialog::FileDialog;
use crate::{map::MapData, screen::Screen};

#[allow(dead_code)]
enum FileDialogAction {
    OpenMap,
    OpenImage,
    SaveMap,
}

pub struct Editor {
    file_dialog: FileDialog,
    picked_path: Option<PathBuf>,
    map_data: Option<MapData>
}

impl Editor {
    pub fn new() -> Self {
        let file_dialog = FileDialog::new()
            .add_file_filter_extensions("map", vec!["map"])
            .default_file_filter("map")
            .show_all_files_filter(false)
            .add_save_extension("map", "map")
            .default_save_extension("map")
            .as_modal(true);

        Self {
            file_dialog,
            picked_path: None,
            map_data: None
        }
    }

    fn topbar(&mut self, ui: &mut Ui) {
        Panel::top("topbar").show(ui, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    _ = ui.button("Save");

                    if ui.button("Save As").clicked() {
                        self.file_dialog.set_user_data(FileDialogAction::SaveMap);
                        self.file_dialog.save_file();
                    };
                });

                ui.separator();
                ui.label("Map")
            });
        });
    }

    fn process_file_dialog(&mut self, ui: &mut Ui) {
        self.file_dialog.update(ui);

        if let Some(path) = self.file_dialog.take_picked() {
            self.picked_path = Some(path.to_path_buf());

            println!("picked: {}", path.display());

            match self.file_dialog.user_data() {
                Some(FileDialogAction::OpenMap) => println!("open map"),
                Some(FileDialogAction::OpenImage) => println!("open image"),
                Some(FileDialogAction::SaveMap) => println!("save map"),
                None => {}
            };
        }
    }
}

impl Screen for Editor {
    fn ui(&mut self, ui: &mut Ui) -> super::Transition {
        self.process_file_dialog(ui);

        if self.map_data.is_none() {
            Area::new("map selection".into())
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ui, |ui| {
                    if ui.button("Import").clicked() {
                        self.file_dialog.set_user_data(FileDialogAction::OpenMap);
                        self.file_dialog.pick_file();
                    }

                    if ui.button("New").clicked() {
                        todo!()
                    }
            });

            ui.disable();
        }

        self.topbar(ui);

        super::Transition::None
    }
}
