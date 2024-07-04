use crate::config::fonts;
use egui::{FontId, Label, RichText, TextEdit};

use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

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
        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     egui::menu::bar(ui, |ui| {
        //         ui.menu_button("File", |ui| {
        //             if ui.button("Open").clicked() {
        //                 let res = rfd::FileDialog::new()
        //                     .add_filter("Text Files", &["txt"])
        //                     .add_filter("All Files", &["*"])
        //                     .pick_file();
        //                 if let Some(path) = res {
        //                     println!("{path:?}");
        //                     self.label = path.to_str().unwrap().to_string();
        //                 }
        //             }
        //         });
        //         ui.add_space(16.0);

        //         if ui.button("new").clicked() {
        //             let res = rfd::MessageDialog::new()
        //                 .set_title("Msg!")
        //                 .set_description("Description!")
        //                 .set_level(rfd::MessageLevel::Info)
        //                 .set_buttons(rfd::MessageButtons::OkCancelCustom(
        //                     "Got it!".to_string(),
        //                     "No!".to_string(),
        //                 ))
        //                 .show();
        //             println!("{res}");
        //         }
        //     });
        // });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label(RichText::new("大文本").font(FontId::proportional(40.0)));
            });
            // ui.separator();

            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .max_height(360.0)
                .show(ui, |ui| {
                    let label = TextEdit::multiline(&mut self.label)
                        .text_color(egui::Color32::from_rgb(51, 255, 51))
                        .interactive(false);
                    ui.add(label);
                });
        });
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                if ui.button("call cmd").clicked() {
                    call_cmd(&mut self.label).unwrap();
                }
            });
        });
    }
}

fn call_cmd(label: &mut String) -> Result<(), Error> {
    let stdout = Command::new("git")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| *label = format!("{}\n{}", label, line));
    Ok(())
}
