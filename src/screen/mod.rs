#[cfg(not(target_os = "android"))]
pub mod editor;

pub mod menu;
pub mod quiz;

pub enum Transition {
    None,
    Quit,
    Switch(ScreenType),
}

pub trait Screen {
    fn update(&mut self) {}
    fn render(&self) {}
    fn ui(&mut self, _ui: &mut egui::Ui) -> Transition {
        Transition::None
    }
}

pub enum ScreenType {
    Quiz,

    #[cfg(not(target_os = "android"))]
    Editor,

    Menu,
}

impl ScreenType {
    pub fn build(&self) -> Box<dyn Screen> {
        match self {
            #[cfg(not(target_os = "android"))]
            Self::Editor => Box::new(editor::Editor::default()),

            Self::Quiz => Box::new(quiz::Quiz::default()),
            Self::Menu => Box::new(menu::Menu::default()),
        }
    }
}
