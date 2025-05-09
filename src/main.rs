use eframe::egui;

const APP_NAME: &str = env!("CARGO_PKG_NAME");

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc))))
    )
}

#[derive(Default)]
struct App {}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // customize egui with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(APP_NAME);
        });
    }
}
