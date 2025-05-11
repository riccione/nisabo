use eframe::egui;

pub fn mod_show_about(ctx: &egui::Context, show: &mut bool) {
    egui::Window::new("About")
        .collapsible(false)
        .resizable(false)
        .open(show)
        .show(ctx, |ui| {
            ui.label("nisabo");
            ui.label("Version: 0.1.0");
            ui.hyperlink("https://");
        });
}
