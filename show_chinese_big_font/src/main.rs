#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use egui::IconData; // hide console window on Windows in release
pub struct UiConfig {
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub icon: IconData,
    pub resizable: bool,
    pub centered: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            title: "你好世界".to_string(),
            width: 480.0,
            height: 320.0,
            icon: eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                .expect("Failed to load icon"),
            resizable: false,
            centered: true,
        }
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let ui_cfg = UiConfig::default();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            // .with_min_inner_size([ui_cfg.height, ui_cfg.width])
            .with_inner_size([ui_cfg.height, ui_cfg.width])
            // .with_max_inner_size([ui_cfg.height, ui_cfg.width])
            .with_maximize_button(false)
            .with_resizable(ui_cfg.resizable)
            .with_icon(ui_cfg.icon),
        centered: ui_cfg.centered,
        ..Default::default()
    };
    eframe::run_native(
        "你好世界",
        native_options,
        Box::new(|cc| Box::new(eframe_template::TemplateApp::new(cc))),
    )
}
