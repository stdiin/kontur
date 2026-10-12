use crate::{
    map::{
        self, Category, MapData,
        object::{MapObject, MapObjectKind},
        viewer::MapViewer,
    },
    screen::Screen,
};
use eframe::egui::{self, Align2, Area, MenuBar, Panel, Ui, Vec2};
use egui_file_dialog::FileDialog;
use egui_ltreeview::{NodeBuilder, TreeView};
use slotmap::{SlotMap, new_key_type};
use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
};

new_key_type! {struct NodeId;}

enum ExplorerAction {
    Rename((NodeId, String)),
    Create((Option<NodeId>, NodeKind)),
    Remove(NodeId),
}

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

struct EditorData {
    map_data: MapData,
    viewer: MapViewer,
    map_save_path: Option<PathBuf>,
    storage: EditorStorage,
    renaming: Option<(NodeId, String)>,
}

impl EditorData {
    fn save(&self, path: &Path) {
        let map_data = MapData {
            name: self.map_data.name.clone(),
            image: self.map_data.image.clone(),
            categories: self.storage.categories(),
        };

        map::save(map_data, path).unwrap();
    }
}

enum EditorMode {
    Selecting,
    Editing(EditorData),
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

                    next_mode = Some(EditorMode::Editing(EditorData {
                        viewer: MapViewer::new(ui, image),
                        map_save_path: Some(path.to_path_buf()),
                        storage: EditorStorage::new(&map_data),
                        map_data,
                        renaming: None,
                    }))
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

