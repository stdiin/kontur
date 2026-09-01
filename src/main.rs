use egui_macroquad::macroquad::{self, prelude::*};

mod screens;
mod viewer;

use crate::screens::{ScreenType, Transition};

#[macroquad::main("Kontur")]
async fn main() {
    let mut screen = ScreenType::Menu.build();

    loop {
        let transition = {
            let mut result = Transition::None;
            egui_macroquad::ui(|ctx| result = screen.ui(ctx));
            result
        };

        match transition {
            Transition::None => {}
            Transition::Quit => break,
            Transition::Switch(next_screen) => screen = next_screen.build(),
        }

        screen.update();
        screen.render();
        egui_macroquad::draw();

        next_frame().await
    }
}
