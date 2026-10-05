use crate::{
    map::{self, MapData, viewer::MapViewer},
    screen::Screen,
};
use eframe::egui::{Align2, Area, MenuBar, Panel, Ui, Vec2};
use egui_file_dialog::FileDialog;
use std::{fs, path::PathBuf};

enum EditorMode {
    Selecting,
    Editing {
        map_data: MapData,
        viewer: MapViewer,
        map_save_path: Option<PathBuf>,
    },
    Creating {
        name: String,
        image_bytes: Option<Vec<u8>>,
        picked_path: Option<PathBuf>,
    },
}

pub struct Editor {
    file_dialog: FileDialog,
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
            mode: EditorMode::Selecting,
        }
    }
}

impl Screen for Editor {
    fn ui(&mut self, ui: &mut Ui) -> super::Transition {
        self.file_dialog.update(ui);

        let mut next_mode = None;

        match &mut self.mode {
            EditorMode::Selecting => {
                if let Some(path) = self.file_dialog.take_picked() {
                    let map_data = map::load(&path).unwrap();
                    let image = map_data.image.clone();

                    next_mode = Some(EditorMode::Editing {
                        map_data,
                        viewer: MapViewer::new(ui, image),
                        map_save_path: Some(path.to_path_buf()),
                    })
                }

                Area::new("map selection".into())
                    .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                    .show(ui, |ui| {
                        if ui.button("Import").clicked() {
                            self.file_dialog.config_mut().default_file_filter = Some("map".into());
                            self.file_dialog.pick_file();
                        }

                        if ui.button("New").clicked() {
                            self.mode = EditorMode::Creating {
                                name: "".to_string(),
                                image_bytes: None,
                                picked_path: None,
                            }
                        }
                    });

                ui.disable();
            }

            EditorMode::Creating {
                name,
                image_bytes,
                picked_path,
            } => {
                if let Some(path) = self.file_dialog.take_picked() {
                    let image = fs::read(&path).unwrap();
                    *image_bytes = Some(image);
                    *picked_path = Some(path)
                }

                Area::new("map creation".into())
                    .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                    .show(ui, |ui| {
                        ui.label("Map name");
                        ui.text_edit_singleline(name);
                        ui.label("Image");
                        ui.horizontal(|ui| {
                            if ui.button("Pick").clicked() {
                                self.file_dialog.config_mut().default_file_filter =
                                    Some("image".into());
                                self.file_dialog.pick_file();
                            };

                            if let Some(path) = picked_path {
                                ui.label(path.display().to_string());
                            }
                        });

                        if ui.button("Create").clicked() {
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
                                    map_save_path: None,
                                });
                            }
                        }
                    });

                ui.disable();
            }

            EditorMode::Editing {
                map_save_path,
                map_data,
                viewer,
            } => {
                if let Some(path) = self.file_dialog.take_picked() {
                    map::save(map_data.clone(), path).unwrap();
                }

                Panel::top("topbar").show(ui, |ui| {
                    MenuBar::new().ui(ui, |ui| {
                        ui.menu_button("File", |ui| {
                            let mut saving_as_new = false;

                            if ui.button("Save").clicked() {
                                if let Some(path) = map_save_path {
                                    map::save(map_data.clone(), path).unwrap();
                                } else {
                                    saving_as_new = true;
                                }
                            };

                            if ui.button("Save As").clicked() {
                                saving_as_new = true;
                            };

                            if saving_as_new {
                                self.file_dialog.config_mut().default_file_name = map_data.name.replace(" ", "_");
                                self.file_dialog.save_file();
                            }
                        });

                        ui.separator();
                        ui.label("Map")
                    });
                });

                viewer.render(ui);
            }
        }

        if let Some(mode) = next_mode {
            self.mode = mode
        }

        super::Transition::None
    }
}
