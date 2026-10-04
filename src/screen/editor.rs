use crate::{
    map::{self, MapData, viewer::MapViewer},
    screen::Screen,
};
use eframe::egui::{Align2, Area, MenuBar, Panel, Ui, Vec2};
use egui_file_dialog::FileDialog;
use std::{fs, path::PathBuf};

// #[allow(dead_code)]
// enum FileDialogAction {
//     OpenMap,
//     OpenImage,
//     SaveMap,
// }

enum EditorMode {
    Selecting,
    Editing {
        map_data: MapData,
        viewer: MapViewer,
    },
    Creating {
        name: String,
        image_bytes: Option<Vec<u8>>,
    },
}

pub struct Editor {
    file_dialog: FileDialog,
    picked_path: Option<PathBuf>,
    mode: EditorMode,
}

impl Editor {
    pub fn new() -> Self {
        let file_dialog = FileDialog::new()
            .add_file_filter_extensions("map", vec!["map"])
            .add_file_filter_extensions("image", vec!["png", "jpg", "jpeg"])
            .show_all_files_filter(false)
            .add_save_extension("map", "map")
            .default_save_extension("map")
            .as_modal(true);

        Self {
            file_dialog,
            picked_path: None,
            mode: EditorMode::Selecting,
        }
    }

    fn topbar(&mut self, ui: &mut Ui) {
        Panel::top("topbar").show(ui, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    // TODO
                    _ = ui.button("Save");

                    if ui.button("Save As").clicked() {
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

            let mut next_mode = None;

            match &mut self.mode {
                EditorMode::Selecting => {
                    let map_data = map::load(path).unwrap();
                    let image = map_data.image.clone();
                    next_mode = Some(EditorMode::Editing {
                        map_data,
                        viewer: MapViewer::new(ui, image),
                    })
                }
                EditorMode::Creating {
                    name: _,
                    image_bytes,
                } => {
                    let bytes = fs::read(path).unwrap();
                    *image_bytes = Some(bytes)
                }
                EditorMode::Editing {
                    map_data: _,
                    viewer: _,
                } => {
                    // TODO: add saving
                }
            }

            if let Some(mode) = next_mode {
                self.mode = mode
            }
        }
    }
}

impl Screen for Editor {
    fn ui(&mut self, ui: &mut Ui) -> super::Transition {
        self.process_file_dialog(ui);

        let mut next_mode = None;

        match &mut self.mode {
            EditorMode::Selecting => {
                Area::new("map selection".into())
                    .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                    .show(ui, |ui| {
                        if ui.button("Import").clicked() {
                            self.file_dialog.config_mut().default_file_filter = Some("map".into());
                            self.file_dialog.pick_file();
                        }

                        if ui.button("New").clicked() {
                            self.file_dialog.config_mut().default_file_filter =
                                Some("image".into());

                            self.mode = EditorMode::Creating {
                                name: "".to_string(),
                                image_bytes: None,
                            }
                        }
                    });

                ui.disable();
            }

            EditorMode::Creating { name, image_bytes } => {
                Area::new("map creation".into())
                    .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                    .show(ui, |ui| {
                        ui.label("Map name");
                        ui.text_edit_singleline(name);
                        ui.label("Image");
                        ui.horizontal(|ui| {
                            if ui.button("Pick").clicked() {
                                self.file_dialog.pick_file();
                            };

                            if let Some(path) = &self.picked_path {
                                ui.label(path.display().to_string());
                            }
                        });

                        if ui.button("create").clicked() {
                            if let Some(bytes) = image_bytes
                                && !name.is_empty()
                            {
                                next_mode = Some(EditorMode::Editing {
                                    map_data: MapData {
                                        name: name.to_owned(),
                                        image: bytes.to_owned(),
                                        ..Default::default()
                                    },
                                    viewer: MapViewer::new(ui, bytes),
                                });
                            }
                        }
                    });
            }

            EditorMode::Editing {
                map_data: _,
                viewer,
            } => {
                viewer.render(ui);
            }
        }

        self.topbar(ui);

        if let Some(mode) = next_mode {
            self.mode = mode
        }

        super::Transition::None
    }
}
