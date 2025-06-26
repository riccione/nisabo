mod app;
mod app_eframe;
mod config;
mod ui;
mod db;
mod metadata;
mod markdown;
mod export;
mod import;
mod constants;
mod utils;
mod font;
use app::App;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        metadata::APP_NAME,
        native_options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(App::new(cc)))
        }),
    )
}
