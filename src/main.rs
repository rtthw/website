


use eframe::egui;



fn main() {
    eframe::run_native(
        "Matthew Norman",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder {
                inner_size: Some([1200.0, 800.0].into()),
                ..Default::default()
            },
            ..Default::default()
        },
        Box::new(move |cc| Ok(Box::new(Program::new(cc)))),
    ).unwrap();
}



struct Program {}

impl Program {
    fn new(cc: &eframe::CreationContext) -> Self {
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

        Self {}
    }
}

impl eframe::App for Program {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("TODO");
            });
        });
    }
}
