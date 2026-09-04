use macroquad::prelude::*;

mod screen;
mod map;
mod gui;

use crate::screen::{ScreenType, Transition};

#[macroquad::main("Kontur")]
async fn main() {
    let mut screen = ScreenType::Menu.build();
    let mut gui = gui::Gui::new();

    loop {
        let transition = {
            let mut result = Transition::None;
            gui.run(|ui| result = screen.ui(ui));
            result
        };

        match transition {
            Transition::None => {}
            Transition::Quit => break,
            Transition::Switch(next_screen) => screen = next_screen.build(),
        }

        screen.update();
        screen.render();
        gui.draw();

        next_frame().await
    }
}
