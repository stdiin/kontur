use crate::{
    map::{self, data::*, viewer::MapViewer}, screen::{Screen, Transition}
};

use macroquad::prelude::*;

use std::{
    fs,
    path::{PathBuf},
    thread::JoinHandle,
};

#[derive(Default)]
struct BackgroundTask<T> {
    handle: Option<JoinHandle<T>>,
}

impl<T> BackgroundTask<T> {
    fn take(&mut self) -> Option<T> {
        self.handle
            .as_ref()
            .is_some_and(JoinHandle::is_finished)
            .then(|| self.handle.take().unwrap())
            .and_then(|v| v.join().ok())
    }
}

struct MapEditor {
    map_data: MapData,
    viewer: MapViewer,
    save_file_path: Option<PathBuf>,
    save_task: BackgroundTask<Option<PathBuf>>,
    renaming: Option<egui::Id>,
}

impl MapEditor {
    fn prompt_save(&mut self) {
        let name = self.map_data.name.clone();

        self.save_task.handle = Some(std::thread::spawn(move || {
            rfd::FileDialog::new()
                .add_filter("map", &["map", "map.ron"])
                .set_file_name(format!("{}.map", name))
                .save_file()
        }));
    }

    fn category_header(
        &mut self,
        ui: &mut egui::Ui,
        index: usize,
        category_to_delete: &mut Option<usize>,
    ) {
        let category = &mut self.map_data.categories[index];

        ui.horizontal(|ui| {
            let id = ui.next_auto_id();
            let text_edit = ui.add(
                egui::TextEdit::singleline(&mut category.name)
                    .clip_text(false)
                    .desired_width(0.0)
                    .interactive(self.renaming == Some(id)),
            );

            if text_edit.lost_focus() {
                self.renaming = None
            }

            text_edit.context_menu(|ui| {
                // TODO: add object menu

                if ui.button("Rename").clicked() {
                    self.renaming = Some(text_edit.id);
                    text_edit.request_focus();
                    ui.close();
                }

                if ui.button("Delete").clicked() {
                    *category_to_delete = Some(index);
                    ui.close();
                }
            });
        });
    }
}

#[derive(Default)]
struct MapCreator {
    map_image_path: PathBuf,
    map_data: MapData,
    import_task: BackgroundTask<Option<PathBuf>>,
}

#[derive(Default)]
struct MapSelector {
    import_task: BackgroundTask<Option<PathBuf>>,
}

enum EditorState {
    Selecting(MapSelector),
    Editing(MapEditor),
    Creator(MapCreator),
}

pub struct Editor {
    state: EditorState,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            state: EditorState::Selecting(MapSelector::default()),
        }
    }
}

