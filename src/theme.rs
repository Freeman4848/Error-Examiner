use eframe::egui::{self, Color32, FontId, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) enum ThemeKind {
    Black,
    Forest,
    #[default]
    Midnight,
    Light,
}

impl ThemeKind {
    pub(crate) const ALL: [Self; 4] = [Self::Black, Self::Forest, Self::Midnight, Self::Light];

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Black => "Black",
            Self::Forest => "Green",
            Self::Midnight => "Midnight",
            Self::Light => "Light",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub(crate) struct Appearance {
    pub(crate) theme: ThemeKind,
    #[serde(default = "default_opacity")]
    pub(crate) opacity: f32,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            theme: ThemeKind::default(),
            opacity: default_opacity(),
        }
    }
}

fn default_opacity() -> f32 {
    1.0
}

#[derive(Clone, Copy)]
pub(crate) struct Palette {
    pub(crate) background: Color32,
    pub(crate) panel: Color32,
    pub(crate) card: Color32,
    pub(crate) input: Color32,
    pub(crate) border: Color32,
    pub(crate) text: Color32,
    pub(crate) muted: Color32,
    pub(crate) accent: Color32,
    pub(crate) user: Color32,
    pub(crate) assistant: Color32,
}

pub(crate) fn install_fonts(context: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "noto".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/NotoSans-Regular.ttf")),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "noto".to_owned());
    context.set_fonts(fonts);
}

pub(crate) fn palette(theme: ThemeKind, opacity: f32) -> Palette {
    let mut colors = match theme {
        ThemeKind::Black => Palette {
            background: Color32::from_rgb(7, 7, 8),
            panel: Color32::from_rgb(10, 10, 11),
            card: Color32::from_rgb(15, 15, 17),
            input: Color32::from_rgb(12, 12, 14),
            border: Color32::from_rgb(38, 38, 42),
            text: Color32::from_rgb(244, 244, 245),
            muted: Color32::from_rgb(161, 161, 170),
            accent: Color32::from_rgb(82, 82, 91),
            user: Color32::from_rgb(248, 113, 113),
            assistant: Color32::from_rgb(74, 222, 128),
        },
        ThemeKind::Forest => Palette {
            background: Color32::from_rgb(6, 16, 12),
            panel: Color32::from_rgb(9, 24, 17),
            card: Color32::from_rgb(13, 34, 24),
            input: Color32::from_rgb(8, 27, 18),
            border: Color32::from_rgb(30, 64, 47),
            text: Color32::from_rgb(231, 248, 238),
            muted: Color32::from_rgb(135, 170, 149),
            accent: Color32::from_rgb(28, 112, 75),
            user: Color32::from_rgb(248, 113, 113),
            assistant: Color32::from_rgb(74, 222, 128),
        },
        ThemeKind::Midnight => Palette {
            background: Color32::from_rgb(8, 15, 30),
            panel: Color32::from_rgb(10, 18, 34),
            card: Color32::from_rgb(15, 23, 42),
            input: Color32::from_rgb(9, 17, 32),
            border: Color32::from_rgb(30, 41, 59),
            text: Color32::from_rgb(226, 232, 240),
            muted: Color32::from_rgb(148, 163, 184),
            accent: Color32::from_rgb(37, 99, 235),
            user: Color32::from_rgb(248, 113, 113),
            assistant: Color32::from_rgb(50, 213, 131),
        },
        ThemeKind::Light => Palette {
            background: Color32::from_rgb(247, 248, 250),
            panel: Color32::from_rgb(255, 255, 255),
            card: Color32::from_rgb(255, 255, 255),
            input: Color32::from_rgb(255, 255, 255),
            border: Color32::from_rgb(226, 232, 240),
            text: Color32::from_rgb(17, 24, 39),
            muted: Color32::from_rgb(100, 116, 139),
            accent: Color32::from_rgb(37, 99, 235),
            user: Color32::from_rgb(220, 38, 38),
            assistant: Color32::from_rgb(5, 150, 105),
        },
    };
    let alpha = (opacity.clamp(0.3, 1.0) * 255.0) as u8;
    colors.background = alpha_color(colors.background, alpha);
    colors.panel = alpha_color(colors.panel, alpha);
    colors.card = alpha_color(colors.card, alpha);
    colors.input = alpha_color(colors.input, alpha);
    colors
}

fn alpha_color(color: Color32, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}

pub(crate) fn apply_theme(context: &egui::Context, theme: ThemeKind, opacity: f32) {
    let colors = palette(theme, opacity);
    let mut style = (*context.style()).clone();
    style.spacing.item_spacing = Vec2::new(8.0, 7.0);
    style.spacing.button_padding = Vec2::new(10.0, 5.0);
    style.visuals.dark_mode = theme != ThemeKind::Light;
    style.visuals.override_text_color = Some(colors.text);
    style.visuals.window_fill = colors.background;
    style.visuals.panel_fill = colors.panel;
    style.visuals.extreme_bg_color = colors.input;
    style.visuals.faint_bg_color = colors.card;
    style.visuals.widgets.inactive.bg_fill = colors.card;
    style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, colors.border);
    style.visuals.widgets.hovered.bg_fill = colors.border;
    style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, colors.muted);
    style.visuals.widgets.active.bg_fill = colors.accent;
    style.visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, colors.accent);
    style.visuals.selection.bg_fill = colors.accent;
    style.visuals.window_rounding = egui::Rounding::same(12.0);
    style
        .text_styles
        .insert(egui::TextStyle::Body, FontId::proportional(14.0));
    style
        .text_styles
        .insert(egui::TextStyle::Button, FontId::proportional(13.0));
    style
        .text_styles
        .insert(egui::TextStyle::Heading, FontId::proportional(20.0));
    context.set_style(style);
}

pub(crate) fn load_app_icon() -> std::sync::Arc<egui::IconData> {
    let image = image::load_from_memory(include_bytes!("../assets/app-icon.png"))
        .expect("embedded app icon should decode")
        .resize_exact(256, 256, image::imageops::FilterType::Lanczos3)
        .into_rgba8();
    std::sync::Arc::new(egui::IconData {
        rgba: image.into_raw(),
        width: 256,
        height: 256,
    })
}

pub(crate) fn load_app_texture(context: &egui::Context) -> egui::TextureHandle {
    let image = image::load_from_memory(include_bytes!("../assets/app-icon.png"))
        .expect("embedded app icon should decode")
        .resize_exact(160, 160, image::imageops::FilterType::Lanczos3)
        .into_rgba8();
    context.load_texture(
        "app-icon",
        egui::ColorImage::from_rgba_unmultiplied([160, 160], image.as_raw()),
        egui::TextureOptions::LINEAR,
    )
}
