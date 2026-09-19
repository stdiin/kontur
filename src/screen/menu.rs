use super::Screen;
use crate::screen::{ScreenType, Transition};
use eframe::egui::{self, Button, Color32, FontId, Response, RichText, Stroke, Ui, Vec2};

struct Theme {
    bg: Color32,
    ink: Color32,
    muted: Color32,
    line: Color32,
    accent: Color32,
    accent_hover: Color32,
    accent_press: Color32,
    paper: Color32,
}

impl Theme {
    const fn atlas() -> Self {
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

pub struct Menu {
    theme: Theme,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            theme: Theme::atlas(),
        }
    }
}

impl Screen for Menu {
    fn ui(&mut self, ui: &mut Ui) -> Transition {
        let theme = &self.theme;
        let mut transition = Transition::None;

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(theme.bg))
            .show(ui, |ui| {
                egui::Area::new("menu".into())
                    .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                    .show(ui.ctx(), |ui| {
                        ui.set_min_width(280.0);
                        ui.set_max_width(360.0);

                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new("Kontur")
                                    .font(FontId::proportional(48.0))
                                    .color(theme.ink),
                            );

                            ui.add_space(6.0);
                            ui.label(RichText::new("GeoKont 2.0").size(13.0).color(theme.muted));
                            ui.add_space(18.0);

                            let w = ui.available_width().min(120.0);
                            let y = ui.cursor().min.y;
                            let x = ui.max_rect().center().x;
                            ui.painter().hline(
                                (x - w * 0.5)..=(x + w * 0.5),
                                y,
                                Stroke::new(1.0, theme.line),
                            );

                            ui.add_space(28.0);

                            if self.text_button(ui, "Play", true).clicked() {
                                transition = Transition::Switch(ScreenType::Quiz);
                            }

                            ui.add_space(8.0);

                            #[cfg(not(target_os = "android"))]
                            {
                                if self.text_button(ui, "Editor", false).clicked() {
                                    transition = Transition::Switch(ScreenType::Editor);
                                }

                                ui.add_space(8.0);
                            }

                            if self.text_button(ui, "Quit", false).clicked() {
                                transition = Transition::Quit;
                            }
                        });
                    });
            });

        transition
    }
}

impl Menu {
    fn text_button(&self, ui: &mut Ui, label: &str, primary: bool) -> Response {
        let theme = &self.theme;
        let width = ui.available_width().min(320.0);
        let height = 44.0;

        ui.scope(|ui| {
            let v = &mut ui.style_mut().visuals.widgets;

            if primary {
                v.inactive.weak_bg_fill = theme.accent;
                v.inactive.bg_fill = theme.accent;
                v.inactive.fg_stroke = Stroke::new(1.0, theme.paper);
                v.inactive.bg_stroke = Stroke::NONE;

                v.hovered.weak_bg_fill = theme.accent_hover;
                v.hovered.bg_fill = theme.accent_hover;
                v.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);
                v.hovered.bg_stroke = Stroke::NONE;
                v.hovered.expansion = 0.0;

                v.active.weak_bg_fill = theme.accent_press;
                v.active.bg_fill = theme.accent_press;
                v.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
                v.active.bg_stroke = Stroke::NONE;
            } else {
                v.inactive.weak_bg_fill = Color32::TRANSPARENT;
                v.inactive.bg_fill = Color32::TRANSPARENT;
                v.inactive.fg_stroke = Stroke::new(1.0, theme.ink);
                v.inactive.bg_stroke = Stroke::new(1.0, theme.line);

                v.hovered.weak_bg_fill = Color32::from_rgba_unmultiplied(46, 92, 122, 18);
                v.hovered.bg_fill = Color32::from_rgba_unmultiplied(46, 92, 122, 18);
                v.hovered.fg_stroke = Stroke::new(1.0, theme.accent);
                v.hovered.bg_stroke = Stroke::new(1.0, theme.accent);
                v.hovered.expansion = 0.0;

                v.active.weak_bg_fill = Color32::from_rgba_unmultiplied(46, 92, 122, 32);
                v.active.bg_fill = Color32::from_rgba_unmultiplied(46, 92, 122, 32);
                v.active.fg_stroke = Stroke::new(1.0, theme.accent_press);
                v.active.bg_stroke = Stroke::new(1.0, theme.accent_press);
            }

            ui.add_sized(
                [width, height],
                Button::new(RichText::new(label).size(16.0))
                    .corner_radius(2.0)
            )
        })
        .inner
    }
}
