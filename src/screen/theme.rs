use eframe::egui::Color32;

pub struct Theme {
    pub bg: Color32,
    pub ink: Color32,
    pub muted: Color32,
    pub line: Color32,
    pub accent: Color32,
    pub accent_hover: Color32,
    pub accent_press: Color32,
    pub paper: Color32,
}

impl Theme {
    pub fn atlas() -> Self {
        Self {
            bg: Color32::from_rgb(244, 241, 234),
            ink: Color32::from_rgb(28, 32, 36),
            muted: Color32::from_rgb(110, 116, 122),
            line: Color32::from_rgb(196, 188, 172),
            accent: Color32::from_rgb(46, 92, 122),
            accent_hover: Color32::from_rgb(36, 78, 106),
            accent_press: Color32::from_rgb(28, 64, 88),
            paper: Color32::from_rgb(255, 252, 246),
        }
    }
}
