


use std::path::PathBuf;

use anyhow::Result;
use eframe::egui;



fn main() -> Result<()> {
    let corpus_path = std::env::current_dir()?.join("test");
    let mut corpus = vec![];
    for e in std::fs::read_dir(&corpus_path)? {
        let entry = e?;
        corpus.push(Document::new_unloaded(entry.path()));
    }

    eframe::run_native(
        "My Website Manager",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder {
                inner_size: Some([1200.0, 800.0].into()),
                ..Default::default()
            },
            ..Default::default()
        },
        Box::new(move |cc| Ok(Box::new(Program::new(cc, corpus)))),
    ).unwrap();

    Ok(())
}



struct Program {
    corpus: Vec<Document>,
}

impl Program {
    fn new(cc: &eframe::CreationContext, corpus: Vec<Document>) -> Self {
        let mut visuals = egui::Visuals::dark();
        visuals.button_frame = false;
        visuals.interact_cursor = Some(egui::CursorIcon::PointingHand);
        visuals.override_text_color = Some(egui::Color32::from_rgb(0xb7, 0xb7, 0xc0));
        visuals.extreme_bg_color = egui::Color32::from_rgb(0x1e, 0x1e, 0x22);
        visuals.window_fill = egui::Color32::from_rgb(0x1e, 0x1e, 0x22);
        visuals.panel_fill = egui::Color32::from_rgb(0x2b, 0x2b, 0x33);
        cc.egui_ctx.set_visuals(visuals);

        cc.egui_ctx.style_mut(|s| {
            use egui::{FontFamily, FontId, TextStyle::*};
            s.text_styles = [
                (Heading, FontId::new(29.0, FontFamily::Proportional)),
                (Body, FontId::new(19.0, FontFamily::Proportional)),
                (Monospace, FontId::new(19.0, FontFamily::Monospace)),
                (Button, FontId::new(19.0, FontFamily::Proportional)),
                (Small, FontId::new(13.0, FontFamily::Proportional)),
            ].into();
        });

        Self {
            corpus,
        }
    }
}

impl eframe::App for Program {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("corpus-list").show(ctx, |ui| {
            egui::ScrollArea::vertical().show_rows(ui, 31.0, self.corpus.len(), |ui, row_range| {
                ui.set_width(ui.available_width());
                for doc in &mut self.corpus[row_range] {
                    ui.horizontal_centered(|ui| {
                        if ui.link(&doc.title).clicked() {
                            println!("TODO: Open documents");
                        }
                        ui.add(egui::Label::new(egui::RichText::new(
                            doc.path.file_name().unwrap().to_string_lossy())
                                .weak()
                                .small())
                            .truncate());
                    });
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("TODO");
            });
        });
    }
}



pub struct Document {
    path: PathBuf,
    title: String,
    state: Option<DocumentState>,
}

impl Document {
    pub fn new_unloaded(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let title = path.file_name().unwrap().to_string_lossy().to_string();

        Self {
            path,
            title,
            state: Some(DocumentState::Unloaded),
        }
    }
}

pub enum DocumentState {
    Unloaded,
}