impl Screen for Editor {
    fn ui(&mut self, ui: &mut egui::Ui) -> Transition {
        let mut next_state: Option<EditorState> = None;

        match &mut self.state {
            EditorState::Creator(creator) => {
                egui::Area::new("map creator".into())
                    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label("Map name");
                            ui.text_edit_singleline(&mut creator.map_data.name);

                            ui.add_space(50.0);

                            ui.label("Map image");
                            if ui.button("Select").clicked() && creator.import_task.handle.is_none()
                            {
                                creator.import_task.handle = Some(std::thread::spawn(|| {
                                    rfd::FileDialog::new()
                                        .add_filter("image", &["png"])
                                        .pick_file()
                                }));
                            }

                            if creator.map_image_path.is_file() {
                                ui.label(format!("Selected: {}", creator.map_image_path.display()));
                            }

                            ui.add_space(50.0);

                            if ui.button("Create").clicked() {
                                if !creator.map_data.name.is_empty()
                                    && !creator.map_data.image.is_empty()
                                {
                                    let map_texture = Texture2D::from_file_with_format(
                                        &creator.map_data.image,
                                        None,
                                    );

                                    next_state = Some(EditorState::Editing(MapEditor {
                                        map_data: creator.map_data.clone().into(),
                                        viewer: MapViewer::new(map_texture),
                                        save_file_path: None,
                                        save_task: BackgroundTask::default(),
                                        renaming: None,
                                    }))
                                }
                            }
                        });
                    });
            }

            EditorState::Editing(editor) => {
                egui::Panel::top("topbar").show(ui, |ui| {
                    egui::MenuBar::new().ui(ui, |ui| {
                        ui.menu_button("File", |ui| {
                            if ui.button("Save").clicked() {
                                ui.close();
                                match &editor.save_file_path {
                                    Some(path) => map::save(editor.map_data.clone(), path).unwrap(),

                                    None => {
                                        editor.prompt_save();
                                    }
                                }
                            }

                            if ui.button("Save As").clicked() {
                                ui.close();
                                editor.prompt_save();
                            }
                        })
                    });
                    
                    ui.separator();
                    ui.label(&editor.map_data.name)
                });

                egui::Panel::right("explorer")
                    .default_size(120.0)
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .auto_shrink(false)
                            .show(ui, |ui| {
                                let mut category_to_delete = None;

                                for index in 0..editor.map_data.categories.len() {
                                    let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(
                                        ui,
                                        ui.make_persistent_id(("category", index)),
                                        false
                                    );

                                    let header_res = ui.horizontal(|ui| {
                                        let old_spacing = ui.spacing().item_spacing.x;
                                        ui.spacing_mut().item_spacing.x = 0.0;

                                        const INDENT: f32 = 18.0;

                                        if !editor.map_data.categories[index].objects.is_empty() {
                                            let response = ui.allocate_response(egui::vec2(INDENT, ui.spacing().interact_size.y), egui::Sense::click());

                                            if response.clicked() {
                                                state.toggle(ui);
                                            }

                                            let center = response.rect.center();

                                            const ARROW_SIZE: f32 = 4.0;

                                            let points = if state.is_open() {
                                                vec![
                                                    center + egui::vec2(-ARROW_SIZE, -2.0),
                                                    center + egui::vec2(ARROW_SIZE, -2.0),
                                                    center + egui::vec2(0.0, 3.0),
                                                ]
                                            } else {
                                                vec![
                                                    center + egui::vec2(-2.0, -ARROW_SIZE),
                                                    center + egui::vec2(3.0, 0.0),
                                                    center + egui::vec2(-2.0, ARROW_SIZE),
                                                ]
                                            };

                                            ui.painter().add(egui::Shape::convex_polygon(
                                                points,
                                                egui::Color32::GRAY,
                                                egui::Stroke::NONE,
                                            ));
                                        } else {
                                            ui.add_space(INDENT);
                                        }

                                        ui.spacing_mut().item_spacing.x = old_spacing;
                                        editor.category_header(ui, index, &mut category_to_delete);
                                    });

                                    state.show_body_indented(&header_res.response, ui, |ui| {
                                        for region in &editor.map_data.categories[index].objects {
                                            ui.horizontal(|ui| {
                                                ui.add_space(8.0);
                                                if ui.selectable_label(false, region.name()).clicked() {
                                                    todo!()
                                                }
                                            });
                                        }
                                    });
                                }

                                if let Some(index) = category_to_delete {
                                    editor.map_data.categories.remove(index);
                                }

                                ui.interact(
                                    ui.available_rect_before_wrap(),
                                    ui.next_auto_id(),
                                    egui::Sense::click(),
                                )
                                .context_menu(|ui| {
                                    ui.label("New...");
                                    if ui.button("Category").clicked() {
                                        editor.map_data.categories.push(Category {
                                            name: format!("Category {}", editor.map_data.categories.len() + 1),
                                            ..Default::default()
                                        });

                                        ui.close();
                                    };
                                });
                            });
                    });

                egui::CentralPanel::default()
                    .frame(egui::Frame::new())
                    .show(ui, |ui| {
                        let rect = ui.max_rect();

                        editor.viewer.set_viewport(Rect::new(
                            rect.left(),
                            rect.top(),
                            rect.width(),
                            rect.height(),
                        ));
                    });
            }

            EditorState::Selecting(selector) => {
                egui::Area::new("map_selection".into())
                    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                    .show(ui, |ui| {
                        if ui
                            .add_sized(
                                [200.0, 100.0],
                                egui::Button::new(egui::RichText::new("Import").size(28.0))
                                    .corner_radius(12.0),
                            )
                            .clicked()
                            && selector.import_task.handle.is_none()
                        {
                            selector.import_task.handle = Some(std::thread::spawn(|| {
                                rfd::FileDialog::new()
                                    .add_filter("map", &["map", "map.ron"])
                                    .pick_file()
                            }));
                        };

                        ui.add_space(100.0);

                        if ui
                            .add_sized(
                                [200.0, 100.0],
                                egui::Button::new(egui::RichText::new("New").size(28.0))
                                    .corner_radius(12.0),
                            )
                            .clicked()
                        {
                            next_state = Some(EditorState::Creator(MapCreator::default()))
                        }
                    });
            }
        };

        if let Some(state) = next_state {
            self.state = state
        }

        Transition::None
    }

    fn update(&mut self) {
        match &mut self.state {
            EditorState::Selecting(selector) => {
                if let Some(Some(map_file_path)) = selector.import_task.take() {
                    if let Some(map) = map::load(&map_file_path) {
                        let map_texture = Texture2D::from_file_with_format(&map.image, None);

                        self.state = EditorState::Editing(MapEditor {
                            map_data: map.into(),
                            viewer: MapViewer::new(map_texture),
                            save_file_path: Some(map_file_path),
                            save_task: BackgroundTask::default(),
                            renaming: None,
                        })
                    }
                }
            }

            EditorState::Creator(creator) => {
                if let Some(Some(path)) = creator.import_task.take() {
                    if let Some(bytes) = fs::read(&path).ok() {
                        creator.map_image_path = path;
                        creator.map_data.image = bytes
                    }
                }
            }

            EditorState::Editing(editor) => {
                editor.viewer.update();

                if let Some(Some(path)) = editor.save_task.take() {
                    map::save(editor.map_data.clone(), &path).unwrap();
                    editor.save_file_path = Some(path)
                }
            }
        }
    }

    fn render(&self) {
        match &self.state {
            EditorState::Editing(editor) => {
                editor.viewer.render();
            }

            _ => {}
        }
    }
}
