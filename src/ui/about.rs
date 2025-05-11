use eframe::egui;
use crate::metadata::{APP_NAME, APP_VERSION, APP_HOMEPAGE};

pub fn show_about(ctx: &egui::Context, show: &mut bool) {
    egui::Window::new("About")
        .collapsible(false)
        .resizable(false)
        .open(show)
        .show(ctx, |ui| {
            ui.label(APP_NAME);
            ui.label(format!("Version: {APP_VERSION}"));
            ui.hyperlink(APP_HOMEPAGE);
        });
}
