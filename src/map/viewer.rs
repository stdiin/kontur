use eframe::egui::{self, Context, DragPanButtons, Pos2, Rect, Scene, TextureHandle, TextureOptions, Vec2, include_image, pos2};

pub struct MapViewer {
    rect: Rect,
    texture: TextureHandle
}

impl MapViewer {
    pub fn new(ctx: &Context) -> Self {
        let image_bytes = include_bytes!("../../assets/map.png");
        let image = image::load_from_memory(image_bytes).unwrap();
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        let texture = ctx.load_texture("test", egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()), TextureOptions::default());

        Self {
            rect: Rect::from_min_size(Pos2::ZERO, texture.size_vec2()),
            texture
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        Scene::new()
            .drag_pan_buttons(DragPanButtons::PRIMARY)
            .zoom_range((ui.available_size() / self.texture.size_vec2()).min_elem()..=3.0)
            .max_inner_size(self.texture.size_vec2())
            .show(ui, &mut self.rect, |ui| {
                ui.image(include_image!("../../assets/map.png"));
            });
    }
}
