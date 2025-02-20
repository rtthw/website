


mod env;

use eframe::egui;
use env::{Environment, Packages};



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
    packages: Packages,
}

impl Website {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let mut visuals = egui::Visuals::dark();
        visuals.button_frame = false;
        visuals.interact_cursor = Some(egui::CursorIcon::PointingHand);
        cc.egui_ctx.set_visuals(visuals);

        Self {
            env: Environment::default(),
            packages: Packages::default(),
        }
    }
}

impl eframe::App for Website {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.env.windows_mut(|wm| {

        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Packages");
            ui.separator();
            for man in self.packages.manifests().clone() {
                if ui.button(man.title()).clicked() {
                    if !self.packages.exec(&mut self.env, man.title()) {
                        println!("ERROR: Failed to execute package \"{}\"", man.title());
                    }
                }
            }
        });
    }
}
