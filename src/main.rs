mod app;
mod app_eframe;
mod config;
mod ui;
mod db;
mod metadata;
mod markdown;
use app::App;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        metadata::APP_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc))))
    )
}
