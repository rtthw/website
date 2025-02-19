


mod env;

use eframe::egui;
use env::Environment;



fn main() {
    eframe::run_native(
        "Matthew Norman",
        eframe::NativeOptions {
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(Website::new(cc)))),
    ).unwrap();
}



struct Website {
    env: Environment,
}

impl Website {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let mut visuals = egui::Visuals::dark();
        visuals.button_frame = false;
        visuals.interact_cursor = Some(egui::CursorIcon::PointingHand);
        cc.egui_ctx.set_visuals(visuals);

        Self {
            env: Environment::default(),
        }
    }
}

impl eframe::App for Website {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Packages");
            ui.separator();
            for man in self.env.packages(|p| p.manifests().clone()) {
                if ui.button(man.title()).clicked() {
                    if !self.env.packages_mut(|p| p.exec(man.title())) {
                        println!("ERROR: Failed to execute package \"{}\"", man.title());
                    }
                }
            }
        });
    }
}
