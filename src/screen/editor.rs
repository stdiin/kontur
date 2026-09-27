use crate::{map::MapData, screen::Screen};
use eframe::egui::{Align2, Area, MenuBar, Panel, Ui, Vec2};
use egui_file_dialog::FileDialog;
use std::path::PathBuf;

#[allow(dead_code)]
enum FileDialogAction {
    OpenMap,
    OpenImage,
    SaveMap,
}

enum EditorState {
    Editing,
    Creating,
    Selecting,
}

pub struct Editor {
    file_dialog: FileDialog,
    picked_path: Option<PathBuf>,
    map_data: Option<MapData>,
    state: EditorState,
}

impl Editor {
    pub fn new() -> Self {
        let file_dialog = FileDialog::new()
            .add_file_filter_extensions("map", vec!["map"])
            .add_file_filter_extensions("image", vec!["png", "jpg", "jpeg"])
            .show_all_files_filter(false)
            // .default_file_filter("map")
            .add_save_extension("map", "map")
            .default_save_extension("map")
            .as_modal(true);

        Self {
            file_dialog,
            picked_path: None,
            map_data: None,
            state: EditorState::Selecting,
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

            match self.file_dialog.user_data() {
                Some(FileDialogAction::OpenMap) => {}
                Some(FileDialogAction::OpenImage) => {}
                Some(FileDialogAction::SaveMap) => println!("save map"),
                None => {}
            };
        }
    }
}

impl Screen for Editor {
    fn ui(&mut self, ui: &mut Ui) -> super::Transition {
        self.process_file_dialog(ui);

        match self.state {
            EditorState::Selecting => {
                Area::new("map selection".into())
                    .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                    .show(ui, |ui| {
                        if ui.button("Import").clicked() {
                            self.file_dialog.set_user_data(FileDialogAction::OpenMap);
                            self.file_dialog.pick_file();
                        }

                        if ui.button("New").clicked() {
                            self.map_data = Some(MapData::default());
                            self.state = EditorState::Creating
                        }
                    });

                ui.disable();
            }

            EditorState::Creating => {
                let map_data = self.map_data.as_mut().unwrap();

                Area::new("map creation".into())
                    .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                    .show(ui, |ui| {
                        ui.label("Map name");
                        ui.text_edit_singleline(&mut map_data.name);
                        ui.label("Image");
                        ui.horizontal(|ui| {
                            if ui.button("Pick").clicked() {
                                self.file_dialog.set_user_data(FileDialogAction::OpenImage);
                                self.file_dialog.pick_file();
                            };

                            if let Some(path) = &self.picked_path {
                                ui.label(path.display().to_string());
                            }
                        });
                    });
            }

            EditorState::Editing => {}
        }

        self.topbar(ui);

        super::Transition::None
    }
}
