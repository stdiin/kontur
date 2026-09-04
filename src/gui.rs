use egui_miniquad::EguiMq;
use macroquad::miniquad;
use macroquad::prelude::*;

pub struct Gui {
    egui_mq: EguiMq,
    input_subscriber_id: usize,
}

impl Gui {
    pub fn new() -> Self {
        let gl = unsafe { get_internal_gl() };

        Self {
            egui_mq: EguiMq::new(gl.quad_context),
            input_subscriber_id: macroquad::input::utils::register_input_subscriber(),
        }
    }

    pub fn run(&mut self, mut f: impl FnMut(&mut egui::Ui)) {
        macroquad::input::utils::repeat_all_miniquad_input(self, self.input_subscriber_id);

        let gl = unsafe { get_internal_gl() };

        self.egui_mq.run(gl.quad_context, |_, ui| {
            f(ui);
        });
    }

    pub fn draw(&mut self) {
        let mut gl = unsafe { get_internal_gl() };

        gl.flush();
        self.egui_mq.draw(gl.quad_context);
    }
}

impl miniquad::EventHandler for Gui {
    fn update(&mut self) {}
    fn draw(&mut self) {}

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(&mut self, button: miniquad::MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_down_event(button, x, y);
    }

    fn mouse_button_up_event(&mut self, button: miniquad::MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_up_event(button, x, y);
    }

    fn char_event(&mut self, character: char, _modifiers: miniquad::KeyMods, _repeat: bool) {
        self.egui_mq.char_event(character);
    }

    fn key_down_event(
        &mut self,
        keycode: miniquad::KeyCode,
        modifiers: miniquad::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.key_down_event(keycode, modifiers);
    }

    fn key_up_event(&mut self, keycode: miniquad::KeyCode, modifiers: miniquad::KeyMods) {
        self.egui_mq.key_up_event(keycode, modifiers);
    }
}
