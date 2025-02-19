


use eframe::egui;



fn main() {
    eframe::run_native(
        "Matthew Norman",
        eframe::NativeOptions {
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(Website::new(cc)))),
    ).unwrap();
}



struct Website;

impl Website {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let mut visuals = egui::Visuals::dark();
        visuals.button_frame = false;
        visuals.interact_cursor = Some(egui::CursorIcon::PointingHand);
        cc.egui_ctx.set_visuals(visuals);

        Self {}
    }
}

impl eframe::App for Website {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("TODO");
            });
        });
    }
}
