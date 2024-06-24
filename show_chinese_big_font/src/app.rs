use egui::{FontId, RichText};
use crate::config::fonts;
pub struct TemplateApp {
    label: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "你好世界!".to_owned(),
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        fonts::setup_custom_fonts(&cc.egui_ctx);
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        let res = rfd::FileDialog::new()
                            .add_filter("Text Files", &["txt"])
                            .add_filter("All Files", &["*"])
                            .pick_file();
                        if let Some(path) = res {
                            println!("{path:?}");
                            self.label = path.to_str().unwrap().to_string();
                        }
                    }
                });
                ui.add_space(16.0);

                if ui.button("new").clicked() {
                    let res = rfd::MessageDialog::new()
                        .set_title("Msg!")
                        .set_description("Description!")
                        .set_level(rfd::MessageLevel::Info)
                        .set_buttons(rfd::MessageButtons::OkCancelCustom(
                            "Got it!".to_string(),
                            "No!".to_string(),
                        ))
                        .show();
                    println!("{res}");
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label(RichText::new("大文本").font(FontId::proportional(40.0)));
                ui.text_edit_multiline(&mut self.label);
            });
            // ui.separator();
        });
    }
}
