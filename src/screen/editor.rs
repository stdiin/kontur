use crate::{
    map::{
        self, Category, MapData,
        object::{MapObject, MapObjectKind},
        viewer::MapViewer,
    },
    screen::Screen,
};
use eframe::egui::{Align2, Area, MenuBar, Panel, Ui, Vec2};
use egui_file_dialog::FileDialog;
use egui_ltreeview::{NodeBuilder, TreeView};
use slotmap::{SlotMap, new_key_type};
use std::{fs, path::PathBuf};

new_key_type! {struct NodeId;}

enum NodeKind {
    Category,
    Object(MapObjectKind),
}

struct Node {
    parent_id: Option<NodeId>,
    children: Vec<NodeId>,
    name: String,
    kind: NodeKind,
}

struct EditorStorage {
    nodes: SlotMap<NodeId, Node>,
    roots: Vec<NodeId>,
}

impl EditorStorage {
    fn new(map_data: &MapData) -> Self {
        let mut storage = Self {
            nodes: SlotMap::with_key(),
            roots: Vec::new(),
        };

        for category in &map_data.categories {
            if let Some(id) = storage.add_node(None, NodeKind::Category, category.name.to_string())
            {
                for object in &category.objects {
                    storage.add_node(
                        Some(id),
                        NodeKind::Object(object.kind.clone()),
                        object.name.to_string(),
                    );
                }
            }
        }

        storage
    }

    fn categories(&self) -> Vec<Category> {
        self.roots
            .iter()
            .filter_map(|id| {
                let node = self.nodes.get(*id)?;

                Some(Category {
                    name: node.name.clone(),
                    objects: node
                        .children
                        .iter()
                        .filter_map(|id| {
                            let child = self.nodes.get(*id)?;
                            if let NodeKind::Object(kind) = &child.kind {
                                Some(MapObject {
                                    name: child.name.clone(),
                                    kind: kind.clone(),
                                })
                            } else {
                                None
                            }
                        })
                        .collect(),
                })
            })
            .collect()
    }

    fn add_node(
        &mut self,
        parent_id: Option<NodeId>,
        kind: NodeKind,
        name: String,
    ) -> Option<NodeId> {
        if let Some(pid) = parent_id
            && !self.nodes.contains_key(pid)
        {
            return None;
        }

        let id = self.nodes.insert(Node {
            name,
            kind,
            parent_id,
            children: Vec::new(),
        });

        match parent_id {
            Some(pid) => {
                if let Some(parent) = self.nodes.get_mut(pid) {
                    parent.children.push(id);
                }
            }
            None => self.roots.push(id),
        }

        Some(id)
    }

    fn remove_node(&mut self, id: NodeId) {
        if let Some(node) = self.nodes.remove(id) {
            match node.parent_id {
                Some(pid) => {
                    if let Some(parent) = self.nodes.get_mut(pid) {
                        parent.children.retain(|v| *v != id);
                    }
                }
                None => self.roots.retain(|v| *v != id),
            }

            let mut to_remove = node.children;

            while let Some(current_id) = to_remove.pop() {
                if let Some(node) = self.nodes.remove(current_id) {
                    to_remove.extend(node.children);
                }
            }
        }
    }
}

enum EditorMode {
    Selecting,
    Editing {
        map_data: MapData,
        viewer: MapViewer,
        map_save_path: Option<PathBuf>,
        storage: EditorStorage,
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
                        viewer: MapViewer::new(ui, image),
                        map_save_path: Some(path.to_path_buf()),
                        storage: EditorStorage::new(&map_data),
                        map_data,
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
                                let map_data = MapData {
                                    name: name.to_owned(),
                                    image: bytes.to_owned(),
                                    ..Default::default()
                                };

                                next_mode = Some(EditorMode::Editing {
                                    viewer: MapViewer::new(ui, bytes),
                                    map_save_path: None,
                                    storage: EditorStorage::new(&map_data),
                                    map_data,
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
                storage,
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
                                self.file_dialog.config_mut().default_file_name =
                                    map_data.name.replace(" ", "_");
                                self.file_dialog.save_file();
                            }
                        });

                        ui.separator();
                        ui.label("Map")
                    });
                });

                Panel::right("explorer").show(ui, |ui| {
                    TreeView::new("explorer".into())
                        .fallback_context_menu(|ui, _| {
                            if ui.button("New category").clicked() {
                                // NEW CATEGORY
                            }
                        })
                        .show(ui, |builder| {
                            for (index, category) in map_data.categories.iter().enumerate() {
                                builder.node(
                                    NodeBuilder::dir(index)
                                        .label_ui(|ui| {
                                            // RENAMING
                                            //                                             if let Some(renaming) = renaming
                                            //                                                 && renaming.index == index
                                            //                                             {
                                            //                                                 let response =
                                            //                                                     ui.text_edit_singleline(&mut renaming.buffer);
                                            //
                                            //                                                 if renaming.is_new {
                                            //                                                     response.request_focus();
                                            //                                                     renaming.is_new = false;
                                            //                                                 }
                                            //
                                            //                                                 if response.lost_focus() {
                                            //                                                     stop_renaming = true;
                                            //                                                 };
                                            //                                             } else {
                                            //                                                 ui.label(&category.name);
                                            //                                             }
                                        })
                                        .context_menu(|ui| {
                                            if ui.button("Rename").clicked() {
                                                // START RENAMING
                                            }
                                        }),
                                );

                                for (index, object) in category.objects.iter().enumerate() {
                                    // builder.leaf(index, object.name);
                                }

                                builder.close_dir();
                            }
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
