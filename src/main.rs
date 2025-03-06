


use std::path::{Path, PathBuf};

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
    current: Option<PathBuf>,
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
            current: None,
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
                            self.current = Some(doc.path.clone());
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
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_width(ui.available_width());
                if let Some(doc) = self.current.as_ref()
                    .and_then(|p| self.corpus.iter_mut().find(|d| &d.path == p))
                {
                    doc.update(ui);
                } else {
                    // TODO: Maybe some sort of home screen?
                    ui.centered_and_justified(|ui| {
                        ui.weak("Nothing here...");
                    });
                }
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
            state: None,
        }
    }
}

impl Document {
    pub fn update(&mut self, ui: &mut egui::Ui) {
        self.state = match self.state.take() {
            None => {
                let (send, recv) = std::sync::mpsc::channel();

                let path = self.path.clone();
                std::thread::spawn(move || {
                    let result = Buffer::load(&path);
                    let _ = send.send(result);
                });

                Some(DocumentState::Loading { recv })
            }
            Some(DocumentState::Loading { recv }) => {
                Some(if let Ok(result) = recv.try_recv() {
                    match result {
                        Ok(content) => DocumentState::Loaded { buffer: content },
                        Err(error) => DocumentState::Error { message: error.to_string() },
                    }
                } else {
                    ui.centered_and_justified(|ui| ui.spinner());
                    DocumentState::Loading { recv }
                })
            }
            Some(DocumentState::Loaded { mut buffer }) => {
                buffer.update(ui);
                Some(DocumentState::Loaded { buffer })
            }
            Some(DocumentState::Error { message }) => {
                let mut next_state = Some(DocumentState::Error { message: message.clone() });
                ui.vertical_centered(|ui| {
                    ui.heading("ERROR");
                    ui.label(&message);
                    if ui.button("Retry").clicked() {
                        next_state = None;
                    }
                });
                next_state
            }
        };
    }
}

pub enum DocumentState {
    Loading {
        recv: std::sync::mpsc::Receiver<Result<Buffer>>,
    },
    Loaded {
        buffer: Buffer,
    },
    Error {
        message: String,
    },
}



pub struct Buffer {
    content: String,
}

impl Buffer {
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| {
                anyhow::anyhow!("Could not read contents for document \"{}\": {e}", path.display())
            })?;

        Ok(Self {
            content,
        })
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        let font_id = egui::FontId::monospace(17.0);
        let row_height = ui.fonts(|f| f.row_height(&font_id));
        let desired_rows = (ui.available_height() / row_height).ceil() as usize;

        ui.add(egui::TextEdit::multiline(&mut self.content)
            .font(font_id)
            .desired_width(ui.available_width())
            .desired_rows(desired_rows)
            .code_editor());
    }
}
