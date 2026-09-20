use eframe::egui::{
    self, Context, DragPanButtons, Pos2, Rect, Scene, TextureHandle, TextureOptions,
};

pub struct MapViewer {
    rect: Rect,
    texture: TextureHandle,
}

impl MapViewer {
    pub fn new(ctx: &Context, image_bytes: impl AsRef<[u8]>) -> Self {
        let image = image::load_from_memory(image_bytes.as_ref()).unwrap();
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        let texture = ctx.load_texture(
            "test",
            egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
            TextureOptions::default(),
        );

        Self {
            rect: Rect::from_min_size(Pos2::ZERO, texture.size_vec2()),
            texture,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        Scene::new()
            .drag_pan_buttons(DragPanButtons::PRIMARY)
            .zoom_range((ui.available_size() / self.texture.size_vec2()).min_elem()..=3.0)
            .show(ui, &mut self.rect, |ui| {
                ui.image((self.texture.id(), self.texture.size_vec2()));
            });
    }
}
