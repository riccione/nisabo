use eframe::egui;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

pub fn show_about(ctx: &egui::Context, show: &mut bool) {
    egui::Window::new("About")
        .collapsible(false)
        .resizable(false)
        .open(show)
        .show(ctx, |ui| {
            ui.label("nisabo");
            ui.label(format!("Version: {APP_VERSION}"));
            ui.hyperlink(APP_HOMEPAGE);
        });
}