                        if ui.button("Create").clicked()
                            && let Some(bytes) = image_bytes
                            && !name.is_empty()
                        {
                            let map_data = MapData {
                                name: name.to_owned(),
                                image: bytes.to_owned(),
                                ..Default::default()
                            };

                            next_mode = Some(EditorMode::Editing(EditorData {
                                viewer: MapViewer::new(ui, bytes),
                                map_save_path: None,
                                storage: EditorStorage::new(&map_data),
                                map_data,
                                renaming: None,
                            }))
                        }
                    });

                ui.disable();
            }

            EditorMode::Editing(data) => {
                if let Some(path) = self.file_dialog.take_picked() {
                    data.save(&path);
                }

                Panel::top("topbar").show(ui, |ui| {
                    MenuBar::new().ui(ui, |ui| {
                        ui.menu_button("File", |ui| {
                            let mut saving_as_new = false;

                            if ui.button("Save").clicked() {
                                if let Some(path) = &data.map_save_path {
                                    data.save(path);
                                } else {
                                    saving_as_new = true;
                                }
                            };

                            if ui.button("Save As").clicked() {
                                saving_as_new = true;
                            };

                            if saving_as_new {
                                self.file_dialog.config_mut().default_file_name =
                                    data.map_data.name.replace(" ", "_");
                                self.file_dialog.save_file();
                            }
                        });

                        ui.separator();
                        ui.label("Map")
                    });
                });

                let actions = RefCell::new(Vec::new());
                let mut stop_renaming = false;
                let mut start_renaming = None;

                Panel::right("explorer").show(ui, |ui| {
                    TreeView::new("explorer".into())
                        .fallback_context_menu(|ui, _| {
                            if ui.button("New category").clicked() {
                                actions
                                    .borrow_mut()
                                    .push(ExplorerAction::Create((None, NodeKind::Category)));
                            }
                        })
                        .show(ui, |builder| {
                            for id in data.storage.roots.iter().copied() {
                                let Some(category) = data.storage.nodes.get(id) else {
                                    continue;
                                };

                                let mut category_node = NodeBuilder::dir(id)
                                    .label(&category.name)
                                    .context_menu(|ui| {
                                        if ui.button("Rename").clicked() {
                                            start_renaming = Some((id, category.name.clone()));
                                        }

                                        if ui.button("Delete").clicked() {
                                            actions.borrow_mut().push(ExplorerAction::Remove(id));
                                        }

                                        ui.menu_button("Add object", |ui| {
                                            if ui.button("Region").clicked() {
                                                actions.borrow_mut().push(ExplorerAction::Create(
                                                    (
                                                        Some(id),
                                                        NodeKind::Object(MapObjectKind::Region),
                                                    ),
                                                ));
                                            }

                                            if ui.button("Peak").clicked() {
                                                actions.borrow_mut().push(ExplorerAction::Create(
                                                    (
                                                        Some(id),
                                                        NodeKind::Object(MapObjectKind::Peak),
                                                    ),
                                                ));
                                            }

                                            if ui.button("River").clicked() {
                                                actions.borrow_mut().push(ExplorerAction::Create(
                                                    (
                                                        Some(id),
                                                        NodeKind::Object(MapObjectKind::River),
                                                    ),
                                                ));
                                            }
                                        });
                                    });

                                if let Some((renaming_id, text_buffer)) = &mut data.renaming
                                    && *renaming_id == id
                                {
                                    category_node = category_node.label_ui(|ui| {
                                        handle_rename_ui(
                                            ui,
                                            id,
                                            &category.name,
                                            text_buffer,
                                            &actions,
                                            &mut stop_renaming,
                                        );
                                    });
                                }

                                builder.node(category_node);

                                for id in category.children.iter().copied() {
                                    if let Some(object) = data.storage.nodes.get(id) {
                                        let mut object_node = NodeBuilder::leaf(id)
                                            .label(&object.name)
                                            .context_menu(|ui| {
                                                if ui.button("Rename").clicked() {
                                                    start_renaming =
                                                        Some((id, object.name.clone()));
                                                }

                                                if ui.button("Delete").clicked() {
                                                    actions
                                                        .borrow_mut()
                                                        .push(ExplorerAction::Remove(id));
                                                }
                                            });

                                        if let Some((renaming_id, text_buffer)) = &mut data.renaming
                                            && *renaming_id == id
                                        {
                                            object_node = object_node.label_ui(|ui| {
                                                handle_rename_ui(
                                                    ui,
                                                    id,
                                                    &object.name,
                                                    text_buffer,
                                                    &actions,
                                                    &mut stop_renaming,
                                                );
                                            });
                                        }

                                        builder.node(object_node);
                                    }
                                }

                                builder.close_dir();
                            }
                        })
                });

                if stop_renaming {
                    data.renaming = None;
                }

                if let Some(rename_state) = start_renaming {
                    data.renaming = Some(rename_state);
                }

                for action in actions.into_inner() {
                    match action {
                        ExplorerAction::Create((parent_id, kind)) => {
                            if let Some(id) = data.storage.add_node(parent_id, kind, String::new())
                            {
                                data.renaming = Some((id, String::new()))
                            }
                        }

                        ExplorerAction::Rename((id, buffer)) => {
                            if let Some(node) = data.storage.nodes.get_mut(id) {
                                node.name = buffer
                            }
                        }

                        ExplorerAction::Remove(id) => {
                            data.storage.remove_node(id);
                        }
                    }
                }

                data.viewer.render(ui);
            }
        }

        if let Some(mode) = next_mode {
            self.mode = mode
        }

        super::Transition::None
    }
}

fn handle_rename_ui(
    ui: &mut Ui,
    id: NodeId,
    original_name: &str,
    text_buffer: &mut String,
    actions: &RefCell<Vec<ExplorerAction>>,
    stop_renaming: &mut bool,
) {
    let response = ui.text_edit_singleline(text_buffer);

    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
        if original_name.is_empty() {
            actions.borrow_mut().push(ExplorerAction::Remove(id));
        }

        *stop_renaming = true;
    } else if response.lost_focus() {
        let new_name = text_buffer.trim().to_string();

        if !new_name.is_empty() {
            actions
                .borrow_mut()
                .push(ExplorerAction::Rename((id, new_name)));
        }

        *stop_renaming = true;
    } else {
        response.request_focus();
    }
}
