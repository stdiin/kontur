use egui_macroquad::macroquad::prelude::*;

pub struct MapViewer {
    camera: Camera2D,
    viewport: Viewport,
    zoom: Zoom,
    input: Input,
    map_texture: Texture2D,
    navigation_enabled: bool,
    events: Vec<Event>,
}

#[derive(Clone)]
pub enum Event {
    Click(Vec2),
    Drag(Vec2),
    Zoom(f32),
}

#[derive(Default)]
struct Input {
    last_mouse_pos: Vec2,
    last_press_pos: Vec2,
    last_pinch_dist: f32,
}

#[derive(Default)]
struct Zoom {
    current: f32,
    target: f32,
    default: f32,
}

#[derive(Default)]
struct Viewport {
    last_size: Vec2,
    rect: Rect,
}

impl Viewport {
    fn to_bottom_left_origin(&self) -> Rect {
        let height = screen_height();

        Rect::new(
            self.rect.x,
            height - self.rect.bottom(),
            self.rect.w,
            self.rect.h,
        )
    }
}

impl MapViewer {
    const MAX_ZOOM: f32 = 0.01;
    const LERP_SMOOTHNESS: f32 = 0.05;

    pub fn new(map_texture: Texture2D) -> Self {
        Self {
            map_texture,
            camera: Camera2D::default(),
            zoom: Zoom::default(),
            input: Input::default(),
            viewport: Viewport::default(),
            navigation_enabled: true,
            events: Vec::new(),
        }
    }

    pub fn update(&mut self) -> &[Event] {
        let viewport_size = self.viewport.rect.size();
        if viewport_size != self.viewport.last_size {
            self.fit_map_to_viewport();
            self.viewport.last_size = viewport_size
        }

        self.events.clear();

        if self.viewport.rect.contains(mouse_position().into()) {
            self.process_input();
        }

        self.process_zoom();

        &self.events
    }

    pub fn render(&self) {
        set_camera(&self.camera);
        draw_texture(&self.map_texture, 0.0, 0.0, WHITE);
    }

    pub fn set_navigation_enabled(&mut self, enabled: bool) {
        self.navigation_enabled = enabled
    }

    pub fn set_viewport(&mut self, viewport: Rect) {
        self.viewport.rect = viewport;

        let viewport = self.viewport.to_bottom_left_origin();

        self.camera.viewport = Some((
            viewport.x as i32,
            viewport.y as i32,
            viewport.w as i32,
            viewport.h as i32,
        ));
    }

    fn fit_map_to_viewport(&mut self) {
        self.zoom.default = self.get_min_zoom();
        self.reset();
    }

    fn get_min_zoom(&self) -> f32 {
        f32::min(
            2.0 / self.map_texture.height(),
            2.0 * (self.viewport.rect.w / self.viewport.rect.h) / self.map_texture.width(),
        )
    }

    fn center(&mut self) {
        self.camera.target = vec2(
            self.map_texture.width() / 2.0,
            self.map_texture.height() / 2.0,
        )
    }

    fn reset(&mut self) {
        self.zoom.current = self.zoom.default;
        self.zoom.target = self.zoom.default;
        self.center();
    }

    fn contains(&self, pos: Vec2) -> bool {
        let map_size = self.map_texture.size();
        pos.x >= 0.0 && pos.x <= map_size.x && pos.y >= 0.0 && pos.y <= map_size.y
    }

    fn pinch_center() -> Option<Vec2> {
        let touch_input = touches();
        (touch_input.len() == 2).then(|| (touch_input[0].position + touch_input[1].position) / 2.0)
    }

    fn process_interactions(&mut self) {
        let pointer_pos = Self::pinch_center().unwrap_or(mouse_position().into());
        let world_pointer_pos = self.camera.screen_to_world(pointer_pos);

        if is_mouse_button_pressed(MouseButton::Left) {
            self.input.last_mouse_pos = pointer_pos;
            self.input.last_press_pos = pointer_pos;
        }

        if is_mouse_button_released(MouseButton::Left)
            && self.input.last_press_pos.distance(pointer_pos) <= 3.0
            && self.contains(world_pointer_pos)
        {
            self.events.push(Event::Click(world_pointer_pos));
        }

        if is_mouse_button_down(MouseButton::Left) && mouse_delta_position() != Vec2::ZERO {
            let last_world_mouse_pos = self.camera.screen_to_world(self.input.last_mouse_pos);
            self.input.last_mouse_pos = pointer_pos;
            self.events
                .push(Event::Drag(last_world_mouse_pos - world_pointer_pos));
        }

        let (_, scroll) = mouse_wheel();

        if scroll != 0.0 {
            self.events.push(Event::Zoom(1.1_f32.powf(scroll)));
        }

        let touches = touches();

        if touches.len() == 2 {
            let pos1 = touches[0].position;
            let pos2 = touches[1].position;
            let dist = pos1.distance(pos2);
            let last_dist = self.input.last_pinch_dist;

            if last_dist != 0.0 {
                self.events.push(Event::Zoom(dist / last_dist));
            }

            self.input.last_pinch_dist = dist;
        } else {
            self.input.last_pinch_dist = 0.0;
        }
    }

    fn process_input(&mut self) {
        self.process_interactions();

        for event in &self.events {
            match event {
                Event::Drag(delta) => self.camera.target += delta.clone(),
                Event::Zoom(factor) => {
                    self.zoom.target =
                        clamp(self.zoom.target * factor, self.zoom.default, Self::MAX_ZOOM)
                }
                _ => {}
            }
        }
    }

    fn process_zoom(&mut self) {
        let camera = &mut self.camera;
        let aspect = self.viewport.rect.w / self.viewport.rect.h;

        let zoom_center = Self::pinch_center().unwrap_or(mouse_position().into());

        let before = camera.screen_to_world(zoom_center);

        self.zoom.current = self
            .zoom
            .current
            .lerp(self.zoom.target, Self::LERP_SMOOTHNESS);

        camera.zoom = vec2(self.zoom.current / aspect, self.zoom.current);

        let after = camera.screen_to_world(zoom_center);

        if self.zoom.target != self.zoom.current {
            camera.target += before - after;
        }
    }
}
