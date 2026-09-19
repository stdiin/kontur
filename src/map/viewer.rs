use eframe::egui::{self, Color32, Context, Image, Rect, Sense, TextureHandle, TextureOptions, Vec2, include_image, pos2, vec2};

pub struct MapViewer {
    offset: Vec2,
    zoom: f32,
    texture: TextureHandle
}

impl MapViewer {
    const MIN_ZOOM: f32 = 1.0;
    const MAX_ZOOM: f32 = 3.0;

    pub fn new(ctx: &Context) -> Self {
        let image_bytes = include_bytes!("../../assets/map.png");
        let image = image::load_from_memory(image_bytes).unwrap();
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        let texture = ctx.load_texture("test", egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()), TextureOptions::default());

        Self {
            offset: Vec2::ZERO,
            zoom: 1.0,
            texture
        }
    }

    fn uv(&self, rect: Rect) -> Rect {
        let image_size = rect.size();
        println!("{image_size}");

        let visible_area = vec2(rect.width() / self.zoom, rect.height() / self.zoom);
        let offset = self.offset;

        Rect::from_min_max(
            (offset / image_size).to_pos2(),
            ((offset + visible_area) / image_size).to_pos2()
        )
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        let rect = ui.available_rect_before_wrap();
        let response = ui.allocate_rect(rect, Sense::click_and_drag());

        ui.input(|i| {
            self.zoom *= i.zoom_delta();
            self.zoom = self.zoom.clamp(Self::MIN_ZOOM, Self::MAX_ZOOM);
        });

        self.offset -= response.drag_delta() / self.zoom;

        let uv = self.uv(rect);

        ui.painter().image(self.texture.id(), rect, uv, Color32::WHITE);
    }
}
